// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType, ReasonCode};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId};

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
/// | Reason Code        |
/// +--------------------+
/// | Property Length    |
/// +--------------------+
/// | Properties ...     |
/// +--------------------+
/// ```
///
/// This type of packet does not contain payload.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PublishAckPacket {
    packet_id: PacketId,

    /// Byte 3 in the Variable Header is the PUBACK Reason Code. If the Remaining Length is 2,
    /// then there is no Reason Code and the value of 0x00 (Success) is used.
    reason_code: ReasonCode,

    /// The length of the Properties in the PUBACK packet Variable Header encoded
    /// as a Variable Byte Integer.  If the Remaining Length is less than 4
    /// there is no Property Length and the value of 0 is used.
    properties: Properties,
}

/// The Client or Server sending the PUBACK packet MUST use one of the PUBACK Reason Codes[MQTT-3.4.2-1].
///
/// The Reason Code and Property Length can be omitted if the Reason Code is 0x00 (Success)
/// and there are no Properties. In this case the PUBACK has a Remaining Length of 2.
pub const PUBLISH_ACK_REASONS: &[ReasonCode] = &[
    ReasonCode::Success,
    ReasonCode::NoMatchingSubscribers,
    ReasonCode::UnspecifiedError,
    ReasonCode::ImplementationSpecificError,
    ReasonCode::NotAuthorized,
    ReasonCode::TopicNameInvalid,
    ReasonCode::PacketIdentifierInUse,
    ReasonCode::QuotaExceeded,
    ReasonCode::PayloadFormatInvalid,
];

pub const PUBLISH_ACK_PROPERTIES: &[PropertyType] = &[
    // The sender uses this value to give additional information to the receiver.
    // The sender MUST NOT send this property if it would increase the size of the PUBACK packet
    // beyond the Maximum Packet Size specified by the receiver [MQTT-3.4.2-2].
    PropertyType::ReasonString,
    // The sender MUST NOT send this property if it would increase the size of the PUBACK
    // packet beyond the Maximum Packet Size specified by the receiver [MQTT-3.4.2-3].
    PropertyType::UserProperty,
];

impl PublishAckPacket {
    pub fn new(packet_id: PacketId) -> Self {
        Self {
            packet_id,
            ..Self::default()
        }
    }

    pub fn set_packet_id(&mut self, packet_id: PacketId) -> &mut Self {
        self.packet_id = packet_id;
        self
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    pub fn set_reason_code(&mut self, reason_code: ReasonCode) -> &mut Self {
        self.reason_code = reason_code;
        self
    }

    pub fn reason_code(&self) -> ReasonCode {
        self.reason_code
    }

    pub fn properties(&self) -> &Properties {
        &self.properties
    }

    pub fn mut_properties(&mut self) -> &mut Properties {
        &mut self.properties
    }
}

impl EncodePacket for PublishAckPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let mut packet_bytes = self.packet_id.bytes();
        if self.reason_code != ReasonCode::Success || !self.properties.is_empty() {
            packet_bytes += self.reason_code.bytes();
        }
        if !self.properties.is_empty() {
            packet_bytes += self.properties.bytes();
        }
        let fixed_header = FixedHeader::new(PacketType::PublishAck, packet_bytes)?;
        fixed_header.encode(buf)?;
        self.packet_id.encode(buf)?;
        if self.reason_code != ReasonCode::Success || !self.properties.is_empty() {
            buf.push(self.reason_code as u8);
        }
        if !self.properties.is_empty() {
            self.properties.encode(buf)?;
        }

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
            return Err(DecodeError::InvalidPacketType);
        }
        if fixed_header.remaining_length() < PacketId::const_bytes() {
            return Err(DecodeError::InvalidRemainingLength);
        }
        let packet_id = PacketId::decode(ba)?;
        let remaining_length = fixed_header.remaining_length() - packet_id.bytes();
        let reason_code = if remaining_length >= ReasonCode::const_bytes() {
            ReasonCode::decode(ba)?
        } else {
            ReasonCode::default()
        };
        if !PUBLISH_ACK_REASONS.contains(&reason_code) {
            log::error!("Invalid reason code: {:?}", reason_code);
            return Err(DecodeError::InvalidReasonCode);
        }

        let properties = if remaining_length > ReasonCode::const_bytes() {
            let properties = Properties::decode(ba)?;
            if let Err(property_type) =
                check_property_type_list(properties.props(), PUBLISH_ACK_PROPERTIES)
            {
                log::error!(
                    "v5/PublishAckPacket: property type {:?} cannot be used in properties!",
                    property_type
                );
                return Err(DecodeError::InvalidPropertyType);
            }
            properties
        } else {
            Properties::new()
        };
        Ok(PublishAckPacket {
            packet_id,
            reason_code,
            properties,
        })
    }
}