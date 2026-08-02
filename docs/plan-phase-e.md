# Phase E — Integration (Wire Real Backend into Frontend)

## Goal

Replace mock data with real Bluetooth connection. The app becomes fully functional.
At the end of this phase, the Mabel app controls the real Space One Pro headphone.

---

## Task E1: Add Tauri commands (IPC bridge)

**Objective:** Expose `mabel-core` DeviceManager as Tauri commands callable from React.

**Steps:**
1. Add `mabel-core` as dependency in `apps/mabel-app/src-tauri/Cargo.toml`
2. Create `src-tauri/src/commands.rs` with `#[tauri::command]` functions:
   - `discover_devices() -> Vec<ConnectionDescriptor>`
   - `connect_device(mac_address: String) -> Result<(), String>`
   - `disconnect_device() -> Result<(), String>`
   - `get_state() -> Option<DeviceState>` (serialized A3062State)
   - `set_sound_modes(modes: SoundModesPayload) -> Result<(), String>`
   - `set_equalizer(eq: EqualizerPayload) -> Result<(), String>`
   - `set_toggle(name: String, value: bool) -> Result<(), String>`
   - `set_auto_power_off(minutes: u16) -> Result<(), String>`
   - `set_button_config(action: String) -> Result<(), String>`
3. Store `DeviceManager` in Tauri managed state
4. Register all commands in `tauri::Builder`

**Reference:** `baseus-desktop/apps/baseus-app/src-tauri/src/commands.rs`

**Test:** `npm run tauri dev` — call commands from browser devtools console.

**Demo:** Dev console: `invoke('discover_devices')` returns device list.

---

## Task E2: Add Tauri events (state push)

**Objective:** Push real-time state updates from backend to frontend.

**Steps:**
1. Create `src-tauri/src/events.rs`
2. On `DeviceManager` state change (watch channel):
   - Emit Tauri event `device-state-updated` with serialized state
3. On connection status change:
   - Emit `device-connection-status` event
4. Frontend listens via `@tauri-apps/api/event`

**Test:** Connect device → frontend receives state update event.

**Demo:** React DevTools shows state updating in real-time from device.

---

## Task E3: Replace mock hooks with Tauri IPC

**Objective:** Swap `useDeviceState` mock implementation with real Tauri commands.

**Steps:**
1. Create `src/lib/tauri-bridge.ts`:
   - `discoverDevices(): Promise<ConnectionDescriptor[]>`
   - `connectDevice(mac: string): Promise<void>`
   - `disconnectDevice(): Promise<void>`
   - `setSoundModes(modes): Promise<void>`
   - `setEqualizer(eq): Promise<void>`
   - `setToggle(name, value): Promise<void>`
   - etc.
2. Update `useDeviceState` hook:
   - On mount: listen for `device-state-updated` event
   - Update React state from event payload
   - Setter functions call Tauri commands instead of local state
3. Add connection flow UI:
   - On app start: auto-discover → if known device found → auto-connect
   - If no device: show "Searching..." → device picker

**Test:** Full flow: app opens → discovers → connects → shows real battery level.

**Demo:** App shows real headphone state. Changing ANC in app changes it on device.

---

## Task E4: Add system tray

**Objective:** Minimize to tray with connection status and quick actions.

**Steps:**
1. Configure Tauri system tray in `tauri.conf.json`
2. Create tray menu:
   - Device name + battery level
   - Quick ANC toggle (NC / Transparency / Normal)
   - "Open Mabel" to restore window
   - "Quit"
3. Update tray icon/tooltip based on connection status
4. Close button minimizes to tray (not quit)

**Reference:** `baseus-desktop/apps/baseus-app/src-tauri/src/tray.rs`

**Test:** Close window → tray icon shows → click "Open" → window restores.

**Demo:** App lives in tray, shows battery in tooltip, quick ANC switching works.

---

## Task E5: Error handling + edge cases

**Objective:** Handle all real-world failure modes gracefully.

**Steps:**
1. Bluetooth adapter not found → show helpful error
2. Device not paired → show pairing instructions
3. Device out of range → show reconnecting state
4. Command timeout → retry once, then show error toast
5. Multiple Soundcore devices → device picker UI
6. App startup when device is off → waiting state with auto-retry
7. LDAC + Dolby mutual exclusion → disable Dolby toggle when LDAC is on (and vice versa)

**Test:** Manually test each failure scenario.

**Demo:** All error states show appropriate UI feedback.

---

## Success Criteria (Phase E complete / MVP done)

- [ ] App discovers and connects to real Space One Pro
- [ ] All 18 features work with real device
- [ ] State updates in real-time (battery, ANC changes from physical button)
- [ ] Commands from app change device settings instantly
- [ ] Reconnection works when device goes out of range and returns
- [ ] System tray with quick controls
- [ ] Error states handled gracefully
- [ ] No crashes or hangs during normal usage
