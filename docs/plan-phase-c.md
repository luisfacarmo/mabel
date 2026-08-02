# Phase C — Soundcore Protocol (Packet Framing + A3062 Parser)

## Goal

Parse raw bytes into typed Rust structs and build command packets.
At the end of this phase, `cargo test -p mabel-protocol` parses the OpenSCQ30 test vector correctly.

---

## Task C1: Implement packet framing (header, command, body, checksum)

**Objective:** Encode/decode the Soundcore packet wire format.

**Steps:**
1. Add dependencies: `nom` (parser combinators)
2. Create `src/framing/mod.rs` with:
   - `Direction` enum: Outbound `[0x08, 0xEE, 0x00, 0x00, 0x00]`, Inbound `[0x09, 0xFF, 0x00, 0x00, 0x01]`
   - `Command` struct: `[u8; 2]` (e.g., `[1, 1]` = state request/response)
   - `Packet` struct: `{ command: Command, body: Vec<u8> }`
   - `ChecksumKind` enum: `None | Suffix`
3. Implement `Packet::parse(input: &[u8]) -> IResult<&[u8], Packet>`:
   - Parse direction header (5 bytes)
   - Parse command (2 bytes LE)
   - Parse length (2 bytes LE)
   - Calculate body length: `length - 5 - 2 - 2 - 1` (if checksum)
   - Parse body
   - Validate checksum (sum of all preceding bytes mod 256)
4. Implement `Packet::to_bytes() -> Vec<u8>`:
   - Serialize direction + command + length + body + checksum
5. Write round-trip test: `build → serialize → parse → compare`

**Reference:** `OpenSCQ30/lib/src/devices/soundcore/common/packet.rs`

**Test:** `Packet::parse(Packet::to_bytes(p)) == p` for various payloads.

**Demo:** `cargo test -p mabel-protocol framing` all pass.

---

## Task C2: Create A3062 state struct

**Objective:** Define the typed struct that holds all device state fields.

**Steps:**
1. Create `src/models/a3062/mod.rs`
2. Create `src/models/a3062/state.rs` with `A3062State`:
   ```rust
   pub struct A3062State {
       pub battery: Battery,           // level: u8 (0-10), offset 1
       pub firmware: String,           // e.g. "03.37"
       pub serial_number: String,      // e.g. "3062DB212C13E97C"
       pub sound_modes: SoundModes,
       pub equalizer: EqualizerConfig,
       pub button_config: ButtonConfig,
       pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
       pub toggles: DeviceToggles,     // dolby, ldac, sidetone, voice prompt, low battery
       pub auto_power_off: AutoPowerOff,
       pub limit_high_volume: LimitHighVolume,
       pub dual_connections: bool,
   }
   ```
3. Create sub-structs in `src/models/a3062/structures.rs`:
   - `SoundModes { mode, nc_mode, adaptive_level, custom_nc_level, custom_transparency, wind_noise }`
   - `EqualizerConfig { preset: Option<Preset>, bands: [u8; 10] }`
   - `ButtonConfig { double_press: Option<ButtonAction> }`
   - `AmbientSoundModeCycle { nc: bool, transparency: bool, normal: bool }`
   - `DeviceToggles { dolby, ldac, sidetone, voice_prompt, low_battery_prompt }`
   - `AutoPowerOff { minutes: u16 }` (0 = disabled)
   - `LimitHighVolume { enabled: bool, db_limit: u8, refresh_rate: RefreshRate }`
4. Derive `Debug, Clone, PartialEq, Serialize, Deserialize` on all structs

**Reference:** `OpenSCQ30/lib/src/devices/soundcore/a3062/state.rs`

**Test:** Structs compile and serialize to JSON.

**Demo:** `cargo test -p mabel-protocol state` passes.

---

## Task C3: Implement A3062 state parser

**Objective:** Parse the body of a command `[1, 1]` response packet into `A3062State`.

