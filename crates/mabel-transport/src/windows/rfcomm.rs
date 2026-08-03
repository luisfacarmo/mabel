//! Windows RFCOMM transport — device discovery + StreamSocket connection.
//!
//! Uses the WinRT `windows` crate to:
//! - Discover paired Bluetooth devices via DeviceInformation API
//! - Connect via StreamSocket to the device's RFCOMM SPP service
//! - Read incoming data on a background thread, forwarding via mpsc channel
//! - Monitor connection status via BluetoothDevice events

use std::sync::Mutex;
use std::thread;

use async_trait::async_trait;
use tokio::sync::{mpsc, watch};
use tracing::{debug, info, instrument, warn};
use windows::{
    Devices::{
        Bluetooth::{
            BluetoothConnectionStatus, BluetoothDevice,
        },
        Enumeration::DeviceInformation,
    },
    Foundation::TypedEventHandler,
    Networking::Sockets::{SocketProtectionLevel, StreamSocket},
    Storage::Streams::{Buffer, DataReader, DataWriter, InputStreamOptions},
    core::{AgileReference, HSTRING},
};

use crate::error::{Result, TransportError};
use crate::traits::{ConnectionDescriptor, ConnectionStatus, RfcommConnection, RfcommTransport};

#[derive(Default)]
pub struct WindowsRfcommTransport;

#[async_trait]
impl RfcommTransport for WindowsRfcommTransport {
    #[instrument(skip(self))]
    async fn discover(&self) -> Result<Vec<ConnectionDescriptor>> {
        tokio::task::spawn_blocking(discover_blocking)
            .await
            .map_err(|e| TransportError::Platform(format!("task join error: {e}")))?
    }

    #[instrument(skip(self))]
    async fn connect(&self, mac_address: &str) -> Result<Box<dyn RfcommConnection>> {
        let mac = mac_address.to_owned();
        tokio::task::spawn_blocking(move || connect_blocking(&mac))
            .await
            .map_err(|e| TransportError::Platform(format!("task join error: {e}")))?
    }
}

// --- Discovery ---

fn discover_blocking() -> Result<Vec<ConnectionDescriptor>> {
    let selector = BluetoothDevice::GetDeviceSelectorFromConnectionStatus(
        BluetoothConnectionStatus::Connected,
    )
    .map_err(|_| TransportError::AdapterUnavailable)?;

    let filter: HSTRING = format!(
        "{} AND System.Devices.Aep.IsPresent:=System.StructuredQueryType.Boolean#True",
        selector
    )
    .into();

    debug!("querying devices with filter");

    let devices = DeviceInformation::FindAllAsyncAqsFilter(&filter)
        .map_err(|e| TransportError::Platform(format!("FindAllAsync failed: {e}")))?
        .get()
        .map_err(|e| TransportError::Platform(format!("FindAllAsync get failed: {e}")))?;

    let count = devices
        .Size()
        .map_err(|e| TransportError::Platform(format!("Size failed: {e}")))?;

    debug!("found {} connected Bluetooth devices", count);

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
            .map_err(|e| TransportError::Platform(format!("FromIdAsync get: {e}")))?;

        let name = bt_device
            .Name()
            .map_err(|e| TransportError::Platform(format!("Name failed: {e}")))?
            .to_string_lossy();

        let mac_u64 = bt_device
            .BluetoothAddress()
            .map_err(|e| TransportError::Platform(format!("BluetoothAddress failed: {e}")))?;

        let mac_address = format_mac(mac_u64);
        debug!("  device: {} ({})", &name, &mac_address);
        descriptors.push(ConnectionDescriptor { name, mac_address });
    }

    Ok(descriptors)
}

// --- Connection ---

