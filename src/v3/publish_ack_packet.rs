// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::{FixedHeader, Packet, PacketType};
use crate::{consts, ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId};

/// Acknowledge packet for Publish message in QoS 1.
///
/// Basic packet structure:
/// ```txt
///  7                  0
/// +--------------------+
/// | Fixed header       |
/// |                    |
/// +--------------------+
/// | Packet id          |
/// |                    |
/// +--------------------+
/// ```
///
/// This type of packet does not contain payload.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PublishAckPacket {
    packet_id: PacketId,
}

impl PublishAckPacket {
    pub fn new(packet_id: PacketId) -> Self {
        Self { packet_id }
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }
}

impl EncodePacket for PublishAckPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let fixed_header = FixedHeader::new(PacketType::PublishAck, consts::PACKET_ID_BYTES)?;
        fixed_header.encode(buf)?;
        self.packet_id.encode(buf)?;
        Ok(buf.len() - old_len)
    }
}

impl Packet for PublishAckPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::PublishAck
    }
}

impl DecodePacket for PublishAckPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::PublishAck {
            Err(DecodeError::InvalidPacketType)
        } else if fixed_header.remaining_length() != consts::PACKET_ID_BYTES {
            Err(DecodeError::InvalidRemainingLength)
        } else {
            let packet_id = PacketId::decode(ba)?;
            Ok(PublishAckPacket { packet_id })
        }
    }
}