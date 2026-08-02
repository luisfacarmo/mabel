//! Soundcore device protocol implementation.
//!
//! Handles packet framing, checksums, and per-model state parsers.
//! Currently targets the Soundcore Space One Pro (A3062).

pub mod framing;
pub mod models;

pub mod framing {
    //! Packet framing: header, command bytes, body, checksum.
}

pub mod models {
    //! Per-device model implementations.

    pub mod a3062 {
        //! Soundcore Space One Pro (A3062) protocol.
    }
}
