# ESP32-WROOM Embedded Rust Setup and Execution Log

This document records everything completed in this repository to set up Embedded Rust for ESP32-WROOM DevKit V1, implement firmware, build, and export flashable binaries.

## 1) Initial repository state

Initial workspace content:

- `README.md` only

Actions taken:

- Inspected repository structure.
- Confirmed no Rust project/toolchain was preconfigured in the container.

---

## 2) Rust installation and baseline toolchain setup

Commands executed:

```bash
curl https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"
rustup default stable
rustc --version
cargo --version
rustup --version
```

Outcome:

- Rust installed via `rustup`.
- Stable toolchain set as default (`rustc 1.93.1` in this environment).

---

## 3) ESP Rust tooling installation

Commands executed:

```bash
source "$HOME/.cargo/env"
cargo install espup --locked
cargo install espflash --locked
cargo install ldproxy --locked
cargo install cargo-generate --locked
```

Outcome:

- Installed `espup`, `espflash`, `ldproxy`, and `cargo-generate`.
- Verified `espup --version` and `espflash --version` worked.

---

## 4) ESP32 Xtensa toolchain setup

Commands executed:

```bash
source "$HOME/.cargo/env"
espup install -t esp32
source "$HOME/export-esp.sh"
rustup toolchain list
```

Outcome:

- Installed Espressif Xtensa Rust toolchain (`esp`) and required GCC tools.
- Export script generated at `~/export-esp.sh`.
- Confirmed both `stable` and `esp` toolchains present.

Important for every new terminal:

```bash
source "$HOME/.cargo/env"
source "$HOME/export-esp.sh"
```

---

## 5) Project creation and firmware base files

Command executed:

```bash
source "$HOME/.cargo/env"
cargo init --name embedded-rust --bin .
```

Files created/updated for firmware build:

- `Cargo.toml`
- `Cargo.lock`
- `.cargo/config.toml`
- `rust-toolchain.toml`
- `build.rs`
- `src/main.rs`

Configuration summary:

- Target MCU: ESP32 (compatible with ESP32-WROOM DevKit V1)
- Target triple: `xtensa-esp32-none-elf`
- Primary dependencies:
  - `esp-hal = 1.0.0`
  - `esp-backtrace = 0.18.1`
  - `esp-println = 0.16.1`
  - `log = 0.4`

---

## 6) Build stabilization and successful compile

Build command used:

```bash
source "$HOME/.cargo/env"
source "$HOME/export-esp.sh"
cargo build --release
```

Adjustments made during iteration:

- Added/updated target build configuration for Xtensa (`xtensa-esp32-none-elf`).
- Fixed linker/toolchain setup to produce successful release builds.
- Verified final release ELF generated at:

`target/xtensa-esp32-none-elf/release/embedded-rust`

---

## 7) Firmware behavior changes completed

### 7.1 Hello-world baseline

Implemented initial firmware and validated compile.

### 7.2 LED blink firmware for ESP32-WROOM DevKit V1

Updated `src/main.rs` to use onboard LED pin `GPIO2`:

- LED ON for 1 second
- LED OFF for 1 second
- repeat forever

### 7.3 ESP logging for LED state

Added logger init and status logs:

- `LED status: ON`
- `LED status: OFF`

### 7.4 Forced newline output in logs

Updated logs to include explicit CRLF (`\r\n`) so each print appears on a new line in serial monitors.

---

## 8) Binary export attempts and final working approach

### Attempt A: raw `objcopy` binary

Command used:

```bash
xtensa-esp32-elf-objcopy -O binary target/xtensa-esp32-none-elf/release/embedded-rust dist/embedded-rust-esp32.bin
```

Result:

- Produced large raw binary (`~13MB`) due to sparse memory layout.
- Flashing at `0x0` failed for 4MB ESP32 with:
  - “will not fit in 4194304 bytes of flash”.

### Attempt B: `espflash save-image` default options

Result:

- Failed because app descriptor check was enforced for this app format.

### Final working export command (recommended)

```bash
source "$HOME/.cargo/env"
mkdir -p dist
espflash save-image \
  --chip esp32 \
  --merge \
  --skip-padding \
  --ignore-app-descriptor \
  --flash-size 4mb \
  target/xtensa-esp32-none-elf/release/embedded-rust \
  dist/embedded-rust-esp32-4mb.bin
```

Outcome:

- Created compact merged binary for ESP32-WROOM 4MB flash:
  - `dist/embedded-rust-esp32-4mb.bin` (~142KB)

---

## 9) Timestamped binary export

Command executed:

```bash
source "$HOME/.cargo/env"
mkdir -p dist
TS=$(date +%Y%m%d-%H%M%S)
OUT="dist/embedded-rust-esp32-4mb-${TS}.bin"
espflash save-image --chip esp32 --merge --skip-padding --ignore-app-descriptor --flash-size 4mb \
  target/xtensa-esp32-none-elf/release/embedded-rust "$OUT"
```

Generated file:

- `dist/embedded-rust-esp32-4mb-20260223-084747.bin`

---

## 10) Current firmware source status

`src/main.rs` currently:

- initializes ESP HAL
- configures `GPIO2` as output
- blinks LED at 1 second ON / 1 second OFF
- logs LED state with explicit `\r\n` newlines

---

## 11) Build and export commands to repeat now

### Build

```bash
source "$HOME/.cargo/env"
source "$HOME/export-esp.sh"
cargo build --release
```

### Export compact 4MB binary

```bash
source "$HOME/.cargo/env"
mkdir -p dist
espflash save-image --chip esp32 --merge --skip-padding --ignore-app-descriptor --flash-size 4mb \
  target/xtensa-esp32-none-elf/release/embedded-rust dist/embedded-rust-esp32-4mb.bin
```

### Export timestamped binary

```bash
source "$HOME/.cargo/env"
mkdir -p dist
TS=$(date +%Y%m%d-%H%M%S)
espflash save-image --chip esp32 --merge --skip-padding --ignore-app-descriptor --flash-size 4mb \
  target/xtensa-esp32-none-elf/release/embedded-rust dist/embedded-rust-esp32-4mb-${TS}.bin
```

---

## 12) Local flashing reference (ESP32-WROOM DevKit V1)

Use compact image at address `0x0`:

```bash
espflash write-bin 0x0 dist/embedded-rust-esp32-4mb.bin
```

If using a timestamped file:

```bash
espflash write-bin 0x0 dist/embedded-rust-esp32-4mb-YYYYMMDD-HHMMSS.bin
```
