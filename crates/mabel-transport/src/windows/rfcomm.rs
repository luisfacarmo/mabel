use async_trait::async_trait;
use tracing::{debug, instrument};
use windows::Devices::{
    Bluetooth::{BluetoothConnectionStatus, BluetoothDevice},
    Enumeration::DeviceInformation,
};

use crate::error::{Result, TransportError};
use crate::traits::{ConnectionDescriptor, RfcommTransport};

#[derive(Default)]
pub struct WindowsRfcommTransport;

#[async_trait]
impl RfcommTransport for WindowsRfcommTransport {
    #[instrument(skip(self))]
    async fn discover(&self) -> Result<Vec<ConnectionDescriptor>> {
        tokio::task::spawn_blocking(|| discover_blocking())
            .await
            .map_err(|e| TransportError::Platform(format!("task join error: {e}")))?
    }
}

fn discover_blocking() -> Result<Vec<ConnectionDescriptor>> {
    let selector = BluetoothDevice::GetDeviceSelectorFromConnectionStatus(
        BluetoothConnectionStatus::Connected,
    )
    .map_err(|_| TransportError::AdapterUnavailable)?;

    let filter = format!(
        "{} AND System.Devices.Aep.IsPresent:=System.StructuredQueryType.Boolean#True",
        selector
    );

    debug!("Querying devices with filter: {}", &filter);

    let devices = DeviceInformation::FindAllAsyncAqsFilter(&filter.into())
        .map_err(|e| TransportError::Platform(format!("FindAllAsync failed: {e}")))?
        .get()
        .map_err(|e| TransportError::Platform(format!("FindAllAsync get failed: {e}")))?;

    let count = devices
        .Size()
        .map_err(|e| TransportError::Platform(format!("Size failed: {e}")))?;

    debug!("Found {} connected Bluetooth devices", count);

    let mut descriptors = Vec::with_capacity(count as usize);

    for i in 0..count {
        let device_info = devices
            .GetAt(i)
            .map_err(|e| TransportError::Platform(format!("GetAt failed: {e}")))?;

        let id = device_info
            .Id()
            .map_err(|e| TransportError::Platform(format!("Id failed: {e}")))?;

        let bt_device = BluetoothDevice::FromIdAsync(&id)
            .map_err(|e| TransportError::Platform(format!("FromIdAsync failed: {e}")))?
            .get()
            .map_err(|e| TransportError::Platform(format!("FromIdAsync get failed: {e}")))?;

        let name = bt_device
            .Name()
            .map_err(|e| TransportError::Platform(format!("Name failed: {e}")))?
            .to_string_lossy();

        let mac_u64 = bt_device
            .BluetoothAddress()
            .map_err(|e| TransportError::Platform(format!("BluetoothAddress failed: {e}")))?;

        let mac_address = format_mac(mac_u64);

        debug!("  Device: {} ({})", &name, &mac_address);
        descriptors.push(ConnectionDescriptor { name, mac_address });
    }

    Ok(descriptors)
}

fn format_mac(addr: u64) -> String {
    format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        (addr >> 40) & 0xFF,
        (addr >> 32) & 0xFF,
        (addr >> 24) & 0xFF,
        (addr >> 16) & 0xFF,
        (addr >> 8) & 0xFF,
        addr & 0xFF,
    )
}
