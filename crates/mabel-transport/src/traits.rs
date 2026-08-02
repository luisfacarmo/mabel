use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, watch};

use crate::error::Result;

/// Describes a discovered Bluetooth device.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConnectionDescriptor {
    pub name: String,
    pub mac_address: String, // hex string "AA:BB:CC:DD:EE:FF"
}

/// Connection status observable by consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

/// Backend that discovers devices and creates connections.
#[async_trait]
pub trait RfcommTransport: Send + Sync {
    /// List currently connected/paired Bluetooth devices.
    async fn discover(&self) -> Result<Vec<ConnectionDescriptor>>;
}

/// An active RFCOMM connection to a device.
#[async_trait]
pub trait RfcommConnection: Send + Sync {
    /// Send raw bytes to the device.
    async fn write(&self, data: &[u8]) -> Result<()>;

    /// Receive channel for incoming packets.
    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>>;

    /// Watch the connection status.
    fn connection_status(&self) -> watch::Receiver<ConnectionStatus>;
}
