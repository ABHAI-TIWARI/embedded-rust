# Rust Memory Safety Features (vs C/C++)

This tutorial explains the memory safety features implemented in this project and how they compare with common C/C++ patterns.

---

## 1) What was implemented

The code is modular and testable:

- `src/memory_safety.rs`
  - Defines `SafetyFeature` (a reusable data model).
  - Contains curated memory safety lessons.
  - Exposes:
    - `memory_safety_features()`
    - `lesson_count()`
    - `feature_by_title()`
  - Includes unit tests.
- `src/lib.rs`
  - Exposes the `memory_safety` module so it can be reused by `main.rs` and tests.
- `src/main.rs`
  - Calls `memory_safety_features()` during startup.
  - Prints each memory safety lesson via ESP logger.
  - Continues LED blinking behavior.

---

## 2) Why this is modular

Instead of hardcoding text directly in `main.rs`, the memory-safety content is isolated in a dedicated module.

Benefits:

1. **Separation of concerns**
   - `main.rs` handles board/runtime behavior (logging + LED blink).
   - `memory_safety.rs` handles tutorial content.
2. **Reusability**
   - Any future CLI/web/docs/export flow can reuse the same feature list.
3. **Maintainability**
   - Updating one lesson does not require touching firmware loop logic.

---

## 3) Why this is testable

The module provides pure functions with deterministic output:

- `lesson_count()` can be validated with exact assertions.
- `feature_by_title()` can be tested for found/not-found cases.
- Text completeness can be tested to avoid blank documentation entries.

Unit tests included in `src/memory_safety.rs` verify:

- Expected number of lessons.
- Presence of specific lesson (`Ownership`).
- `None` for unknown lesson keys.
- Non-empty strings for all entries.

---

## 4) Memory safety features explained (Rust vs C/C++)

### A) Ownership

Rust:

- Every value has one owner.
- Ownership transfer (move) is explicit.
- Value is dropped exactly once.

C/C++ risk:

- Ownership is often a convention, not enforced by compiler in all cases.
- Multiple pointers may believe they own the same allocation.
- Can cause double free or memory leaks.

### B) Borrowing Rules

Rust:

- Compiler enforces either:
  - many immutable references (`&T`), or
  - one mutable reference (`&mut T`).
- Prevents unsafe aliasing patterns.

C/C++ risk:

- Mutable aliasing can happen through pointers/references.
- Shared mutable access can introduce undefined behavior and race conditions.

### C) Lifetimes

Rust:

- References are checked so they never outlive owned data.
- Prevents dangling references at compile time.

C/C++ risk:

- Returning/storing pointer to stack memory after scope ends is possible.
- Use-after-free and dangling pointer bugs may occur.

### D) Bounds-Checked Access

Rust:

- Safe indexing validates bounds.
- Out-of-bounds in safe code fails predictably instead of silent corruption.

C/C++ risk:

- Out-of-bounds reads/writes can corrupt adjacent memory.
- Often hard to diagnose and can become security vulnerabilities.

### E) No Null References

Rust:

- Absence is explicit with `Option<T>`.
- Caller must handle `Some`/`None`.

C/C++ risk:

- Null pointers can be dereferenced by mistake.
- Leads to crash or undefined behavior.

---

## 5) How `main.rs` uses the memory safety tutorial

On boot, `main.rs` calls:

```rust
for feature in memory_safety_features() {
    log::info!("Memory Safety Feature: {}\\r\\n", feature.title);
    log::info!("Rust guarantee: {}\\r\\n", feature.rust_guarantee);
    log::info!("C/C++ risk: {}\\r\\n", feature.c_cpp_risk);
}
```

Then firmware continues with LED blink (GPIO2):

- `ON` for 1 second
- `OFF` for 1 second

---

## 6) Build and test commands

### Firmware build (ESP32 target)

```bash
source "$HOME/.cargo/env"
source "$HOME/export-esp.sh"
cargo build --release
```

### Unit tests (host target)

```bash
source "$HOME/.cargo/env"
cargo test --lib --target x86_64-unknown-linux-gnu
```

---

## 7) Key takeaway

Rust memory safety is mostly **compile-time enforced** rather than relying on runtime discipline.
This project demonstrates that idea in a small embedded-friendly module that is:

- easy to call from firmware,
- easy to test,
- easy to extend with more lessons.
