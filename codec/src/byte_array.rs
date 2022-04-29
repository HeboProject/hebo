// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, ByteOrder};

use super::utils;

#[derive(Debug)]
pub enum ByteArrayError {
    OutOfRangeError,
    InvalidString(utils::StringError),
}

impl From<utils::StringError> for ByteArrayError {
    fn from(e: utils::StringError) -> ByteArrayError {
        ByteArrayError::InvalidString(e)
    }
}

pub struct ByteArray<'a> {
    offset: usize,
    data: &'a [u8],
}

impl<'a> ByteArray<'a> {
    /// Create a new ByteArray object based on byte slice.
    pub fn new(data: &'a [u8]) -> Self {
        ByteArray { offset: 0, data }
    }

    /// Get length of inner byte slice.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Get remaining length of bytes available to read.
    pub fn remaining_bytes(&self) -> usize {
        assert!(self.offset <= self.data.len());
        self.data.len() - self.offset
    }

    /// Read one byte from slice.
    pub fn read_byte(&mut self) -> Result<u8, ByteArrayError> {
        self.offset += 1;
        if self.offset > self.data.len() {
            Err(ByteArrayError::OutOfRangeError)
        } else {
            Ok(self.data[self.offset - 1])
        }
    }

    /// Read a u16 value from slice.
    pub fn read_u16(&mut self) -> Result<u16, ByteArrayError> {
        Ok(BigEndian::read_u16(self.read_bytes(2)?))
    }

    /// Read a u32 value from slice.
    pub fn read_u32(&mut self) -> Result<u32, ByteArrayError> {
        Ok(BigEndian::read_u32(self.read_bytes(4)?))
    }

    /// Read an UTF-8 string with `len` from slice.
    pub fn read_string(&mut self, len: usize) -> Result<String, ByteArrayError> {
        let bytes = self.read_bytes(len)?;
        utils::to_utf8_string(bytes).map_err(ByteArrayError::from)
    }

    /// Read a byte array with `len` from slice.
    pub fn read_bytes(&mut self, len: usize) -> Result<&[u8], ByteArrayError> {
        self.offset += len;
        if self.offset > self.data.len() {
            Err(ByteArrayError::OutOfRangeError)
        } else {
            Ok(&self.data[self.offset - len..self.offset])
        }
    }

    /// Reset offset value to 0.
    pub fn reset_offset(&mut self) {
        self.offset = 0;
    }
}
