use mabel_transport::RfcommTransport;
use mabel_transport::WindowsRfcommTransport;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("Scanning for connected Bluetooth devices...\n");

    let transport = WindowsRfcommTransport::default();

    match transport.discover().await {
        Ok(devices) => {
            if devices.is_empty() {
                println!("No connected Bluetooth devices found.");
                println!("Make sure your headphones are powered on and paired.");
            } else {
                println!("Found {} device(s):\n", devices.len());
                for device in &devices {
                    let is_soundcore = device.name.contains("soundcore")
                        || device.name.contains("Space One");
                    let marker = if is_soundcore { " <-- TARGET" } else { "" };
                    println!("  {} ({}){}", device.name, device.mac_address, marker);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
        }
    }
}
