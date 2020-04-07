// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::error::Error;
use std::io;

/// Packet identifier
pub type PacketId = u16;

/// Convert native data types to network byte stream.
pub trait ToNetPacket {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize>;
}

pub trait FromNetPacket: Sized {
    fn from_net(buf: &[u8]) -> Result<Self, Error>;
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PacketType {
    Unknown = 0,

    /// Request to connect to broker
    ConnectCmd = 1,

    /// Broker reply to connect request
    ConnectAck = 2,

    /// Publish message
    Publish = 3,

    /// Publish acknowledgement
    PubAck = 4,

    /// Publish received
    PubRecv = 5,

    /// Publish release
    PubRel = 6,

    /// Publish complete
    PubCompl = 7,

    /// Client subscribe request
    Subscribe = 8,

    /// Subscribe acknowledgement
    SubAck = 9,

    /// Unsubscribe request
    UnSubscribe = 10,

    /// Unsubscribe acknowledgement
    UnSubAck = 11,

    /// Client ping request
    PingReq = 12,

    /// Server ping response
    PingResp = 13,

    /// Client is disconnecting
    Disconnect = 14,

    Reserved = 15,
}

impl From<u8> for PacketType {
    fn from(flag: u8) -> Self {
        match flag {
            0 => PacketType::Unknown,
            1 => PacketType::ConnectCmd,
            2 => PacketType::ConnectAck,
            3 => PacketType::Publish,
            4 => PacketType::PubAck,
            5 => PacketType::PubRecv,
            6 => PacketType::PubRel,
            7 => PacketType::PubCompl,
            8 => PacketType::Subscribe,
            9 => PacketType::SubAck,
            10 => PacketType::UnSubscribe,
            11 => PacketType::UnSubAck,
            12 => PacketType::PingReq,
            13 => PacketType::PingResp,
            14 => PacketType::Disconnect,
            15 => PacketType::Reserved,

            _ => PacketType::Unknown,
        }
    }
}

impl Default for PacketType {
    fn default() -> Self {
        PacketType::ConnectCmd
    }
}

/// Packet flags
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PacketFlags {
    Reserved,
    Publish {
        dup: bool,
        qos: QoSLevel,
        retain: bool,
    },
    Subscribe,
}

impl Default for PacketFlags {
    fn default() -> Self {
        PacketFlags::Reserved
    }
}

/// Header flags of a mqtt packet.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FixedHeader {
    pub packet_type: PacketType,
    pub packet_flags: PacketFlags,
}

impl FromNetPacket for FixedHeader {
    fn from_net(buf: &[u8]) -> Result<Self, Error> {
        if buf.len() == 0 {
            return Err(Error::PacketEmpty);
        }
        let flags = buf[0];
        let packet_type = ((flags & 0b1111_0000) >> 4).into();
        let packet_flags = match flags & 0b0000_1111 {
            0 => PacketFlags::Reserved,
            _ => return Err(Error::InvalidFixedHeader),
        };
        Ok(FixedHeader {
            packet_type,
            packet_flags,
        })
    }
}

impl ToNetPacket for FixedHeader {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        let packet_type = (self.packet_type as u8 & 0b0000_1111) << 4;
        let packet_flags = match self.packet_flags {
            PacketFlags::Reserved => 0b0000_0000,
            PacketFlags::Publish { dup, qos, retain } => {
                let dup = if dup { 0b0000_10000 } else { 0b0000_0000 };
                let qos = match qos {
                    QoSLevel::QoS0 => 0b0000_0000,
                    QoSLevel::QoS1 => 0b0000_0010,
                    QoSLevel::QoS2 => 0b0000_0100,
                };

                let retain = if retain { 0b0000_0001 } else { 0b0000_0000 };
                dup + qos + retain
            }
            PacketFlags::Subscribe => 0b0000_0010,
        };
        let flags = packet_type + packet_flags;
        v.push(flags);

        Ok(1)
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Version {
    V31 = 3,
    V311 = 4,
    V5 = 5,
}

impl Default for Version {
    fn default() -> Self {
        Version::V311
    }
}

impl ToNetPacket for Version {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        v.push(*self as u8);
        Ok(1)
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum QoSLevel {
    QoS0 = 0,
    QoS1 = 1,
    QoS2 = 2,
}

impl Default for QoSLevel {
    fn default() -> Self {
        QoSLevel::QoS0
    }
}
