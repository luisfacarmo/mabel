# Mabel — Implementation Plan (Overview)

## Goal

Desktop companion app for Soundcore Space One Pro (A3062) headphones on Windows.
Tauri v2 + React frontend, Rust multi-crate backend.

## Execution Order (adjusted for frontend-first)

The original plan was backend → frontend. We're flipping it:
**Frontend first (with mock data) → Backend (transport + protocol) → Integration.**

This lets you see and iterate on the UI before the Bluetooth layer is ready.

```
Phase A: Tauri App Shell + React Frontend (mock data)
Phase B: RFCOMM Transport Layer (Windows)
Phase C: Soundcore Protocol (packet framing + A3062 parser)
Phase D: Core Layer (device manager + state machine)
Phase E: Integration (wire real backend into frontend)
```

## Dependency Graph

```
Phase A (frontend, standalone with mocks)
    ↓ (integration at Phase E)
Phase B (transport) ──┐
                      ├──► Phase D (core) ──► Phase E (wire everything)
Phase C (protocol) ──┘
```

## Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Frontend framework | React 19 + Vite | User choice |
| Styling | Tailwind CSS 4 + Framer Motion | Modern, fast iteration |
| Package manager | npm | User choice |
| Charts/EQ | Recharts or custom SVG | Interactive 10-band EQ |
| Desktop shell | Tauri v2 | Lightweight, Rust backend |
| Transport | `windows` crate (WinRT) | Proven by OpenSCQ30 |
| Protocol parser | nom (parser combinators) | Same as OpenSCQ30, battle-tested |
| Async runtime | tokio | Standard for Rust async |

## File Structure (target)

```
mabel/
├── apps/
│   └── mabel-app/
│       ├── src/                    # React frontend
│       │   ├── components/
│       │   ├── hooks/
│       │   ├── lib/
│       │   ├── pages/
│       │   ├── assets/
│       │   ├── App.tsx
│       │   └── main.tsx
│       ├── src-tauri/
│       │   └── src/
│       │       ├── main.rs
│       │       ├── lib.rs
│       │       ├── commands.rs
│       │       └── state.rs
│       ├── index.html
│       ├── package.json
│       ├── vite.config.ts
│       └── tailwind.config.ts
├── crates/
│   ├── mabel-protocol/             # Packet framing + A3062 parser
│   ├── mabel-transport/            # RFCOMM Windows backend
│   └── mabel-core/                 # Device manager, state machine
└── docs/
    ├── plan-overview.md            # This file
    ├── plan-phase-a.md             # Frontend + Tauri shell
    ├── plan-phase-b.md             # Transport
    ├── plan-phase-c.md             # Protocol
    ├── plan-phase-d.md             # Core
    ├── plan-phase-e.md             # Integration
    └── references/                 # Manual, screenshots
```

## MVP Feature Set (18 features)

1. Battery level display
2. ANC mode selector (NC / Transparency / Normal)
3. Adaptive Noise Cancelling level
4. Custom Noise Cancelling level
5. Custom Transparency level
6. Wind Noise Reduction toggle
7. ANC mode cycle configuration
8. 10-band Equalizer with presets
9. Button configuration (double press action)
10. Dolby Audio toggle
11. LDAC toggle
12. Sidetone toggle
13. Voice Prompt toggle
14. Low Battery Prompt toggle
15. Auto Power Off duration
16. Limit High Volume (toggle + dB)
17. Dual Connections (toggle + device list)
18. Device info (firmware + serial)
