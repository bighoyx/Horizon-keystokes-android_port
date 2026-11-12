# Custom Keystrokes for Horizon (WASM / .hmod)

WebAssembly (.hmod) keystrokes HUD overlay built for Horizon API 0.1 (compatible with Horizon Android 0.7.8).

## Features
- Displays W, A, S, D, Space, SFT (Left Shift), LMB, and RMB.
- Smooth 0.2s color fadeout animation on key release.
- Rebindable keys in Options > Controls.

## Build from source
1. cargo build --release --target wasm32-unknown-unknown
2. cargo run --offline --bin pack
