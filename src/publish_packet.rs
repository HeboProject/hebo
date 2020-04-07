// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::base::*;
use byteorder::{BigEndian, WriteBytesExt};
use std::io::{Result, Write};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct PublishPacket {
    pub fixed_header: FixedHeader,
    topic: Vec<u8>,
    msg: Vec<u8>,
}

impl ToNetPacket for PublishPacket {
    fn to_net(&self, v: &mut Vec<u8>) -> Result<usize> {
        let old_len = v.len();
        self.fixed_header.to_net(v)?;
        v.push(self.msg_len());
        v.write_u16::<BigEndian>(self.topic.len() as u16)?;
        v.write(&self.topic)?;
        v.write(&self.msg)?;

        Ok(v.len() - old_len)
    }
}

impl PublishPacket {
    pub fn new(topic: &[u8]) -> PublishPacket {
        let fixed_header = FixedHeader {
            packet_type: PacketType::Publish,
            packet_flags: PacketFlags::Publish {
                dup: false,
                qos: QoSLevel::QoS0,
                retain: false,
            },
        };
        PublishPacket {
            fixed_header: fixed_header,
            topic: Vec::from(topic),
            msg: vec![],
        }
    }

    fn topic(&self) -> &str {
        self.topic
    }

    fn message(&self) -> &[u8] {
        self.msg
    }

    pub fn msg_len(&self) -> u8 {
        (
            2 // topic len
         + self.topic.len() // topic
         + self.msg.len()
            // message
        ) as u8
    }
}
