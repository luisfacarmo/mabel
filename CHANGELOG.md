# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Known Issues
- RFCOMM connects to wrong service UUID (00001108 Headset instead of Soundcore data channel)
- Commands are sent but device does not respond with state updates
- Frontend shows "Offline" until bidirectional channel is resolved

---

## [0.1.0-alpha.2] — 2026-08-02

### Added
- **Device loop** with tokio::select!, auto-reconnect every 5s, command dispatch
- **Tauri IPC bridge** (`src/lib/tauri.ts`) with typed invoke wrappers and event listeners
- **System tray** — minimize to tray, left-click show/hide, right-click menu (Show/Quit)
- **Settings persistence** — JSON file in OS config dir (low_battery_alerts, threshold, minimize_to_tray)
- **Battery alerts** — emit `low-battery` event when battery below configured threshold
- **RFCOMM connect()** — StreamSocket connection with read channel (thread + mpsc) and connection status (watch channel)
- **Sequential service discovery** — try all 6 RFCOMM services until one accepts connection
- **Reactive frontend** — pages read from device-state events, no more local useState for device values
- **Commands wired** — ANC, EQ, LDAC, Dolby, Sidetone, Auto Power Off all invoke real Tauri commands
- **Dark theme fix** — globals.css converted from light to dark theme (#0f0f0f bg, #1a1a1a surface)
- **Tauri capabilities** — window controls (close, minimize, maximize, drag) permissions added
- **Cross-project audit** (`docs/cross-project-audit.md`) — reuse analysis across baseus-desktop, OpenSCQ30, SoundcoreManager

### Changed
- `commands.rs` rewritten — 7 commands that dispatch to device loop via mpsc channel
- `lib.rs` rewritten — tracing init, device loop spawn, tray setup
- `useConnection` hook — listens to Tauri `connection-state` events (mock fallback in browser)
- `useDeviceState` hook — listens to Tauri `device-state` events (mock fallback in browser)
- `useCommands` hook — invokes real Tauri commands (mock fallback in browser)
- All pages (Home, ANC, Sound, Settings) — removed local useState, read from provider reactively

### Fixed
- Window controls not working (missing capabilities in `default.json`)
- ModeCard icon using light color (`#f3f4f6`) on dark theme — changed to `bg-surface-hover`
- EQ dots using `bg-white` — changed to `bg-surface`

---

## [0.1.0-alpha.1] — 2026-08-02

### Added
- **Phase A complete** — Tauri v2 + React frontend with 5 pages
  - HomePage with device hero, battery bar, ANC mode cards, quick toggles
  - AncPage with mode selector, NC level picker, wind noise toggle
  - SoundPage with 10-band EQ visualizer and 6 preset chips
  - ControlsPage with Radix Select for button double-press action
  - SettingsPage with toggle rows for audio, connections, power, device info
- **Layout** — sidebar navigation (5 items), animated outlet with Framer Motion page transitions
- **Custom title bar** — borderless window with drag region, minimize/maximize/close buttons
- **Design system** — Tailwind CSS 4 dark theme, CSS variables, Radix UI primitives
- **Mock data layer** — 3 independent hooks (useConnection, useDeviceState, useCommands)
- **Headset image** — real product photo in Home page hero
- **Phase C complete** — `mabel-protocol` crate (37 tests)
  - Packet framing with nom parser (Direction, Packet, checksum validation)
  - A3062 state struct with all sub-types (Battery, SoundModes, EQ, Toggles, etc.)
  - State parser from raw bytes (verified against OpenSCQ30 test vector)
  - 9 command builders (request_state, set_sound_modes, set_equalizer, etc.)
  - PacketStream for fragmented input handling
- **Phase B** — `mabel-transport` crate
  - RfcommTransport + RfcommConnection traits
  - MockTransport with rx_queue + tx_log for testing
  - Windows RFCOMM discovery via WinRT DeviceInformation API

### Changed
- `mabel-protocol` Cargo.toml — added nom v8, serde_json dependencies
- `mabel-transport` Cargo.toml — windows crate features for RFCOMM

---

## [0.0.1] — 2026-08-02

### Added
- Initial workspace scaffold (Cargo.toml workspace with 3 crates + 1 app)
- Empty crate stubs: mabel-protocol, mabel-transport, mabel-core
- Project documentation: implementation plan (5 phases), naming rationale
- Reference materials: manual screenshots, visual notes
- MIT license

[Unreleased]: https://github.com/luisfacarmo/mabel/compare/v0.1.0-alpha.2...HEAD
[0.1.0-alpha.2]: https://github.com/luisfacarmo/mabel/compare/v0.1.0-alpha.1...v0.1.0-alpha.2
[0.1.0-alpha.1]: https://github.com/luisfacarmo/mabel/compare/v0.0.1...v0.1.0-alpha.1
[0.0.1]: https://github.com/luisfacarmo/mabel/releases/tag/v0.0.1
