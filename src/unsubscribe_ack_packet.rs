// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::base::*;
use super::error::Error;
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use std::default::Default;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UnsubscribeAckPacket {
    packet_id: PacketId,
}

impl FromNetPacket for UnsubscribeAckPacket {
    fn from_net(buf: &[u8]) -> Result<UnsubscribeAckPacket, Error> {
        if buf.len() == 0 {
            return Err(Error::PacketEmpty);
        }
        let mut offset = 0;
        let fixed_header = FixedHeader::from_net(buf)?;
        offset += 1;
        let remaining_len = buf[offset] as usize;
        assert_eq!(remaining_len, 2);
        offset += 1;
        let packet_id = BigEndian::read_u16(&buf[offset..offset + 2]) as PacketId;
        offset += 2;

        Ok(UnsubscribeAckPacket { packet_id })
    }
}

impl UnsubscribeAckPacket {
    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }
}