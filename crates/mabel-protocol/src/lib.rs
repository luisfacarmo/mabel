//! Soundcore device protocol implementation.
//!
//! Handles packet framing, checksums, and per-model state parsers.
//! Currently targets the Soundcore Space One Pro (A3062).

pub mod error;
pub mod framing;
pub mod models;
pub mod stream;

pub use error::ProtocolError;
pub use framing::{Direction, Packet};
pub use stream::PacketStream;
