# Phase D — Core Layer (Device Manager + State Machine)

## Goal

Orchestrate transport + protocol into a usable device manager.
At the end of this phase, a CLI example connects, prints state, and changes ANC mode.

## Design Principles Applied

- **Modular:** `mabel-core` is a thin orchestration crate — no event loops, no framework coupling.
- **Abstract:** Consumers (Tauri app, CLI, tests) control the event loop. Core only provides building blocks.
- **No monoliths:** The device loop lives in the consumer (src-tauri or CLI example), NOT in the core crate. Core exposes composable pieces.
- **Testable day-1:** MockTransport included from Phase B. All core logic is unit-testable without hardware.

---

## Task D1: Create DeviceManager (stateless orchestrator)

**Objective:** High-level API that composes transport + protocol without owning the event loop.

**Steps:**
1. Create `src/manager.rs` in `mabel-core`
2. Implement `DeviceManager`:
   ```rust
   pub struct DeviceManager {
       transport: Box<dyn RfcommTransport>,
       state: watch::Sender<Option<A3062State>>,
   }
   ```
3. Methods (all pure, no spawning):
   - `fn new(transport) -> Self`
   - `async fn discover() -> Result<Vec<ConnectionDescriptor>>`
   - `async fn connect(mac: MacAddr6) -> Result<RfcommConnection>`
   - `async fn request_state(conn: &dyn RfcommConnection) -> Result<A3062State>`
   - `async fn send_command(conn: &dyn RfcommConnection, cmd: DeviceCommand) -> Result<()>`
   - `fn parse_incoming(bytes: &[u8]) -> Result<Option<A3062State>>` (stateless parser)
4. The manager does NOT own the connection or run a loop — it provides operations that the consumer orchestrates.

**Why this shape:** The Tauri app runs its own `select!` loop (like baseus-desktop's `device.rs`). The CLI runs a different loop. Tests run no loop at all. If core owned the loop, each consumer would fight it.

**Reference:** `baseus-desktop/apps/baseus-app/src-tauri/src/device.rs` (loop is in consumer, not in crate)

**Test:** Unit test with mock transport: `request_state()` returns parsed state.

**Demo:** `cargo test -p mabel-core manager` passes.

---

## Task D2: Implement packet handler (incoming → state)

**Objective:** Modular function that takes raw incoming bytes and produces state updates.

**Steps:**
1. Create `src/handler.rs` with:
   ```rust
   /// Process incoming bytes through the PacketStream and return any state updates.
   pub fn handle_incoming(
       stream: &mut PacketStream,
       bytes: &[u8],
   ) -> Vec<StateEvent>
   ```
2. `StateEvent` enum:
   - `StateEvent::FullUpdate(A3062State)` — from command `[1,1]` response
   - `StateEvent::Ack(Command)` — acknowledgement of a sent command
3. Build ACK packet when state update received:
   - `pub fn build_ack(command: Command) -> Vec<u8>`
4. This is a pure function — no tokio, no channels, no side effects. Consumers decide what to do with the events.

**Why pure:** The Tauri app emits events. The CLI prints to stdout. Tests assert on return values. If this function had side effects, each consumer would need to mock them.

**Test:** Feed test vector bytes → get `StateEvent::FullUpdate` with correct parsed state.

**Demo:** `cargo test -p mabel-core handler` passes.

---

## Task D3: Implement command dispatch

**Objective:** Send commands to device (set ANC, set EQ, etc.).

**Steps:**
1. Add methods to `DeviceManager`:
   - `async fn set_sound_modes(modes: SoundModes) -> Result<()>`
   - `async fn set_equalizer(eq: EqualizerConfig) -> Result<()>`
   - `async fn set_button_config(config: ButtonConfig) -> Result<()>`
   - `async fn set_auto_power_off(apo: AutoPowerOff) -> Result<()>`
   - `async fn set_ldac(enabled: bool) -> Result<()>`
   - `async fn set_dolby(enabled: bool) -> Result<()>`
   - `async fn set_sidetone(enabled: bool) -> Result<()>`
2. Each method:
   - Builds command packet via `mabel-protocol`
   - Serializes to bytes
   - Sends via `connection.write()`
   - Optionally waits for ACK or state update confirmation
3. Error handling: return error if not connected

**Test:** Mock connection: send command → verify correct bytes were written.

**Demo:** `cargo test -p mabel-core commands` passes.

---

## Task D4: Add reconnection logic

**Objective:** Auto-reconnect when device disconnects unexpectedly.

**Steps:**
1. Watch `connection_status` receiver
2. On `Disconnected`:
   - Clean up old connection
   - Set state to `None`
   - Start reconnect loop (exponential backoff: 1s, 2s, 4s, max 30s)
   - On reconnect: re-request state
3. Expose reconnection status to consumers (for UI: "Reconnecting...")
4. Allow manual disconnect (stops reconnect loop)

**Test:** Mock: disconnect event → reconnect attempt → success → state restored.

**Demo:** `cargo test -p mabel-core reconnect` passes.

---

## Task D5: CLI example

**Objective:** Runnable binary that proves the full stack works.

**Steps:**
1. Create `examples/cli.rs` in `mabel-core`:
   - Discover devices
   - Connect to first Soundcore device
   - Print full state (battery, ANC mode, EQ, toggles)
   - Interactive menu: change ANC mode, toggle LDAC, etc.
   - Ctrl+C to disconnect and exit
2. Use `tracing-subscriber` for structured logging

**Test:** Manual test with real headphone.

**Demo:** Terminal CLI connects and shows device state, accepts commands.

---

## Success Criteria (Phase D complete)

- [ ] DeviceManager connects and retrieves state
- [ ] State updates flow reactively via watch channel
- [ ] Commands change device settings
- [ ] Reconnection works after unexpected disconnect
- [ ] CLI example fully functional with real device
- [ ] `cargo test -p mabel-core` — all tests pass
