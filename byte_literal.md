# Byte Literal Tutorial in Rust

This tutorial explains Rust byte literals with executable examples from this project.
It follows the same modular/testable style used in other tutorials.

## 1) Module overview

Implementation file: `src/byte_literal.rs`

The module provides:
- topic metadata for learning (`ByteLiteralTopic`)
- runnable demo functions
- aggregate demo summary (`ByteLiteralDemoResults`)
- unit tests

## 2) Byte literal forms

### A) Single-byte literal

```rust
let b = b'A';
```

- Type: `u8`
- Value: `65`
- Useful when you need one protocol/control byte.

### B) Escaped byte literal

```rust
let nl = b'\n';
```

- Type: `u8`
- Value: `10`
- Common for line-based serial protocols.

### C) Byte string literal

```rust
let msg = b"HELLO";
```

- Type: `&[u8; 5]`
- Good for UART/SPI payload constants.

### D) Hex byte literal

```rust
let marker = b'\x7F';
```

- Type: `u8`
- Gives exact byte values used in framed protocols.

## 3) Executable demos included

In `src/byte_literal.rs`:
- `demo_single_byte_literal()`
- `demo_escaped_newline_literal()`
- `demo_byte_string_literal()`
- `demo_byte_string_sum()`
- `run_byte_literal_demo()`

`run_byte_literal_demo()` returns:
- `ascii_a`
- `newline`
- `bytes_len`
- `bytes_first`
- `hello_sum`

## 4) How firmware uses it

`src/main.rs` now:
- logs all byte-literal topics and explanations
- runs `run_byte_literal_demo()`
- prints real demo values over ESP logger before LED blinking loop

## 5) Build and test

### Unit tests (host)

```bash
source "$HOME/.cargo/env"
cargo test --lib --target x86_64-unknown-linux-gnu
```

### Firmware build (ESP32)

```bash
source "$HOME/.cargo/env"
source "$HOME/export-esp.sh"
cargo build --release
```

## 6) Quick comparison with C/C++

- Rust byte literals are explicit and strongly typed (`u8`, `&[u8; N]`).
- This helps avoid accidental char/int confusion in protocol code.
- The compiler catches type mismatches early.
