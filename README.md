# Mabel

> Desktop companion for Soundcore headphones. Premium UI, open protocol, Rust-powered.
>
> *"Because some mysteries are best solved in silence."*

## What is this?

Mabel is a desktop application for controlling Soundcore headphones from Windows (and eventually macOS/Linux). It aims to replicate the mobile Soundcore app experience with a premium dark UI built for desktop.

Named after Mabel Mora from *Only Murders in the Building* — always wearing her headphones, always investigating.

## Features (planned for v0.1)

- Connect via Bluetooth RFCOMM (Windows)
- Battery level display
- ANC modes (Noise Cancelling / Transparency / Normal) with adaptive levels
- 10-band equalizer with presets
- Dolby Audio, LDAC, Side Tone toggles
- Auto Power Off configuration
- Dual Connections management
- Wind Noise Reduction
- Button configuration

## Supported Devices

| Model | Name | Status |
|-------|------|--------|
| A3062 | Soundcore Space One Pro | In Progress |

More devices welcome via community contributions.

## Architecture

```
mabel/
├── crates/
│   ├── mabel-protocol/     # Packet framing + per-model parsers
│   ├── mabel-transport/    # Bluetooth RFCOMM abstraction
│   └── mabel-core/         # Device manager, state machine
├── apps/
│   └── mabel-app/          # Tauri v2 + React frontend
└── docs/
```

Each crate is independently testable. Adding support for a new device means creating a new module in `mabel-protocol` without touching anything else.

## Tech Stack

- **Backend:** Rust (workspace with 3 crates)
- **Frontend:** React + Tailwind CSS + Framer Motion + Recharts
- **Desktop:** Tauri v2
- **Protocol:** Based on [OpenSCQ30](https://github.com/Oppzippy/OpenSCQ30) reverse engineering

## Building

> Coming soon — the project is in early scaffolding phase.

## Contributing

Contributions are welcome! If you own a Soundcore device and want to help:

1. Check the [OpenSCQ30 docs](https://github.com/Oppzippy/OpenSCQ30/blob/master/docs/development.md) on how to capture Bluetooth packets
2. Open an issue with your device model and captured state update packet
3. PRs for new device support, UI improvements, or platform support are appreciated

## Credits

- Protocol knowledge from [OpenSCQ30](https://github.com/Oppzippy/OpenSCQ30) by Oppzippy
- UI inspiration from [baseus-desktop](https://github.com/elaxptr/baseus-desktop) and the official Soundcore mobile app

## License

MIT
