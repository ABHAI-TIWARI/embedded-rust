# Activity History

This file is a running log of work performed in this repository.
I will append new entries here as tasks are completed.

## 2026-02-23

### 1) Environment and toolchain setup
- Initialized Rust with `rustup` and set default stable toolchain.
- Installed embedded tooling: `espup`, `espflash`, `ldproxy`, `cargo-generate`.
- Installed ESP32 Xtensa toolchain via `espup install -t esp32`.
- Verified ESP environment loading through `~/export-esp.sh`.

### 2) Project initialization and configuration
- Initialized crate in workspace root using `cargo init --bin`.
- Configured project for ESP32 target (`xtensa-esp32-none-elf`).
- Added embedded dependencies and linker/runner target configuration.

### 3) Firmware implementation iterations
- Implemented initial hello-world firmware.
- Updated firmware for ESP32-WROOM DevKit V1 onboard LED blink (GPIO2, 1s ON/1s OFF).
- Added LED state logging (`ON` / `OFF`) with explicit newline handling (`\r\n`).
- Integrated modular memory safety tutorial output at startup.

### 4) Modular + testable memory safety tutorial
- Added `src/lib.rs` and `src/memory_safety.rs`.
- Implemented reusable API:
  - `memory_safety_features()`
  - `lesson_count()`
  - `feature_by_title()`
- Added unit tests for module behavior and content completeness.
- Scoped ESP-only dependencies in `Cargo.toml` to Xtensa target so host tests run cleanly.

### 5) Documentation updates
- Expanded and updated `SETUP_AND_BUILD_STEPS.md` with complete setup/build/export history.
- Added `memory_safety.md` with tutorial explanation (Rust vs C/C++, modularity, testing approach).

### 6) Build and binary export history
- Repeatedly validated firmware build with `cargo build --release`.
- Exported compact merged binary for 4MB flash:
  - `dist/embedded-rust-esp32-4mb.bin`
- Exported timestamped binary for target test:
  - `dist/embedded-rust-esp32-4mb-20260223-093003.bin`
- Confirmed raw `objcopy` binary is not suitable for direct 4MB flash at `0x0`.

### 7) Current request handled
- Created this dedicated running history file: `ACTIVITY_HISTORY.md`.
- Committed to maintaining this file for future actions.
