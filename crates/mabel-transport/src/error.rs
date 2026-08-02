use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("bluetooth adapter unavailable")]
    AdapterUnavailable,

    #[error("device not found: {0}")]
    DeviceNotFound(String),

    #[error("connection failed: {0}")]
    ConnectionFailed(String),

    #[error("write error: {0}")]
    WriteError(String),

    #[error("disconnected")]
    Disconnected,

    #[error("platform error: {0}")]
    Platform(String),
}

pub type Result<T> = std::result::Result<T, TransportError>;
