# ESP32-WROOM Embedded Rust Setup Log

This file records each step performed to install Embedded Rust + ESP32 support, create a hello-world firmware, build it, and export a binary for flashing from a local machine.

## 1) Workspace inspection

Commands run:

```bash
ls -la
cat README.md
```

Result:
- Repository initially contained only a minimal `README.md`.

---

## 2) Install Rust toolchain

Commands run:

```bash
curl https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"
rustup default stable
rustc --version
cargo --version
```

Result:
- Installed Rust via `rustup`.
- Set default stable toolchain.
- Verified compiler and cargo availability.

---

## 3) Install ESP Rust tools

Commands run:

```bash
source "$HOME/.cargo/env"
cargo install espup --locked
cargo install espflash --locked
cargo install ldproxy --locked
```

Result:
- Installed:
  - `espup 0.16.0`
  - `espflash 4.3.0`
  - `ldproxy 0.3.4`

---

## 4) Install ESP32 Xtensa toolchain

Command run:

```bash
source "$HOME/.cargo/env"
espup install -t esp32
```

Result:
- Installed Espressif Xtensa Rust toolchain (`esp`) and GCC toolchain.
- Generated environment file: `~/export-esp.sh`.

Important for new terminals:

```bash
source "$HOME/export-esp.sh"
```

---

## 5) Initialize firmware project and add ESP32 files

Command run:

```bash
source "$HOME/.cargo/env"
cargo init --name embedded-rust --bin .
```

Then these files were created/updated:

- `Cargo.toml`
- `.cargo/config.toml`
- `rust-toolchain.toml`
- `build.rs`
- `src/main.rs`

### Project configuration summary

- Target MCU: **ESP32** (fits ESP32-WROOM modules)
- Target triple: `xtensa-esp32-none-elf`
- Main crates:
  - `esp-hal = 1.0.0`
  - `esp-backtrace = 0.18.1`
  - `esp-println = 0.16.1`
  - `log = 0.4`
- Entry point: `#[main]`
- Hello-world output methods:
  - `esp_println::println!`
  - UART0 text output (115200 baud)

---

## 6) Build (compile) firmware

Command run:

```bash
source "$HOME/.cargo/env"
source "$HOME/export-esp.sh"
cargo build --release
```

Build notes:
- First build attempts exposed missing/compatibility issues.
- Fixed by:
  - Enabling build-std core in `.cargo/config.toml`
  - Updating code to stable `esp-hal 1.0.0` APIs
  - Using `xtensa-esp32-elf-gcc` linker in `.cargo/config.toml`

Final result:
- Build successful.
- Produced ELF:
  - `target/xtensa-esp32-none-elf/release/embedded-rust`

---

## 7) Export single binary for local-machine flashing

Command run:

```bash
source "$HOME/.cargo/env"
source "$HOME/export-esp.sh"
mkdir -p dist
espflash save-image \
  --chip esp32 \
  --ignore-app-descriptor \
  --merge \
  target/xtensa-esp32-none-elf/release/embedded-rust \
  dist/embedded-rust-esp32-merged.bin
```

Result:
- Export successful.
- Flashable merged binary created:
  - `dist/embedded-rust-esp32-merged.bin`

---

## 8) Flash from your local machine (ESP32-WROOM)

Copy the exported binary to your local machine, then flash with one of the following methods.

### Option A: `esptool.py` (common and reliable)

```bash
esptool.py --chip esp32 --port /dev/ttyUSB0 --baud 460800 write_flash -z 0x0 embedded-rust-esp32-merged.bin
```

Notes:
- On Windows, port is usually like `COM3`.
- On Linux/macOS, often `/dev/ttyUSB0` or `/dev/ttyACM0`.

### Option B: Using `espflash` directly with ELF (if project exists on local machine)

```bash
source "$HOME/export-esp.sh"
cargo build --release
espflash flash --monitor --chip esp32 target/xtensa-esp32-none-elf/release/embedded-rust
```

---

## 9) Expected hello-world output

After flashing and opening serial monitor at 115200 baud, expected output includes:

- `Hello, world from ESP32-WROOM!`
- `Hello from UART0 on ESP32-WROOM!`
- repeated heartbeat lines every ~1 second.

---

## 10) Quick rerun commands (in this repo)

```bash
source "$HOME/.cargo/env"
source "$HOME/export-esp.sh"
cargo build --release
espflash flash --monitor --chip esp32 target/xtensa-esp32-none-elf/release/embedded-rust
```
