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

### 8) Variable and data type tutorial demo added
- Added new modular file: `src/variaabl_datatype.rs`.
- Implemented reusable APIs:
  - `variable_datatype_features()`
  - `topic_count()`
  - `topic_by_name()`
- Added unit tests for topic count, lookup success/failure, and field completeness.
- Exposed module in `src/lib.rs`.
- Integrated startup logging of variable/data-type tutorial topics in `src/main.rs`.
- Added explanation file: `variable_datatype.md`.
- Validation completed:
  - `cargo test --lib --target x86_64-unknown-linux-gnu` (8 tests passed)
  - `cargo build --release` (ESP32 build passed)

### 9) Variable/data type demo upgraded with real executable examples
- Updated `src/variaabl_datatype.rs` to include concrete demo functions:
  - `demo_immutability()`
  - `demo_mutability()`
  - `demo_strong_typing()`
  - `demo_type_inference()`
  - `demo_option_instead_of_null()`
  - `demo_checked_add_overflow()`
  - `run_variable_datatype_demo()`
- Updated `src/main.rs` to log actual computed demo results (not only rule text).
- Updated `variable_datatype.md` to document executable examples.
- Validation completed:
  - `cargo test --lib --target x86_64-unknown-linux-gnu` (10 tests passed)
  - `cargo build --release` (ESP32 build passed)

### 10) Added tuple initialization + underscore-unused variable examples
- Extended `src/variaabl_datatype.rs` with:
  - `demo_tuple_group_initialization()` using tuple `(u16, u16, bool)`
  - `demo_underscore_unused_variable()` showing `_raw_adc` naming convention
- Expanded feature list to include these topics and updated summary result struct.
- Updated `src/main.rs` to log tuple demo values and underscore-demo output.
- Updated `variable_datatype.md` with dedicated explanation sections for:
  - tuple initialization for grouped variables
  - underscore prefix for intentionally unused variables
- Validation completed:
  - `cargo test --lib --target x86_64-unknown-linux-gnu` (10 tests passed)
  - `cargo build --release` (ESP32 build passed)

### 11) Added byte literal tutorial module and docs
- Added new module: `src/byte_literal.rs` with:
  - topic list API: `byte_literal_topics()`
  - executable demos:
    - `demo_single_byte_literal()`
    - `demo_escaped_newline_literal()`
    - `demo_byte_string_literal()`
    - `demo_byte_string_sum()`
    - `run_byte_literal_demo()`
  - unit tests for byte values and aggregate outputs
- Exposed module in `src/lib.rs` (`pub mod byte_literal;`).
- Updated `src/main.rs` to log byte literal topics and demo outputs at startup.
- Added explanation tutorial: `byte_literal.md`.
- Validation completed:
  - `cargo test --lib --target x86_64-unknown-linux-gnu` (16 tests passed)
  - `cargo build --release` (ESP32 build passed)
