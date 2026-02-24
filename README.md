# Embedded Rust ESP32 Tutorials

Embedded Rust demo project for ESP32-WROOM DevKit V1.

This repository contains:
- firmware that blinks onboard LED on GPIO2,
- startup tutorial logs (memory safety, variable/data types, byte literals, char, static/global variables),
- reproducible build/export flow for ESP32 4MB flash.

## Target Hardware

- Board: ESP32-WROOM DevKit V1
- Onboard LED pin: GPIO2

## Repository Layout

- `src/main.rs` → firmware entrypoint (tutorial logs + LED blink)
- `src/memory_safety.rs` → memory safety tutorial module
- `src/variaabl_datatype.rs` → variable/data type tutorial module
- `src/byte_literal.rs` → byte literal tutorial module
- `src/char_tutorial.rs` → char tutorial module
- `dist/` → exported binaries for flashing

## Prerequisites

Install Rust + ESP toolchain (already done in this workspace setup):

```bash
source "$HOME/.cargo/env"
source "$HOME/export-esp.sh"
```

## Build Firmware

```bash
source "$HOME/.cargo/env"
source "$HOME/export-esp.sh"
cargo build --release
```

Release ELF output:
- `target/xtensa-esp32-none-elf/release/embedded-rust`

## Export Flashable Binary (4MB-safe)

```bash
source "$HOME/.cargo/env"
mkdir -p dist
TS=$(date +%Y%m%d-%H%M%S)
espflash save-image --chip esp32 --merge --skip-padding --ignore-app-descriptor --flash-size 4mb \
	target/xtensa-esp32-none-elf/release/embedded-rust dist/embedded-rust-esp32-4mb-${TS}.bin
```

## Flash on Target

```bash
espflash write-bin 0x0 dist/embedded-rust-esp32-4mb-YYYYMMDD-HHMMSS.bin
```

## Tutorials

- [memory_safety.md](memory_safety.md)
- [variable_datatype.md](variable_datatype.md)
- [byte_literal.md](byte_literal.md)
- [char.md](char.md)
- [staticvariable.md](staticvariable.md)

## Logs / Execution History

- [SETUP_AND_BUILD_STEPS.md](SETUP_AND_BUILD_STEPS.md)
- [ACTIVITY_HISTORY.md](ACTIVITY_HISTORY.md)