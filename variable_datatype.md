# Variable and Data Types in Rust (vs C/C++)

This tutorial explains the variable/data-type demo added to this project.
It follows the same style as the memory safety tutorial: modular code, testable functions, and startup logging from firmware `main`.

---

## 1) What was added

### Code module
- `src/variaabl_datatype.rs`
  - Defines `VariableDatatypeFeature` model.
  - Defines `VariableDatatypeDemoResults` summary model.
  - Stores a curated list of variable/data-type topics.
  - Exposes reusable APIs and executable examples:
    - `variable_datatype_features()`
    - `topic_count()`
    - `topic_by_name()`
    - `demo_immutability()`
    - `demo_mutability()`
    - `demo_strong_typing()`
    - `demo_type_inference()`
    - `demo_option_instead_of_null()`
    - `demo_checked_add_overflow()`
    - `demo_tuple_group_initialization()`
    - `demo_underscore_unused_variable()`
    - `run_variable_datatype_demo()`
  - Includes unit tests.

### Library exposure
- `src/lib.rs`
  - Exports module using:
  - `pub mod variaabl_datatype;`

### Firmware integration
- `src/main.rs`
  - Calls `variable_datatype_features()`.
  - Logs each topic in tutorial format, similar to memory demo.

---

## 2) Why this design is modular

The tutorial content is separated from board logic.

- `main.rs` handles runtime behavior (logger + LED loop).
- `variaabl_datatype.rs` handles educational content and lookup APIs.

This makes it easy to:
- update tutorial topics without touching firmware flow,
- reuse content in docs/CLI/tests,
- keep code clean and maintainable.

---

## 3) Why this design is testable

The module uses deterministic pure functions.

- `topic_count()` verifies expected dataset size.
- `topic_by_name()` verifies lookups.
- full-record assertions ensure no empty fields.

Included tests validate:
- expected topic count,
- successful lookup of a known topic,
- `None` for unknown topic,
- non-empty strings for all topic fields.

---

## 4) Executable examples now included

The module now contains runnable demo functions, not only printed rule text.

Examples:

- `demo_immutability()` returns `10`
- `demo_mutability()` increments counter and returns `2`
- `demo_strong_typing()` returns `3300u16`
- `demo_type_inference()` returns `128u8`
- `demo_option_instead_of_null()` returns `None`
- `demo_checked_add_overflow()` uses `checked_add` and returns `None` on overflow
- `demo_tuple_group_initialization()` returns grouped tuple `(3300, 120, true)`
- `demo_underscore_unused_variable()` shows `_raw_adc` intentionally unused and returns active channel

`run_variable_datatype_demo()` aggregates all results into a single summary struct used by `main` logs.

---

## 5) Tutorial topics explained

## A) Immutability by Default
Rust:
- Variables are immutable unless `mut` is used.
- Reduces accidental writes.

C/C++ contrast:
- Variables are mutable by default.
- Extra discipline (`const`) is required for immutability.

## B) Explicit Mutability
Rust:
- `let mut value = ...` clearly communicates write intent.

C/C++ contrast:
- Mutability isn’t explicit in every variable declaration.

## C) Strong Static Typing
Rust:
- Types are strict (`u8`, `u16`, `i32`, etc.).
- Helps prevent unintended mixing and invalid operations.

C/C++ contrast:
- Also statically typed, but Rust’s safety model is stricter around ownership and safe APIs.

## D) Type Inference with Safety
Rust:
- Compiler infers types where obvious, while preserving strict checks.

C/C++ contrast:
- `auto` exists, but conversion behavior and aliasing patterns can still be more error-prone.

## E) Option Instead of Null
Rust:
- Uses `Option<T>` (`Some`/`None`) for absent values.
- Forces explicit handling at compile time.

C/C++ contrast:
- Null pointers/sentinel values are common and can cause runtime faults.

## F) Checked Integer Handling
Rust:
- Checked math APIs (like `checked_add`) return `Option<T>`.
- Makes overflow handling explicit.

C/C++ contrast:
- Overflow behavior can be undefined or silent, depending on type and compiler rules.

## G) Tuple Initialization for Grouped Variables
Rust:
- Tuple lets you initialize and carry related values together in one variable.
- Example used in this project:

```rust
let sensor_group: (u16, u16, bool) = (3300, 120, true);
```

- This can represent a grouped reading like `(voltage_mv, current_ma, is_valid)`.

C/C++ contrast:
- Usually requires custom `struct` or separate variables for quick grouping.
- Rust tuple gives a lightweight typed grouping option immediately.

## H) Prefix `_` for Intentionally Unused Variables
Rust:
- If a variable is intentionally not used, prefix it with underscore.
- Example:

```rust
let _raw_adc: u16 = 512;
let active_channel: u8 = 2;
```

- `_raw_adc` tells the compiler this non-use is intentional, avoiding unused-variable warning.

When to use:
- placeholder values during development,
- partially used return values,
- future-use variables in demos/tests.

---

## 6) How `main` uses this module

At startup, firmware does both:

- logs the topic title,
- logs Rust-side example,
- logs C/C++ comparison note,
- runs executable demos and logs their result values.

Then normal LED blink loop continues.

---

## 7) Build and test commands

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

---

## 8) Key takeaway

Rust makes variable usage and data modeling safer by combining:
- immutability by default,
- explicit mutability,
- strict typing,
- explicit absence handling (`Option`),
- checked arithmetic APIs.

This module demonstrates those ideas in a reusable and test-driven embedded project style.
