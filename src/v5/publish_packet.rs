// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};
use bytes::BytesMut;
use std::io::Write;

use super::{FixedHeader, Packet, PacketType};
use crate::{
    consts, topic, utils, ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket,
    PacketId, QoS,
};

/// PublishPacket is used to transport application messages from the Client to the Server,
/// or from the Server to the Client.
///
/// Basic structure of packet:
///
/// ```txt
///  7                     0
/// +-----------------------+
/// | Fixed header          |
/// |                       |
/// +-----------------------+
/// | Topic name length     |
/// |                       |
/// +-----------------------+
/// | Topic name ...        |
/// +-----------------------+
/// | Packet Identifier     |
/// |                       |
/// +-----------------------+
/// | Msg payload ...       |
/// +-----------------------+
/// | Properties ...        |
/// +-----------------------+
/// ```
///
/// Note that `packet_id` only appears in QoS 1 and QoS 2 packets.
///
/// Response of PublischPacket:
/// * QoS 0, no response
/// * QoS 1, PublishAckPacket
/// * QoS 2, PublishRecPacket
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PublishPacket {
    /// If dup field is false, it indicates that this is the first time to send this packet.
    /// If it is true, then this packet might be re-delivery of an earlier attempt to send the
    /// Packet.
    ///
    /// It must be false if QoS is 0.
    dup: bool,

    /// `qos` field indicates the level of assurance for delivery of packet.
    qos: QoS,

    /// Usage of `retain` flag in PublishPacket is complex:
    ///
    /// If `retain` flag is true in the packet the Client sent to the Server,
    /// this packet is stored on the server so that it can be delivered to future
    /// subscribers. When a new subscription is established, the last retained packet
    /// will be sent to the subscriber. If the Server receives a QoS 0 message with
    /// the `retain` flag set to true, it must discard any message previously retained
    /// for the same topic. The Server should store the new QoS 0 message as the new
    /// retained message for that topic, but may choose to discard it at any time.
    ///
    /// When sending a PublishPacket the Server must set the `retain` flag to true if
    /// a message is sent as a result of a new subscription. The Server must set
    /// `retain` flag to false when sending PublishPacket to already connected subscribers.
    ///
    /// If a PublishPacket sent to the Server with `retain` flag on and payload contains
    /// zero bytes, this packet is normally delivered to subscribers. And this packet is
    /// used as notification to the Server to delete any retained messages on the same topic.
    /// And any future subscribers for the same topic will not receive any retained messages
    /// any more. So this means one-time shot.
    ///
    /// If `retain` flag is false in PublishPacket sent to the Server, status of
    /// the retained message of that topic is not removed or replaced.
    retain: bool,

    /// `topic` name must not contain wildcard characters.
    topic: String,

    /// `packet_id` field is useless if QoS is 0.
    packet_id: PacketId,

    /// Payload contains `msg` field.
    /// TODO(Shaohua): Replace with Bytes or Vec<u8>, BytewMut is useless.
    msg: BytesMut,
}

impl PublishPacket {
    // TODO(Shaohua): No need to copy topic and msg
    pub fn new(topic: &str, qos: QoS, msg: &[u8]) -> Result<PublishPacket, EncodeError> {
        utils::validate_utf8_string(topic)?;
        topic::validate_pub_topic(topic)?;

        Ok(PublishPacket {
            qos,
            dup: false,
            retain: false,
            topic: topic.to_string(),
            packet_id: 0,
            msg: BytesMut::from(msg),
        })
    }

    pub fn append(&mut self, msg_parts: &[u8]) {
        self.msg.extend_from_slice(msg_parts);
    }

    pub fn set_retain(&mut self, retain: bool) -> &mut Self {
        self.retain = retain;
        self
    }

    pub fn retain(&self) -> bool {
        self.retain
    }

    pub fn set_dup(&mut self, dup: bool) -> Result<&mut Self, EncodeError> {
        // The DUP flag MUST be set to 0 for all QoS 0 messages [MQTT-3.3.1-2].
        if dup && self.qos == QoS::AtMostOnce {
            return Err(EncodeError::InvalidPacketType);
        }
        self.dup = dup;
        Ok(self)
    }

    pub fn dup(&self) -> bool {
        self.dup
    }

    pub fn set_qos(&mut self, qos: QoS) -> &mut Self {
        if qos == QoS::AtMostOnce {
            self.packet_id = 0;
        }
        self.qos = qos;
        self
    }

    pub fn qos(&self) -> QoS {
        self.qos
    }

