//! Protocol error types.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ProtocolError>;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("invalid packet: {0}")]
    InvalidPacket(String),

    #[error("checksum mismatch: expected {expected:#04x}, got {actual:#04x}")]
    ChecksumMismatch { expected: u8, actual: u8 },

    #[error("incomplete packet: need more data")]
    Incomplete,

    #[error("unsupported command: [{0:#04x}, {1:#04x}]")]
    UnsupportedCommand(u8, u8),

    #[error("parse error: {0}")]
    ParseError(String),
}