fn connect_blocking(mac_address: &str) -> Result<Box<dyn RfcommConnection>> {
    info!("connecting to {}", mac_address);

    let device = find_device_by_mac(mac_address)?;

    // Get all RFCOMM services
    let services_result = device
        .GetRfcommServicesAsync()
        .map_err(|e| TransportError::Platform(format!("GetRfcommServicesAsync: {e}")))?
        .get()
        .map_err(|e| TransportError::Platform(format!("GetRfcommServicesAsync get: {e}")))?;

    let services = services_result
        .Services()
        .map_err(|e| TransportError::Platform(format!("Services: {e}")))?;

    let count = services.Size().unwrap_or(0);
    debug!("found {} RFCOMM services, trying each...", count);

    if count == 0 {
        return Err(TransportError::ConnectionFailed(
            "no RFCOMM services found on device".into(),
        ));
    }

    // Try each service until one connects successfully
    let mut last_error = String::new();

    for i in 0..count {
        let service = match services.GetAt(i) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let uuid_str = service
            .ServiceId()
            .and_then(|id| id.Uuid())
            .map(|u| format!("{:?}", u))
            .unwrap_or_else(|_| "unknown".into());

        debug!("  trying service [{}]: {}", i, uuid_str);

        let socket = match StreamSocket::new() {
            Ok(s) => s,
            Err(e) => {
                debug!("    StreamSocket::new failed: {e}");
                continue;
            }
        };

        let host_name = match service.ConnectionHostName() {
            Ok(h) => h,
            Err(e) => {
                debug!("    ConnectionHostName failed: {e}");
                continue;
            }
        };

        let service_name = match service.ConnectionServiceName() {
            Ok(s) => s,
            Err(e) => {
                debug!("    ConnectionServiceName failed: {e}");
                continue;
            }
        };

        match socket
            .ConnectWithProtectionLevelAsync(
                &host_name,
                &service_name,
                SocketProtectionLevel::BluetoothEncryptionAllowNullAuthentication,
            )
            .and_then(|op| op.get())
        {
            Ok(()) => {
                info!("  -> connected via service [{}]: {}", i, uuid_str);

                let socket_ref = AgileReference::new(&socket)
                    .map_err(|e| TransportError::Platform(format!("AgileRef socket: {e}")))?;
                let device_ref = AgileReference::new(&device)
                    .map_err(|e| TransportError::Platform(format!("AgileRef device: {e}")))?;

                let read_rx = spawn_read_channel(&socket)?;

                let (status_tx, status_rx) = watch::channel(ConnectionStatus::Connected);
                let status_token = device
                    .ConnectionStatusChanged(&TypedEventHandler::new(
                        move |dev: windows::core::Ref<'_, BluetoothDevice>, _| {
                            if let Some(dev) = dev.as_ref() {
                                let s = if dev.ConnectionStatus()?
                                    == BluetoothConnectionStatus::Connected
                                {
                                    ConnectionStatus::Connected
                                } else {
                                    ConnectionStatus::Disconnected
                                };
                                status_tx.send_replace(s);
                            }
                            Ok(())
                        },
                    ))
                    .map_err(|e| TransportError::Platform(format!("StatusChanged: {e}")))?;

                return Ok(Box::new(WindowsRfcommConnection {
                    device: device_ref,
                    socket: socket_ref,
                    read_channel: Mutex::new(Some(read_rx)),
                    status_rx,
                    _status_token: status_token,
                }));
            }
            Err(e) => {
                debug!("    connect failed: {e}");
                last_error = e.to_string();
                continue;
            }
        }
    }

    Err(TransportError::ConnectionFailed(format!(
        "all {} services failed, last error: {}",
        count, last_error
    )))
}

fn find_device_by_mac(mac_address: &str) -> Result<BluetoothDevice> {
    let mac_hex = mac_address.replace(':', "");

    let connected_filter = BluetoothDevice::GetDeviceSelectorFromConnectionStatus(
        BluetoothConnectionStatus::Connected,
    )
    .map_err(|_| TransportError::AdapterUnavailable)?;

    let filter: HSTRING = format!(
        "{} AND System.DeviceInterface.Bluetooth.DeviceAddress:=\"{}\" AND System.Devices.Aep.IsPresent:=System.StructuredQueryType.Boolean#True",
        connected_filter, mac_hex
    ).into();

    debug!("finding device with MAC filter");

    let devices = DeviceInformation::FindAllAsyncAqsFilter(&filter)
        .map_err(|e| TransportError::Platform(format!("FindAll: {e}")))?
        .get()
        .map_err(|e| TransportError::Platform(format!("FindAll get: {e}")))?;

    if devices.Size().unwrap_or(0) == 0 {
        return Err(TransportError::DeviceNotFound(mac_address.to_string()));
    }

    let device_info = devices
        .GetAt(0)
        .map_err(|e| TransportError::Platform(format!("GetAt: {e}")))?;

    let id = device_info
        .Id()
        .map_err(|e| TransportError::Platform(format!("Id: {e}")))?;

    BluetoothDevice::FromIdAsync(&id)
        .map_err(|e| TransportError::Platform(format!("FromIdAsync: {e}")))?
        .get()
        .map_err(|e| TransportError::Platform(format!("FromIdAsync get: {e}")))
}

