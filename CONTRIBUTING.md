# Contributing

Contributions are welcome! Here's how to get started.

## How to contribute

1. Fork this repository
2. Create a branch for your change (`git checkout -b feat/my-feature`)
3. Make your changes
4. Test locally (see below)
5. Commit with a clear message (`feat: add X` or `fix: resolve Y`)
6. Open a Pull Request

## Local setup

### Requirements
- Rust 1.75+ (stable)
- Node 20+ (for Tauri frontend)
- Tauri CLI 2.x (`cargo install tauri-cli --version ^2`)
- Bluetooth adapter (for device testing)
- Linux: `libbluetooth-dev`, `libdbus-1-dev`, `libwebkit2gtk-4.1-dev`

### Install
```bash
# Backend (Rust)
cargo build

# Frontend
cd apps/mabel-app
npm install
```

### Run (development)
```bash
cd apps/mabel-app
cargo tauri dev
```

### Run checks
```bash
# Rust
cargo clippy --workspace
cargo test --workspace

# Frontend lint
cd apps/mabel-app
npm run lint
```

## Commit style

We follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` — new feature
- `fix:` — bug fix
- `docs:` — documentation only
- `chore:` — maintenance, deps, CI
- `refactor:` — code restructuring without behavior change

## What we accept

- Bug fixes with evidence (logs, packet captures)
- Protocol reverse-engineering contributions (documented)
- New device model support
- UI/UX improvements
- Documentation and README improvements

## What we don't accept

- Breaking changes without prior discussion
- Proprietary protocol leaks or legally questionable RE methods
- PRs that mix unrelated changes
- Direct commits to `master` — always use a PR

## Architecture notes

- **Workspace crates**:
  - `mabel-protocol` — Soundcore packet encoding/decoding
  - `mabel-transport` — RFCOMM Bluetooth connection management
  - `mabel-core` — Device state machine, command dispatch
- **Frontend**: Tauri 2.x + React + TypeScript
- **IPC**: Tauri commands + events (typed wrappers in `src/lib/tauri.ts`)

## Questions?

Open an Issue. We'll respond as soon as possible.
