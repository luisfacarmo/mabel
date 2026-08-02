//! Bluetooth transport abstraction.
//!
//! Provides a trait-based interface for connecting to Soundcore devices
//! via RFCOMM (Windows) with future support for other platforms.

pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum TransportError {
        #[error("device not found: {0}")]
        DeviceNotFound(String),
        #[error("connection failed: {0}")]
        ConnectionFailed(String),
        #[error("disconnected")]
        Disconnected,
        #[error("I/O error: {0}")]
        Io(String),
    }
}

pub use error::TransportError;
