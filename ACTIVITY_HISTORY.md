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

### 12) Built and exported new timestamped binary
- Rebuilt firmware with:
  - `cargo build --release`
- Exported compact 4MB-safe timestamped image into `dist` with:
  - `espflash save-image --chip esp32 --merge --skip-padding --ignore-app-descriptor --flash-size 4mb ...`
- New artifact created:
  - `dist/embedded-rust-esp32-4mb-20260223-104334.bin`
- Artifact details:
  - size: `145K`
  - sha256: `f5e10fe329111b37b572c18feb19f9c437b3e1f24025181045aa3629507e613a`

### 13) Added char tutorial + built/exported new binary
- Added new tutorial module: `src/char_tutorial.rs` with:
  - topic API: `char_topics()`
  - executable demos:
    - `demo_ascii_char()`
    - `demo_unicode_char()`
    - `demo_unicode_utf8_len()`
    - `demo_ascii_uppercase()`
    - `demo_digit_to_number()`
    - `run_char_demo()`
  - unit tests for all demos and summary output
- Exposed module in `src/lib.rs` (`pub mod char_tutorial;`).
- Updated `src/main.rs` to print char tutorial topics and demo outputs at startup.
- Added tutorial documentation file: `char.md`.
- Validation completed:
  - `cargo test --lib --target x86_64-unknown-linux-gnu` (23 tests passed)
  - `cargo build --release` (ESP32 build passed)
- Exported new timestamped binary:
  - `dist/embedded-rust-esp32-4mb-20260223-104636.bin`
- Artifact details:
  - size: `146K`
  - sha256: `774934a3231071e05c52a681f338cf5141659aa549c815195937d5db24849b0c`

### 14) Expanded README with project explanation and tutorial links
- Updated `README.md` from minimal placeholder to full documentation.
- Added:
  - project purpose and target hardware details,
  - repository layout summary,
  - build command,
  - 4MB-safe binary export command,
  - flash command,
  - links to tutorial markdown files:
    - `memory_safety.md`
    - `variable_datatype.md`
    - `byte_literal.md`
    - `char.md`
  - links to setup/history docs:
    - `SETUP_AND_BUILD_STEPS.md`
    - `ACTIVITY_HISTORY.md`

### 15) Added variable scope/static tutorial + exported new binary
- Added new tutorial module: `src/staticvariable.rs` covering:
  - local variable demo
  - global/static variable demo (atomic counter)
  - naming convention demos (`snake_case`, `UPPER_CASE`, `CamelCase`)
  - aggregate demo API: `run_staticvariable_demo()`
  - unit tests for all APIs
- Exposed module in `src/lib.rs` (`pub mod staticvariable;`).
- Updated `src/main.rs` to print static/scope topics and demo outputs in logger.
- Added tutorial documentation file: `staticvariable.md`.
- Firmware build completed and new timestamped binary exported:
  - `dist/embedded-rust-esp32-4mb-20260224-063346.bin`
- Artifact details:
  - size: `147K`
  - sha256: `4919a304a492209e8f882828758d7d258d51063228fa915cab360f6e5bb4ff93`
- Note:
  - Host test run hit a toolchain/config conflict (`duplicate lang item core`) in current environment; ESP32 release build/export still succeeded.

### 16) Expanded static variable tutorial coverage in docs
- Updated `staticvariable.md` to explicitly include requested lecture topics:
  - global vs local variables distinction,
  - `let` and local type inference,
  - `static` requirement for global variables,
  - uppercase naming for global/static variables,
  - immutable vs mutable static variables,
  - `unsafe` requirement and data-race caution for `static mut`,
  - memory placement/safety notes (ROM/flash vs RAM),
  - `'static` lifetime explanation.

### 17) Added line-by-line explanatory comments in staticvariable module
- Updated `src/staticvariable.rs` with tutorial comments before:
  - key structs and fields,
  - global/static variables,
  - each demo function,
  - each unit test.
- Goal was readability for beginners directly in source code.
- Validation completed:
  - `cargo build --release` (ESP32 build passed)

### 18) Added readability comments across all Rust source files
- Added explanatory comments to remaining Rust files:
  - `src/lib.rs`
  - `build.rs`
  - `src/main.rs`
  - `src/memory_safety.rs`
  - `src/variaabl_datatype.rs`
  - `src/byte_literal.rs`
  - `src/char_tutorial.rs`
- Existing comments in `src/staticvariable.rs` were kept and aligned with tutorial style.
- Validation completed:
  - `cargo build --release` (ESP32 build passed)

### 19) Added fresher-focused explanation directly in main source
- Expanded `src/main.rs` with beginner-friendly training comments that explain:
  - `#![no_std]` and `#![no_main]` for bare-metal firmware,
  - embedded entrypoint model (`#[main] fn main() -> !`),
  - C-to-Rust mapping for superloop firmware,
  - app descriptor purpose,
  - GPIO2 LED control and ON/OFF loop behavior.

### 20) Added explicit explanations for requested Rust syntax items
- Added direct explanation comments for:
  - `#![cfg_attr(not(test), no_std)]` in `src/lib.rs`
  - `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` in tutorial modules
  - `use core::sync::atomic::{AtomicU32, Ordering};` in `src/staticvariable.rs`
- Validation completed:
  - `cargo build --release` (ESP32 build passed)
