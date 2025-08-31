# Turn-Based Boardgame Engine (Rust)

A Rust-based engine for turn-based board games.  
Supports both local and online multiplayer play.

## Features

- **Turn-based only** (no real-time rollback required)
- **Rust workspace structure**:
  - `engine-core` : Game state machine (deterministic transitions, snapshots, replay)
  - `engine-net`  : WebRTC networking (STUN/TURN fallback, reconnection support)
  - `engine-ui`   : UI abstraction (egui, WebAssembly compatible)
  - `engine-proto`: Network message definitions
  - `signaling-server`: Lightweight signaling server (WebSocket + TURN credentials)
  - `game-logic-othello`: Example game (Othello/Reversi logic)
  - `game-ui-othello`   : Othello UI (egui implementation)
- **Local multiplayer**: Multiple players on the same PC
- **Online play**: Private matches via room codes (future: random matchmaking)
- **Reconnection support**: Resume game after disconnection (snapshot + log replay)
- **Replay system**: Step through matches move by move

## Requirements

- Rust 1.80+
- Native: Linux / macOS / Windows
- Browser: WebAssembly (`wasm32-unknown-unknown`, with [Trunk](https://trunkrs.dev))

## Quick Start

### Native
```bash
cargo run -p host-app-native
