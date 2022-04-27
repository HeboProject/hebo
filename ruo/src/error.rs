// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::fmt::{self, Display};
use std::io;
use tokio_tungstenite::tungstenite;

/// Represent the types of errors.
#[derive(Clone, Debug)]
pub enum ErrorKind {
    /// Error occurred while performing I/O.
    IoError,

    /// Packet encode error.
    EncodeError,

    /// Packet decode error.
    DecodeError,

    /// Not expected packet.
    PacketError,

    /// Send packet error.
    SendError,

    /// Socket stream error.
    SocketError,

    /// Configure error.
    ConfigError,

    /// Invalid pid.
    PidError,

    /// Invalid connect status.
    InvalidConnectStatus,
}

#[derive(Clone, Debug)]
pub struct Error {
    /// Type of current error.
    kind: ErrorKind,

    /// Detail message about this error.
    message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Error {
            kind,
            message: message.to_owned(),
        }
    }

    pub fn from_string(kind: ErrorKind, message: String) -> Self {
        Error { kind, message }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::from_string(ErrorKind::IoError, format!("IoError {:?}", err))
    }
}

impl From<tungstenite::Error> for Error {
    fn from(err: tungstenite::Error) -> Self {
        Error::from_string(ErrorKind::IoError, format!("Websocket error: {:?}", err))
    }
}

impl From<tokio_rustls::webpki::Error> for Error {
    fn from(err: tokio_rustls::webpki::Error) -> Self {
        Error::from_string(ErrorKind::ConfigError, format!("webpki error: {:?}", err))
    }
}

impl From<quinn::ConnectError> for Error {
    fn from(err: quinn::ConnectError) -> Self {
        Error::from_string(
            ErrorKind::SocketError,
            format!("Quic connect error: {:?}", err),
        )
    }
}

impl From<quinn::ConnectionError> for Error {
    fn from(err: quinn::ConnectionError) -> Self {
        Error::from_string(
            ErrorKind::SocketError,
            format!("Quic connection error: {:?}", err),
        )
    }
}

impl From<quinn::ConfigError> for Error {
    fn from(err: quinn::ConfigError) -> Self {
        Error::from_string(
            ErrorKind::ConfigError,
            format!("Quic config error: {:?}", err),
        )
    }
}

impl From<quinn::WriteError> for Error {
    fn from(err: quinn::WriteError) -> Self {
        Error::from_string(
            ErrorKind::SocketError,
            format!("Quic write error: {:?}", err),
        )
    }
}

impl From<codec::EncodeError> for Error {
    fn from(err: codec::EncodeError) -> Self {
        Error::from_string(ErrorKind::EncodeError, format!("{:?}", err))
    }
}

impl From<codec::DecodeError> for Error {
    fn from(err: codec::DecodeError) -> Self {
        Error::from_string(ErrorKind::DecodeError, format!("{:?}", err))
    }
}