fn spawn_read_channel(socket: &StreamSocket) -> Result<mpsc::Receiver<Vec<u8>>> {
    let (tx, rx) = mpsc::channel(100);
    let stream_ref = AgileReference::new(
        &socket
            .InputStream()
            .map_err(|e| TransportError::Platform(format!("InputStream: {e}")))?,
    )
    .map_err(|e| TransportError::Platform(format!("AgileReference stream: {e}")))?;

    thread::spawn(move || {
        let result = (|| -> windows::core::Result<()> {
            let buffer = Buffer::Create(1024)?;
            let stream = stream_ref.resolve()?;

            loop {
                stream
                    .ReadAsync(&buffer, 1024, InputStreamOptions::Partial)?
                    .get()?;

                let len = buffer.Length()? as usize;
                if len == 0 {
                    debug!("read channel: zero-length read, disconnected");
                    break;
                }

                let mut data = vec![0u8; len];
                let reader = DataReader::FromBuffer(&buffer)?;
                reader.ReadBytes(&mut data)?;

                if tx.blocking_send(data).is_err() {
                    debug!("read channel: receiver dropped, stopping");
                    break;
                }
            }
            Ok(())
        })();

        match result {
            Ok(()) => debug!("read channel: closed cleanly"),
            Err(e) => warn!("read channel: error: {e}"),
        }
    });

    Ok(rx)
}

// --- Connection struct ---

struct WindowsRfcommConnection {
    #[allow(dead_code)]
    device: AgileReference<BluetoothDevice>,
    socket: AgileReference<StreamSocket>,
    read_channel: Mutex<Option<mpsc::Receiver<Vec<u8>>>>,
    status_rx: watch::Receiver<ConnectionStatus>,
    #[allow(dead_code)]
    _status_token: i64,
}

unsafe impl Send for WindowsRfcommConnection {}
unsafe impl Sync for WindowsRfcommConnection {}

#[async_trait]
impl RfcommConnection for WindowsRfcommConnection {
    async fn write(&self, data: &[u8]) -> Result<()> {
        let socket = self.socket.clone();
        let data = data.to_owned();

        tokio::task::spawn_blocking(move || -> Result<()> {
            let writer = DataWriter::new()
                .map_err(|e| TransportError::Platform(format!("DataWriter::new: {e}")))?;
            writer
                .WriteBytes(&data)
                .map_err(|e| TransportError::Platform(format!("WriteBytes: {e}")))?;
            let buffer = writer
                .DetachBuffer()
                .map_err(|e| TransportError::Platform(format!("DetachBuffer: {e}")))?;

            let stream = socket
                .resolve()
                .map_err(|e| TransportError::Platform(format!("resolve socket: {e}")))?
                .OutputStream()
                .map_err(|e| TransportError::Platform(format!("OutputStream: {e}")))?;

            stream
                .WriteAsync(&buffer)
                .map_err(|e| TransportError::Platform(format!("WriteAsync: {e}")))?
                .get()
                .map_err(|e| TransportError::Platform(format!("WriteAsync get: {e}")))?;

            Ok(())
        })
        .await
        .map_err(|e| TransportError::Platform(format!("write join: {e}")))?
    }

    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>> {
        self.read_channel
            .lock()
            .unwrap()
            .take()
            .expect("read_channel() can only be called once")
    }

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.status_rx.clone()
    }
}

// --- Utilities ---

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
