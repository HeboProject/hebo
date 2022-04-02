// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use super::{FixedHeader, Packet, PacketType};
use crate::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId, QoS, SubTopic,
};

/// Topic/QoS pair.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubscribeTopic {
    /// Subscribed `topic` contains wildcard characters to match interested topics with patterns.
    topic: SubTopic,

    /// Maximum level of QoS of packet the Server can send to the Client.
    qos: QoS,
}

impl SubscribeTopic {
    pub fn new(topic: &str, qos: QoS) -> Result<Self, EncodeError> {
        let topic = SubTopic::new(topic)?;
        Ok(Self { topic, qos })
    }

    pub fn topic(&self) -> &str {
        self.topic.as_ref()
    }

    pub fn qos(&self) -> QoS {
        self.qos
    }

    pub fn bytes(&self) -> usize {
        1 + self.topic.bytes()
    }
}

impl EncodePacket for SubscribeTopic {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        self.topic.encode(buf)?;
        let qos: u8 = 0b0000_0011 & (self.qos as u8);
        buf.push(qos);

        Ok(self.bytes())
    }
}

impl DecodePacket for SubscribeTopic {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let topic = SubTopic::decode(ba)?;

        let qos_flag = ba.read_byte()?;
        // The upper 6 bits of the Requested QoS byte are not used in the current version of the protocol.
        // They are reserved for future use. The Server MUST treat a SUBSCRIBE packet as malformed
        // and close the Network Connection if any of Reserved bits in the payload are non-zero,
        // or QoS is not 0,1 or 2 [MQTT-3-8.3-4].
        if qos_flag & 0b1111_0000 != 0b0000_0000 {
            return Err(DecodeError::InvalidQoS);
        }
        let qos = QoS::try_from(qos_flag & 0b0000_0011)?;

        Ok(Self { topic, qos })
    }
}

/// Subscribe packet is sent from the Client to the Server to subscribe one or more topics.
/// This packet also specifies the maximum QoS with which the Server can send Application
/// message to the Client.
///
/// Basic struct of this packet:
///
/// ```txt
/// +----------------------------+
/// | Fixed header               |
/// |                            |
/// +----------------------------+
/// | Packet Id                  |
/// |                            |
/// +----------------------------+
/// | Topic 0 length             |
/// |                            |
/// +----------------------------+
/// | Topic 0 ...                |
/// +----------------------------+
/// | Topic 0 QoS                |
/// +----------------------------+
/// | Topic 1 length             |
/// |                            |
/// +----------------------------+
/// | Topic 1 ...                |
/// +----------------------------+
/// | Tpoic 1 QoS                |
/// +----------------------------+
/// | ...                        |
/// +----------------------------+
/// ```
///
/// Each topic name is followed by associated QoS flag.
///
/// If a Server receives a Subscribe packet containing a Topic Filter that is identical
/// to an existing Subscription's Topic Filter then it must completely replace existing
/// Subscription with a new Subscription. The Topic Filter in the new Subscription will
/// be identical to the previous Subscription, also QoS may be different. Any existing
/// retained message will be re-sent to the new Subscrption.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SubscribePacket {
    /// `packet_id` is used by the Server to reply SubscribeAckPacket to the client.
    packet_id: PacketId,

    /// A list of topic the Client subscribes to.
    topics: Vec<SubscribeTopic>,
}

impl SubscribePacket {
    pub fn new(topic: &str, qos: QoS, packet_id: PacketId) -> Result<SubscribePacket, EncodeError> {
        let topic = SubscribeTopic::new(topic, qos)?;
        Ok(SubscribePacket {
            packet_id,
            topics: vec![topic],
        })
    }

    pub fn set_packet_id(&mut self, packet_id: PacketId) -> &mut Self {
        self.packet_id = packet_id;
        self
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    pub fn set_topics(&mut self, topics: &[SubscribeTopic]) -> &mut Self {
        self.topics.clear();
        self.topics.extend_from_slice(topics);
        self
    }

    pub fn topics(&self) -> &[SubscribeTopic] {
        &self.topics
    }

    pub fn mut_topics(&mut self) -> &mut Vec<SubscribeTopic> {
        &mut self.topics
    }
}

impl DecodePacket for SubscribePacket {
    fn decode(ba: &mut ByteArray) -> Result<SubscribePacket, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::Subscribe {
            return Err(DecodeError::InvalidPacketType);
        }

        let packet_id = PacketId::decode(ba)?;
        if packet_id.value() == 0 {
            // SUBSCRIBE, UNSUBSCRIBE, and PUBLISH (in cases where QoS > 0) Control Packets
            // MUST contain a non-zero 16-bit Packet Identifier. [MQTT-2.3.1-1]
            return Err(DecodeError::InvalidPacketId);
        }

        let mut topics = Vec::new();
        let mut remaining_length = packet_id.bytes();

        // Parse topic/qos list.
        while remaining_length < fixed_header.remaining_length() {
            let topic = SubscribeTopic::decode(ba)?;
            remaining_length += topic.bytes();
            topics.push(topic);
        }

        // The payload of a SUBSCRIBE packet MUST contain at least one Topic Filter / QoS pair.
        // A SUBSCRIBE packet with no payload is a protocol violation [MQTT-3.8.3-3].
        if topics.is_empty() {
            return Err(DecodeError::EmptyTopicFilter);
        }

        Ok(SubscribePacket { packet_id, topics })
    }
}

impl EncodePacket for SubscribePacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let mut remaining_length = self.packet_id.bytes();
        for topic in &self.topics {
            remaining_length += topic.bytes();
        }

        let fixed_header = FixedHeader::new(PacketType::Subscribe, remaining_length)?;
        fixed_header.encode(buf)?;

        // Variable header
        self.packet_id.encode(buf)?;

        // Payload
        for topic in &self.topics {
            topic.encode(buf)?;
        }

        Ok(buf.len() - old_len)
    }
}

impl Packet for SubscribePacket {
    fn packet_type(&self) -> PacketType {
        PacketType::Subscribe
    }
}