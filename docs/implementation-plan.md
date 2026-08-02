# Mabel — Implementation Plan

## Overview

This plan breaks the MVP into 5 phases. Each phase has a clear deliverable that can be tested independently. Complete them in order — each phase builds on the previous.

---

## Phase 1: Transport Layer
**Goal:** Connect to the Space One Pro via Bluetooth RFCOMM on Windows.

| # | Task | Reference |
|---|------|-----------|
| 1.1 | Define `BluetoothTransport` trait (send/recv/connect) | `baseus-desktop/crates/baseus-transport/src/lib.rs` |
| 1.2 | Implement `WindowsRfcommTransport` using WinRT StreamSocket | `OpenSCQ30/lib/src/connection_backend/windows/rfcomm.rs` |
| 1.3 | Implement device discovery (find paired Soundcore devices) | `OpenSCQ30/lib/src/connection_backend/windows/rfcomm.rs` → `devices()` |
| 1.4 | Write integration test: connect to real device, send ping | Manual test with headphone on |

**Success criteria:** `cargo test -p mabel-transport` connects to the Space One Pro and reads raw bytes back.

---

## Phase 2: Protocol Layer
**Goal:** Parse the A3062 state update packet into a typed Rust struct.

| # | Task | Reference |
|---|------|-----------|
| 2.1 | Implement Soundcore packet framing (header, command, body, checksum) | `OpenSCQ30/lib/src/devices/soundcore/common/packet/` |
| 2.2 | Create `A3062StateUpdate` struct with all fields | `OpenSCQ30/lib/src/devices/soundcore/a3062/packets/inbound.rs` |
| 2.3 | Implement parser for A3062 state (battery, ANC, EQ, toggles) | `OpenSCQ30/lib/src/devices/soundcore/a3062/state.rs` |
| 2.4 | Implement command builders (set_anc, set_eq, set_toggle) | `OpenSCQ30/lib/src/devices/soundcore/a3062/packets/outbound.rs` |
| 2.5 | Write unit tests with known packet bytes from OpenSCQ30 test | `OpenSCQ30/lib/src/devices/soundcore/a3062.rs` → test at bottom |

**Success criteria:** `cargo test -p mabel-protocol` parses the test vector from OpenSCQ30 issue #194 correctly.

---

## Phase 3: Core Layer
**Goal:** Device manager that scans, connects, holds state, and dispatches commands.

| # | Task | Reference |
|---|------|-----------|
| 3.1 | Create `DeviceManager` (scan, connect, disconnect, list) | `baseus-desktop/apps/baseus-app/src-tauri/src/device.rs` |
| 3.2 | Implement state machine (request state → parse → store → notify) | `OpenSCQ30/lib/src/devices/soundcore/a3062.rs` → `soundcore_device!` macro logic |
| 3.3 | Implement command dispatch (set_anc, set_eq → serialize → send) | `baseus-desktop/apps/baseus-app/src-tauri/src/commands.rs` |
| 3.4 | Add reconnection logic | `baseus-desktop` device.rs reconnect pattern |
| 3.5 | Integration test: connect → read state → change ANC → verify | Manual with device |

**Success criteria:** A CLI binary (`cargo run -p mabel-core --example cli`) connects, prints battery/ANC state, and changes ANC mode.

---

## Phase 4: Tauri App Shell
**Goal:** Tauri v2 window that connects to the device and exposes IPC commands.

| # | Task | Reference |
|---|------|-----------|
| 4.1 | Scaffold Tauri v2 app (`apps/mabel-app/`) with `create-tauri-app` | `baseus-desktop/apps/baseus-app/src-tauri/` |
| 4.2 | Add `#[tauri::command]` for: connect, get_state, set_anc, set_eq | `baseus-desktop/apps/baseus-app/src-tauri/src/commands.rs` |
| 4.3 | Emit Tauri events for state updates (battery, ANC changes) | `baseus-desktop/apps/baseus-app/src-tauri/src/device.rs` |
| 4.4 | Add system tray with connection status | `baseus-desktop` tray pattern |
| 4.5 | Verify: `pnpm tauri dev` opens window, connects, shows raw state | Manual test |

**Success criteria:** App window opens, connects to headphone, displays JSON state in dev console.

---

## Phase 5: React Frontend
**Goal:** Premium UI matching the mockup (dark theme, animations, EQ visualizer).

| # | Task | Reference |
|---|------|-----------|
| 5.1 | Setup React + Vite + Tailwind + Framer Motion | `baseus-desktop/apps/baseus-app/package.json` |
| 5.2 | Build layout: sidebar navigation + content area | `baseus-desktop/apps/baseus-app/src/App.tsx` |
| 5.3 | Home tab: battery ring with animation + session timer | `baseus-desktop/apps/baseus-app/src/components/HomeTab.tsx` |
| 5.4 | ANC tab: mode selector + strength slider | `baseus-desktop/apps/baseus-app/src/components/AncTab.tsx` |
| 5.5 | EQ tab: preset chips + 10-band interactive visualizer | Custom (Recharts + drag interaction) |
| 5.6 | Settings tab: toggles grid (Dolby, LDAC, Side Tone, etc.) | Mockup `mockup-soundcore-desktop.html` |
| 5.7 | Wire up IPC: connect Tauri commands to React state | `baseus-desktop/apps/baseus-app/src/lib/tauri.ts` |
| 5.8 | Polish: transitions, loading states, error handling | — |

**Success criteria:** Full app matching the mockup, all 16 MVP features functional with real device.

---

## Dependency Graph

```
Phase 1 (transport) ──┐
                      ├──► Phase 3 (core) ──► Phase 4 (tauri) ──► Phase 5 (frontend)
Phase 2 (protocol) ──┘
```

Phases 1 and 2 can be developed in parallel. Phase 3 needs both. Phase 4 needs 3. Phase 5 needs 4.

---

## Estimated Timeline (hobby pace, evenings/weekends)

| Phase | Estimate |
|-------|----------|
| Phase 1 | 2-3 sessions |
| Phase 2 | 2-3 sessions |
| Phase 3 | 2 sessions |
| Phase 4 | 1-2 sessions |
| Phase 5 | 3-5 sessions |
| **Total** | **~12 sessions** |

---

## Key Files to Study Before Starting

| What | Where |
|------|-------|
| RFCOMM Windows connection | `OpenSCQ30/lib/src/connection_backend/windows/rfcomm.rs` |
| A3062 device implementation | `OpenSCQ30/lib/src/devices/soundcore/a3062.rs` |
| A3062 state struct | `OpenSCQ30/lib/src/devices/soundcore/a3062/state.rs` |
| A3062 packet parser | `OpenSCQ30/lib/src/devices/soundcore/a3062/packets/` |
| A3062 structures (sound modes) | `OpenSCQ30/lib/src/devices/soundcore/a3062/structures.rs` |
| Tauri command pattern | `baseus-desktop/apps/baseus-app/src-tauri/src/commands.rs` |
| Frontend IPC layer | `baseus-desktop/apps/baseus-app/src/lib/tauri.ts` |
| UI components reference | `baseus-desktop/apps/baseus-app/src/components/` |
