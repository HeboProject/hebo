// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;
use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};

use super::{
    consts, topic, ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader,
    Packet, PacketId, PacketType, QoS,
};

/// Topic/QoS pair.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubscribeTopic {
    /// Subscribed `topic` contains wildcard characters to match interested topics with patterns.
    topic: String,

    /// Maximum level of QoS of packet the Server can send to the Client.
    qos: QoS,
}

impl SubscribeTopic {
    pub fn new(topic: String, qos: QoS) -> Self {
        Self { topic, qos }
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn qos(&self) -> QoS {
        self.qos
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
    pub fn new(topic: &str, qos: QoS, packet_id: PacketId) -> Result<SubscribePacket, DecodeError> {
        // TODO(Shaohua): Do not copy topic string.
        topic::validate_sub_topic(&topic)?;
        let topic = SubscribeTopic::new(topic.to_string(), qos);
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

        let packet_id = ba.read_u16()? as PacketId;

        let mut topics = Vec::new();
        let mut remaining_length = consts::PACKET_ID_BYTES;

        // Parse topic/qos list.
        while remaining_length < fixed_header.remaining_length() {
            let topic_len = ba.read_u16()? as usize;
            remaining_length += consts::TOPIC_LENGTH_BYTES;

            let topic = ba.read_string(topic_len)?;
            topic::validate_sub_topic(&topic)?;
            remaining_length += topic_len;

            let qos_flag = ba.read_byte()?;
            remaining_length += consts::QOS_BYTES;
            let qos = QoS::try_from(qos_flag & 0b0000_0011)?;

            let topic = SubscribeTopic::new(topic, qos);
            topics.push(topic);
        }

        if topics.is_empty() {
            return Err(DecodeError::EmptyTopics);
        }

        Ok(SubscribePacket { packet_id, topics })
    }
}

impl EncodePacket for SubscribePacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let mut remaining_length = consts::PACKET_ID_BYTES; // Variable length
        for topic in &self.topics {
            remaining_length += consts::TOPIC_LENGTH_BYTES // Topic length bytes
                + topic.topic().len() // Topic
                + consts::QOS_BYTES; // Requested QoS
        }

        let fixed_header = FixedHeader::new(PacketType::Subscribe, remaining_length);
        fixed_header.encode(buf)?;

        // Variable header
        buf.write_u16::<BigEndian>(self.packet_id)?;

        // Payload
        for topic in &self.topics {
            buf.write_u16::<BigEndian>(topic.topic().len() as u16)?;
            buf.write_all(&topic.topic().as_bytes())?;
            let qos: u8 = 0b0000_0011 & (topic.qos() as u8);
            buf.push(qos);
        }

        Ok(buf.len() - old_len)
    }
}

impl Packet for SubscribePacket {
    fn packet_type(&self) -> PacketType {
        PacketType::Subscribe
    }
}
