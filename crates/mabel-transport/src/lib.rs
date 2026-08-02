//! Bluetooth transport abstraction for Mabel.
//!
//! Provides trait-based RFCOMM connectivity. Platform-specific implementations
//! live in submodules. `MockTransport` is always available for testing.

pub mod error;
pub mod mock;
pub mod traits;

#[cfg(windows)]
pub mod windows;

pub use error::TransportError;
pub use mock::MockTransport;
pub use traits::{ConnectionDescriptor, ConnectionStatus, RfcommConnection, RfcommTransport};

#[cfg(windows)]
pub use windows::WindowsRfcommTransport;
