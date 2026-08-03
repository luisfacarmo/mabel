# Mabel

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2-24C8D8.svg)](https://tauri.app/)

> Desktop companion for Soundcore headphones. Premium UI, open protocol, Rust-powered.
>
> *"Because some mysteries are best solved in silence."*

> [!WARNING]
> Mabel is under active development and not yet stable. The Bluetooth protocol layer is being validated — device communication may not work on all firmware versions.

## What is this?

Mabel is a desktop application for controlling Soundcore headphones from Windows (and eventually macOS/Linux). It replicates the mobile Soundcore app experience with a premium dark UI built for desktop.

Named after Mabel Mora from *Only Murders in the Building* — always wearing her headphones, always investigating.

## Current Status

| Component | Status | Notes |
|-----------|--------|-------|
| React UI (5 pages) | ✅ Working | Dark theme, animations, all controls |
| Protocol parser | ✅ Working | 37 tests passing, A3062 state decode |
| Command builders | ✅ Working | 9 outbound commands |
| RFCOMM discovery | ✅ Working | Finds paired Soundcore devices |
| RFCOMM data channel | 🔧 In progress | Connects but wrong service UUID — investigating |
| Real-time control | ⏳ Blocked | Waiting on correct RFCOMM channel |
| System tray | ✅ Working | Minimize to tray, show/hide |
| Settings persistence | ✅ Working | JSON config file |

## Features (v0.1 target)

- [x] Connect via Bluetooth RFCOMM (Windows)
- [x] Battery level display
- [x] ANC modes (Noise Cancelling / Transparency / Normal) with adaptive levels
- [x] 10-band equalizer with presets
- [x] Dolby Audio, LDAC, Side Tone toggles
- [x] Auto Power Off configuration
- [x] Wind Noise Reduction
- [x] Button configuration
- [ ] Dual Connections management
- [ ] Real-time bidirectional communication (in progress)

## Supported Devices

| Model | Name | Status |
|-------|------|--------|
| A3062 | Soundcore Space One Pro | In Progress |

More devices welcome via community contributions.

## Architecture

```
mabel/
├── crates/
│   ├── mabel-protocol/     # Packet framing + A3062 parser (37 tests)
│   ├── mabel-transport/    # Bluetooth RFCOMM abstraction (Windows)
│   └── mabel-core/         # Device manager (planned)
├── apps/
│   └── mabel-app/          # Tauri v2 + React frontend
│       ├── src/            # React UI (5 pages, 15+ components)
│       └── src-tauri/      # Rust backend (device loop, commands, tray)
└── docs/
```

Each crate is independently testable. Adding support for a new device means creating a new module in `mabel-protocol` without touching anything else.

## Tech Stack

- **Backend:** Rust (workspace with 3 crates)
- **Frontend:** React 19 + Tailwind CSS 4 + Framer Motion + Radix UI
- **Desktop:** Tauri v2
- **Transport:** Windows RFCOMM via WinRT (`windows` crate)
- **Protocol:** Based on [OpenSCQ30](https://github.com/Oppzippy/OpenSCQ30) reverse engineering

## Building

### Prerequisites

- [Rust](https://rustup.rs/) (stable, 1.75+)
- [Node.js](https://nodejs.org/) (20+)
- Windows 10/11 with Bluetooth adapter

### Development

```bash
cd apps/mabel-app
npm install
npm run tauri dev
```

### Tests

```bash
cargo test -p mabel-protocol
```

## Contributing

Contributions are welcome! If you own a Soundcore device and want to help:

1. Check the [OpenSCQ30 docs](https://github.com/Oppzippy/OpenSCQ30/blob/master/docs/development.md) on how to capture Bluetooth packets
2. Open an issue with your device model and captured state update packet
3. PRs for new device support, UI improvements, or platform support are appreciated

## Credits

- Protocol knowledge from [OpenSCQ30](https://github.com/Oppzippy/OpenSCQ30) by Oppzippy
- UI inspiration from [baseus-desktop](https://github.com/nicoboss/baseus-desktop) and the official Soundcore mobile app

## License

MIT
