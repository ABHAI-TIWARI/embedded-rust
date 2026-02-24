# Tutorial: Static Variables, Local vs Global, and Naming in Rust

This tutorial focuses on static/global variables and their relation to local variables in Rust.

Implementation module: `src/staticvariable.rs`

## 1) Focus on Static Variables: Global vs Local

Rust clearly separates:
- **Local variables**: declared inside functions, short-lived by scope.
- **Global/static variables**: declared with `static`, available for full program lifetime.

## 2) Local Variables (`let` + Type Inference)

Local variables are created with `let`.

```rust
let local_counter = 1;
```

Rust infers type in many cases, so explicit annotations are often optional:

```rust
let local_counter: u32 = 1;
```

In this project:
- `demo_local_variable()` demonstrates local scope behavior.

## 3) Global Variables (`static`, not `let`)

You **cannot** use `let` outside functions for global data.
Global data must use `static` (or `const` for compile-time constants).

```rust
static GLOBAL_BOOT_COUNT: AtomicU32 = AtomicU32::new(0);
```

In this project:
- `demo_global_static_increment()` updates and reads a static atomic counter.

## 4) Naming Convention for Globals

Global/static and constant names are written in uppercase style:

```rust
static GLOBAL_BOOT_COUNT: AtomicU32 = AtomicU32::new(0);
const MAX_SENSOR_LIMIT: u16 = 4095;
```

This helps visually separate global/static values from local `snake_case` variables.

## 5) Immutable vs Mutable Static Variables

### Immutable static

- Declared with `static NAME: Type = value;`
- Cannot be changed at runtime.
- Typically placed in read-only memory regions (ROM/flash, platform-dependent).

Example:

```rust
static DEVICE_NAME: &str = "ESP32-WROOM";
```

### Mutable static

- Declared with `static mut`.
- Can be modified, but reading/writing requires `unsafe`.
- Risk: data races in concurrent/multithreaded or interrupt-driven contexts.

Example (educational only):

```rust
static mut COUNTER: u32 = 0;

unsafe {
    COUNTER += 1;
}
```

Safer alternative used in this repo:
- `AtomicU32` (`GLOBAL_BOOT_COUNT`) avoids raw `static mut` access patterns.

## 6) Memory Safety Notes

- Immutable statics have stable memory addresses and are safe to read globally.
- Mutable global state is stored in writable memory (RAM) and needs synchronization.
- Rust requires explicit `unsafe` for `static mut` to make risk visible.
- Safer wrappers (atomics, mutexes, critical sections) are preferred.

## 7) Lifetime (`'static`)

Static variables live for the full program duration and effectively have `'static` lifetime.

Example:

```rust
static GREETING: &str = "hello"; // &'static str
```

## 8) Executable Demo APIs

Module: `src/staticvariable.rs`

- `staticvariable_topics()`
- `demo_local_variable()`
- `demo_global_static_increment()`
- `demo_naming_convention()`
- `demo_constant_naming()`
- `demo_type_naming()`
- `run_staticvariable_demo()`

`run_staticvariable_demo()` returns:
- local variable result,
- global/static counter value,
- naming-convention validation,
- constant value,
- type-name output.

## 9) Firmware Logging

`src/main.rs` logs:
- scope/static tutorial topics,
- Rust examples,
- explanations,
- computed demo results.

## 10) Build and Test

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

## 11) Notes for Embedded Projects

- Prefer local variables when state does not need cross-function lifetime.
- Use `static` carefully for truly global state.
- For mutable global state, prefer atomic or synchronization primitives.
- Follow naming conventions to keep codebase maintainable.
