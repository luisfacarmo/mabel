# Phase B — RFCOMM Transport Layer (Windows)

## Goal

Connect to the Space One Pro via Bluetooth RFCOMM on Windows.
At the end of this phase, `cargo test -p mabel-transport` connects to a real device and reads raw bytes.

## Design Principles Applied

- **Modular:** Trait-based — any consumer uses `dyn RfcommTransport`, never touches Windows types.
- **Testable day-1:** `MockTransport` included from Task B1. Protocol and core crates test against it without hardware.
- **Abstract:** Platform-specific code lives in `src/windows/`. Adding Linux/macOS later means adding `src/linux/` without changing the trait.
- **No monoliths:** Transport does ONE thing — bytes in, bytes out. No parsing, no state, no reconnection logic.

---

## Task B1: Define `BluetoothTransport` trait

**Objective:** Create the async trait interface that abstracts Bluetooth RFCOMM connections.

**Steps:**
1. Define `TransportError` enum (already stubbed, expand it)
2. Define `ConnectionDescriptor` struct: `{ name: String, mac_address: MacAddr6 }`
3. Define `ConnectionStatus` enum: `Connected | Disconnected`
4. Define trait `RfcommTransport`:
   - `async fn discover() -> Result<HashSet<ConnectionDescriptor>>`
   - `async fn connect(mac_address: MacAddr6) -> Result<Box<dyn RfcommConnection>>`
5. Define trait `RfcommConnection`:
   - `async fn write(&self, data: &[u8]) -> Result<()>`
   - `fn read_channel(&self) -> mpsc::Receiver<Vec<u8>>`
   - `fn connection_status(&self) -> watch::Receiver<ConnectionStatus>`
6. Add dependencies to `Cargo.toml`: `macaddr`, `async-trait`, `uuid`

**Reference:** `OpenSCQ30/lib/src/api/connection.rs`

7. Implement `MockRfcommTransport` + `MockRfcommConnection`:
   - Pre-loadable `rx_queue: VecDeque<Vec<u8>>` for simulated incoming packets
   - `tx_log: Vec<Vec<u8>>` records all outgoing writes for test assertions
   - Mirrors the pattern from `baseus-desktop/crates/baseus-transport/src/lib.rs`
   - Available to `mabel-protocol` and `mabel-core` for unit tests without hardware

**Reference (Mock):** `baseus-desktop/crates/baseus-transport/src/lib.rs` → `MockTransport`

**Test:** Trait compiles. MockTransport usable in unit tests across workspace.

**Demo:** `cargo check -p mabel-transport` passes. Mock is importable by other crates.

---

## Task B2: Implement `WindowsRfcommTransport` — device discovery

**Objective:** Find paired Soundcore devices using WinRT `DeviceInformation` API.

**Steps:**
1. Add `windows` crate dependency with features:
   - `Devices_Bluetooth`, `Devices_Bluetooth_Rfcomm`
   - `Devices_Enumeration`
   - `Networking_Sockets`
   - `Storage_Streams`
   - `Foundation`
2. Create `src/windows/mod.rs` and `src/windows/rfcomm.rs`
3. Implement `discover()`:
   - Use `DeviceInformation::FindAllAsyncAqsFilter` with connected Bluetooth filter
   - Extract name + MAC address from each result
   - Return `HashSet<ConnectionDescriptor>`
4. Use `tokio::task::spawn_blocking` for WinRT calls (they're synchronous)

**Reference:** `OpenSCQ30/lib/src/connection_backend/windows/rfcomm.rs` → `devices()` method

**Test:** Integration test that lists paired Bluetooth devices (needs real BT adapter).

**Demo:** Running the test prints discovered devices to console.

---

## Task B3: Implement `WindowsRfcommTransport` — connect via StreamSocket

**Objective:** Establish RFCOMM socket connection to a specific device by MAC address.

**Steps:**
1. Implement `connect(mac_address)`:
   - Find device by MAC using AQS filter
   - Get RFCOMM services from device
   - Select appropriate service UUID (Soundcore uses SPP or custom UUID)
   - Create `StreamSocket` and connect with encryption
2. Implement `WindowsRfcommConnection` struct:
   - Hold `AgileReference<StreamSocket>` and `AgileReference<BluetoothDevice>`
   - Implement `write()`: use `DataWriter` → `OutputStream`
   - Implement `read_channel()`: spawn blocking thread with `InputStream` + `DataReader`
   - Implement `connection_status()`: register `ConnectionStatusChanged` event
3. Implement `Drop` for cleanup (close socket, remove event handler)

**Reference:** `OpenSCQ30/lib/src/connection_backend/windows/rfcomm.rs` → full file

**Test:** Integration test: connect to Space One Pro, verify connection status is Connected.

**Demo:** `cargo run --example discover` lists devices; `cargo run --example connect` connects to headphone.

---

## Task B4: Integration test with real device

**Objective:** Verify end-to-end: discover → connect → read raw bytes.

**Steps:**
1. Create `examples/discover.rs`: list all paired Bluetooth devices
2. Create `examples/connect.rs`: connect to first Soundcore device, print raw incoming bytes
3. Document: "Turn on headphone, ensure paired, run test"
4. Verify bytes are received (the device sends state updates periodically)

**Test:** Raw bytes print to console after connecting.

**Demo:** Terminal shows hex dump of incoming packets from the headphone.

---

## Success Criteria (Phase B complete)

- [ ] `cargo check -p mabel-transport` compiles on Windows
- [ ] Discovery finds paired Soundcore device
- [ ] Connection establishes successfully
- [ ] Raw bytes received from device via read channel
- [ ] Connection status reports Connected/Disconnected correctly
- [ ] Clean disconnect on drop (no resource leaks)