    /// The Packet Identifier field is only present in PUBLISH Packets where the QoS level is 1 or 2.
    pub fn set_packet_id(&mut self, packet_id: PacketId) -> &mut Self {
        self.packet_id = packet_id;
        self
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    pub fn set_topic(&mut self, topic: &str) -> Result<&mut Self, EncodeError> {
        utils::validate_utf8_string(topic)?;
        topic::validate_pub_topic(topic)?;
        self.topic = topic.to_string();
        Ok(self)
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn message(&self) -> &[u8] {
        &self.msg
    }

    // TODO(Shaohua): Add message related operations.
}

impl DecodePacket for PublishPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;

        let (dup, qos, retain) =
            if let PacketType::Publish { dup, qos, retain } = fixed_header.packet_type() {
                (dup, qos, retain)
            } else {
                return Err(DecodeError::InvalidPacketType);
            };

        // The DUP flag MUST be set to 0 for all QoS 0 messages [MQTT-3.3.1-2].
        if dup && qos == QoS::AtMostOnce {
            return Err(DecodeError::InvalidPacketFlags);
        }

        // In the QoS 1 delivery protocol, the Sender MUST send a PUBLISH Packet
        // containing this Packet Identifier with QoS=1, DUP=0.
        // [MQTT-4.3.2-1].
        if dup && qos == QoS::AtLeastOnce {
            return Err(DecodeError::InvalidPacketFlags);
        }

        let topic_len = ba.read_u16()? as usize;
        let topic = ba.read_string(topic_len)?;
        // The Topic Name MUST be present as the first field in the PUBLISH Packet Variable header.
        // It MUST be a UTF-8 encoded string [MQTT-3.3.2-1] as defined in section 1.5.3.
        utils::validate_utf8_string(&topic)?;

        // The Topic Name in the PUBLISH Packet MUST NOT contain wildcard characters [MQTT-3.3.2-2].
        topic::validate_pub_topic(&topic)?;

        // Parse packet id.
        //
        // A PUBLISH packet MUST NOT contain a Packet Identifier if its QoS value is
        // set to 0 [MQTT-2.2.1-2].
        let packet_id = if qos != QoS::AtMostOnce {
            let packet_id = ba.read_u16()? as PacketId;
            // Each time a Client sends a new SUBSCRIBE, UNSUBSCRIBE,or PUBLISH (where QoS > 0)
            // MQTT Control Packet it MUST assign it a non-zero Packet Identifier
            // that is currently unused [MQTT-2.2.1-3].
            if packet_id == 0 {
                return Err(DecodeError::InvalidPacketId);
            }
            packet_id
        } else {
            0
        };

        // It is valid for a PUBLISH Packet to contain a zero length payload.
        if fixed_header.remaining_length() < topic_len + consts::TOPIC_LENGTH_BYTES {
            return Err(DecodeError::InvalidRemainingLength);
        }
        let mut msg_len = fixed_header.remaining_length()
            - consts::TOPIC_LENGTH_BYTES // topic length bytes
            - topic_len; // topic
        if qos != QoS::AtMostOnce {
            if msg_len < consts::PACKET_ID_BYTES {
                return Err(DecodeError::InvalidRemainingLength);
            }

            // Packet identifier is presesnt in QoS1/QoS2 packets.
            msg_len -= consts::PACKET_ID_BYTES;
        }

        let msg = BytesMut::from(ba.read_bytes(msg_len)?);
        Ok(PublishPacket {
            qos,
            retain,
            dup,
            topic,
            packet_id,
            msg,
        })
    }
}

impl EncodePacket for PublishPacket {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = v.len();

        let mut remaining_length = consts::TOPIC_LENGTH_BYTES // Topic length bytes
            + self.topic.len() // Topic length
            + self.msg.len(); // Message length
        if self.qos != QoS::AtMostOnce {
            // For `packet_id` field.
            remaining_length += consts::PACKET_ID_BYTES;
        }

        let fixed_header = FixedHeader::new(
            PacketType::Publish {
                dup: self.dup,
                retain: self.retain,
                qos: self.qos,
            },
            remaining_length,
        );
        fixed_header.encode(v)?;

        // Write variable header
        v.write_u16::<BigEndian>(self.topic.len() as u16)?;
        v.write_all(&self.topic.as_bytes())?;

        // The Packet Identifier field is only present in PUBLISH Packets where the QoS level is 1 or 2.
        if self.qos() != QoS::AtMostOnce {
            v.write_u16::<BigEndian>(self.packet_id)?;
        }

        // Write payload
        v.write_all(&self.msg)?;

        Ok(v.len() - old_len)
    }
}

impl Packet for PublishPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::Publish {
            dup: self.dup,
            retain: self.retain,
            qos: self.qos,
        }
    }
}
