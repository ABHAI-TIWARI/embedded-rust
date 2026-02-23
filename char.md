# Char Tutorial in Rust

This tutorial explains Rust `char` with executable examples in this project.
It follows the same modular and testable style as other tutorials.

## 1) What was added

Code module: `src/char_tutorial.rs`

Contains:
- `CharTopic` for tutorial entries
- `CharDemoResults` for aggregated runnable output
- APIs:
  - `char_topics()`
  - `demo_ascii_char()`
  - `demo_unicode_char()`
  - `demo_unicode_utf8_len()`
  - `demo_ascii_uppercase()`
  - `demo_digit_to_number()`
  - `run_char_demo()`
- Unit tests for all demo outputs

## 2) Key `char` concepts

### A) Character literal

```rust
let c: char = 'A';
```

- `char` represents a Unicode scalar value.
- This is different from C/C++ where `char` is usually 1 byte.

### B) Unicode character literal

```rust
let heart: char = '❤';
```

- Rust supports Unicode characters directly.

### C) UTF-8 byte length

```rust
let n = '❤'.len_utf8();
```

- `char` can use 1 to 4 bytes in UTF-8.
- `'❤'` takes 3 bytes in UTF-8.

### D) ASCII transformation

```rust
let up = 'a'.to_ascii_uppercase();
```

- Useful for protocol parsing and command normalization.

### E) Digit conversion

```rust
let value = '7'.to_digit(10);
```

- Returns `Option<u32>`.
- Safe conversion pattern (handles invalid chars gracefully).

## 3) How firmware uses this

`src/main.rs` now:
- prints char tutorial topics and explanation
- runs `run_char_demo()`
- logs real results before entering LED blink loop

## 4) Build and test

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

## 5) C/C++ comparison quick note

- Rust `char` is Unicode-aware and not equivalent to a C 1-byte `char`.
- For raw bytes/protocol frames, use byte literals (`b'A'`, `b"OK"`).
- Use `char` for character semantics; use `u8`/byte arrays for raw binary data.
