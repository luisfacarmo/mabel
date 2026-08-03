use std::collections::VecDeque;
use std::sync::Mutex;

use async_trait::async_trait;
use tokio::sync::{mpsc, watch};

use crate::error::Result;
use crate::traits::{ConnectionDescriptor, ConnectionStatus, RfcommConnection, RfcommTransport};

/// Mock transport for unit tests. Pre-load devices and packets.
pub struct MockTransport {
    pub devices: Vec<ConnectionDescriptor>,
}

impl MockTransport {
    pub fn new(devices: Vec<ConnectionDescriptor>) -> Self {
        Self { devices }
    }
}

#[async_trait]
impl RfcommTransport for MockTransport {
    async fn discover(&self) -> Result<Vec<ConnectionDescriptor>> {
        Ok(self.devices.clone())
    }

    async fn connect(&self, _mac_address: &str) -> Result<Box<dyn RfcommConnection>> {
        Ok(Box::new(MockConnection::new()))
    }
}

/// Mock connection for testing send/receive without hardware.
pub struct MockConnection {
    pub rx_queue: Mutex<VecDeque<Vec<u8>>>,
    pub tx_log: Mutex<Vec<Vec<u8>>>,
    status_tx: watch::Sender<ConnectionStatus>,
}

impl MockConnection {
    pub fn new() -> Self {
        let (status_tx, _) = watch::channel(ConnectionStatus::Connected);
        Self {
            rx_queue: Mutex::new(VecDeque::new()),
            tx_log: Mutex::new(Vec::new()),
            status_tx,
        }
    }

    pub fn push_rx(&self, data: Vec<u8>) {
        self.rx_queue.lock().unwrap().push_back(data);
    }

    pub fn simulate_disconnect(&self) {
        self.status_tx.send_replace(ConnectionStatus::Disconnected);
    }
}

impl Default for MockConnection {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RfcommConnection for MockConnection {
    async fn write(&self, data: &[u8]) -> Result<()> {
        self.tx_log.lock().unwrap().push(data.to_vec());
        Ok(())
    }

    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>> {
        let (tx, rx) = mpsc::channel(100);
        let packets: Vec<Vec<u8>> = self.rx_queue.lock().unwrap().drain(..).collect();
        tokio::spawn(async move {
            for pkt in packets {
                let _ = tx.send(pkt).await;
            }
        });
        rx
    }

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.status_tx.subscribe()
    }
}
