use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, watch};

use crate::error::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConnectionDescriptor {
    pub name: String,
    pub mac_address: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

#[async_trait]
pub trait RfcommTransport: Send + Sync {
    async fn discover(&self) -> Result<Vec<ConnectionDescriptor>>;
    async fn connect(&self, mac_address: &str) -> Result<Box<dyn RfcommConnection>>;
}

#[async_trait]
pub trait RfcommConnection: Send + Sync {
    async fn write(&self, data: &[u8]) -> Result<()>;
    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>>;
    fn connection_status(&self) -> watch::Receiver<ConnectionStatus>;
}
