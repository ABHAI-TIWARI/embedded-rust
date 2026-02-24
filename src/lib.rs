// Apply `no_std` for normal firmware builds, but allow `std` during tests.
// `cfg_attr(condition, attr)` means: apply `attr` only when condition is true.
// Here: when NOT running tests, compile as `no_std`.
#![cfg_attr(not(test), no_std)]

// Memory safety tutorial module (ownership, borrowing, lifetimes, etc.).
pub mod memory_safety;
// Variables and data types tutorial module.
pub mod variaabl_datatype;
// Byte literal tutorial module.
pub mod byte_literal;
// Character (`char`) tutorial module.
pub mod char_tutorial;
// Local/global/static variable and naming tutorial module.
pub mod staticvariable;