**Steps:**
1. Create `src/models/a3062/parser.rs`
2. Implement `parse_state_update(body: &[u8]) -> Result<A3062State>`:
   - Parse fields in exact order (matching OpenSCQ30 test vector byte layout):
     - Battery (1 byte, offset by 1)
     - Firmware version (5 bytes ASCII)
     - Serial number (16 bytes ASCII)
     - EQ bands (10 bytes) + 2 unknown
     - HearID data (skip/store raw for now)
     - 2 unknown bytes
     - Button config (1 byte)
     - Ambient sound mode cycle (3 bytes)
     - Sound modes (6 bytes)
     - 1 unknown byte
     - Low battery prompt (1 byte)
     - Dolby audio (1 byte)
     - LDAC (1 byte)
     - Dual connections (1 byte bool)
     - Auto power off (variable)
     - Limit high volume (variable)
     - Side tone (1 byte)
     - Ambient sound mode voice prompt (1 byte)
3. Use nom combinators for each field
4. Unit test with the known test vector from OpenSCQ30 issue #194

**Reference:** `OpenSCQ30/lib/src/devices/soundcore/a3062/packets/inbound/state_update.rs`

**Test vector (from OpenSCQ30):**
```
Body bytes: [4, 255, 48, 51, 46, 51, 55, 51, 48, 54, 50, 68, 66, 50, 49, 50, 67, 49, 51,
69, 57, 55, 67, 5, 0, 90, 140, 160, 160, 150, 140, 120, 100, 120, 0, 30, 255, 0, 255,
255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255,
255, 255, 255, 0, 0, 0, 4, 4, 7, 3, 1, 80, 1, 1, 0, 5, 49, 1, 1, 0, 1, 1, 1, 0, 90, 0,
0, 1, 0, 0, 0, 0, 0, 0, 0, 0]
```

Expected parse result:
- Battery: 5/10
- Firmware: "03.37"
- Serial: "3062DB212C13E97C"
- ANC mode: Transparency
- NC mode: Adaptive
- EQ preset: Podcast

**Demo:** `cargo test -p mabel-protocol parse_state` passes with correct values.

---

## Task C4: Implement command builders (outbound packets)

**Objective:** Build packets that change device settings.

**Steps:**
1. Create `src/models/a3062/commands.rs`
2. Implement builders:
   - `request_state() -> Packet` (command `[1, 1]`, empty body)
   - `set_sound_modes(modes: &SoundModes) -> Packet`
   - `set_equalizer(eq: &EqualizerConfig) -> Packet`
   - `set_button_config(config: &ButtonConfig) -> Packet`
   - `set_auto_power_off(apo: &AutoPowerOff) -> Packet`
   - `set_ldac(enabled: bool) -> Packet`
   - `set_dolby(enabled: bool) -> Packet`
   - `set_sidetone(enabled: bool) -> Packet`
3. Each builder produces a `Packet` with correct command bytes and serialized body
4. Unit tests: build → serialize → verify known bytes

**Reference:** `OpenSCQ30/lib/src/devices/soundcore/a3062/packets/outbound.rs`

**Test:** Command packets serialize to expected byte sequences.

**Demo:** `cargo test -p mabel-protocol commands` all pass.

---

## Task C5: Add `nom` streaming parser for packet boundaries

**Objective:** Handle partial reads from the RFCOMM stream (packets may arrive fragmented).

**Steps:**
1. Create `src/framing/stream.rs`
2. Implement `PacketStream`:
   - Internal buffer that accumulates bytes
   - `push(data: &[u8])` — append to buffer
   - `next_packet() -> Option<Packet>` — try to parse one complete packet from buffer
   - Uses nom's streaming parsers (returns `Incomplete` if not enough data)
3. Unit test: feed a packet in 3 chunks → eventually get the full parsed packet

**Test:** Fragmented input correctly reassembles into complete packets.

**Demo:** `cargo test -p mabel-protocol stream` passes.

---

## Success Criteria (Phase C complete)

- [ ] Packet framing round-trips correctly
- [ ] A3062 state parser handles the test vector from OpenSCQ30
- [ ] All parsed fields match expected values
- [ ] Command builders produce correct byte sequences
- [ ] Streaming parser handles fragmented input
- [ ] `cargo test -p mabel-protocol` — all tests pass
