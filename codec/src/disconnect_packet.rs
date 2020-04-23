// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::default::Default;
use std::io;

use crate::base::*;
use crate::error::Error;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct DisconnectPacket {}

impl DisconnectPacket {
    pub fn new() -> DisconnectPacket {
        Self::default()
    }
}

impl ToNetPacket for DisconnectPacket {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        let fixed_header = FixedHeader {
            packet_type: PacketType::Disconnect,
            packet_flags: PacketFlags::Disconnect,
            remaining_length: RemainingLength(0),  // No payload
        };
        fixed_header.to_net(v)
    }
}

impl FromNetPacket for DisconnectPacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<DisconnectPacket, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::Disconnect);
        Ok(DisconnectPacket {})
    }
}