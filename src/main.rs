#![no_std]
#![no_main]

//! Embedded firmware entry for ESP32 (Rust version of a classic C superloop app).
//!
//! Quick mapping for Embedded C developers:
//! - `#![no_std]` => no desktop/OS standard library, similar to bare-metal C.
//! - `#![no_main]` => no host-style startup; runtime/boot code calls our entry.
//! - `#[main] fn main() -> !` => firmware entrypoint that never returns.
//! - `loop { ... }` => classic infinite superloop used in MCU firmware.
//!
//! Runtime behavior in this file:
//! 1) initialize peripherals,
//! 2) configure GPIO2 output (onboard LED),
//! 3) initialize logger,
//! 4) print tutorial topics/demos,
//! 5) blink LED forever.

// Panic handler + backtrace integration for embedded target.
// In C terms, this is like having a panic/fault reporting helper so crashes are diagnosable.
use esp_backtrace as _;
// ESP HAL imports for GPIO control and simple timing loops.
use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    main,
    time::{Duration, Instant},
};
// Tutorial modules used to print educational startup logs.
use embedded_rust::byte_literal::{byte_literal_topics, run_byte_literal_demo};
use embedded_rust::char_tutorial::{char_topics, run_char_demo};
use embedded_rust::memory_safety::memory_safety_features;
use embedded_rust::staticvariable::{run_staticvariable_demo, staticvariable_topics};
use embedded_rust::variaabl_datatype::{run_variable_datatype_demo, variable_datatype_features};

// Inject ESP-IDF compatible app descriptor into the final firmware image.
// Some flashing/image tools expect this metadata block to exist.
esp_bootloader_esp_idf::esp_app_desc!();

// Firmware entry point (replaces standard Rust `fn main()`).
// Return type `!` means this function never returns (normal for MCU firmware).
#[main]
fn main() -> ! {
    // Initialize chip peripherals with default board configuration.
    let peripherals = esp_hal::init(esp_hal::Config::default());
    // Configure onboard LED pin (GPIO2) as output.
    // Equivalent idea in C: set GPIO mode to output and initialize output level.
    let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    // Initialize logger so `log::info!` appears on serial output.
    esp_println::logger::init_logger_from_env();

    // Print memory safety tutorial topics.
    for feature in memory_safety_features() {
        log::info!("Memory Safety Feature: {}\r\n", feature.title);
        log::info!("Rust guarantee: {}\r\n", feature.rust_guarantee);
        log::info!("C/C++ risk: {}\r\n", feature.c_cpp_risk);
    }

    // Print variable/data type tutorial topics.
    for topic in variable_datatype_features() {
        log::info!("Variable/DataType Topic: {}\r\n", topic.topic);
        log::info!("Rust example: {}\r\n", topic.rust_example);
        log::info!("C/C++ note: {}\r\n", topic.c_cpp_note);
    }

    // Run variable/data type executable demos and print results.
    let demo = run_variable_datatype_demo();
    log::info!("Demo immutable value: {}\r\n", demo.immutable_value);
    log::info!("Demo mutable counter: {}\r\n", demo.mutable_counter);
    log::info!("Demo strong type voltage_mv: {}\r\n", demo.voltage_mv);
    log::info!("Demo inferred duty cycle: {}\r\n", demo.inferred_duty_cycle);
    log::info!("Demo Option has value: {}\r\n", demo.option_has_value);
    log::info!(
        "Demo checked_add overflow handled: {}\r\n",
        demo.checked_add_overflowed
    );
    log::info!(
        "Demo tuple group (voltage,current,ok): ({},{},{})\r\n",
        demo.tuple_group.0,
        demo.tuple_group.1,
        demo.tuple_group.2
    );
    log::info!(
        "Demo underscore-unused active channel: {}\r\n",
        demo.active_channel
    );

    // Print byte literal tutorial topics.
    for topic in byte_literal_topics() {
        log::info!("Byte Literal Topic: {}\r\n", topic.title);
        log::info!("Rust example: {}\r\n", topic.rust_example);
        log::info!("Explanation: {}\r\n", topic.explanation);
    }

    // Run byte literal demos and print computed values.
    let byte_demo = run_byte_literal_demo();
    log::info!("Byte demo b'A' value: {}\r\n", byte_demo.ascii_a);
    log::info!("Byte demo b'\\n' value: {}\r\n", byte_demo.newline);
    log::info!("Byte demo b\"HELLO\" length: {}\r\n", byte_demo.bytes_len);
    log::info!("Byte demo first byte: {}\r\n", byte_demo.bytes_first);
    log::info!("Byte demo HELLO sum: {}\r\n", byte_demo.hello_sum);

    // Print char tutorial topics.
    for topic in char_topics() {
        log::info!("Char Topic: {}\r\n", topic.title);
        log::info!("Rust example: {}\r\n", topic.rust_example);
        log::info!("Explanation: {}\r\n", topic.explanation);
    }

    // Run char demos and print computed values.
    let char_demo = run_char_demo();
    log::info!("Char demo ascii char: {}\r\n", char_demo.ascii_char);
    log::info!("Char demo unicode char: {}\r\n", char_demo.unicode_char);
    log::info!(
        "Char demo unicode utf8 length: {}\r\n",
        char_demo.unicode_len_utf8
    );
    log::info!("Char demo uppercase: {}\r\n", char_demo.upper_char);
    log::info!(
        "Char demo digit conversion: {:?}\r\n",
        char_demo.digit_to_number
    );

    // Print static/local scope tutorial topics.
    for topic in staticvariable_topics() {
        log::info!("Static/Scope Topic: {}\r\n", topic.title);
        log::info!("Rust example: {}\r\n", topic.rust_example);
        log::info!("Explanation: {}\r\n", topic.explanation);
    }

    // Run scope/static demos and print computed values.
    let scope_demo = run_staticvariable_demo();
    log::info!("Scope demo local counter: {}\r\n", scope_demo.local_counter);
    log::info!(
        "Scope demo global boot count: {}\r\n",
        scope_demo.global_boot_count
    );
    log::info!("Scope demo snake_case valid: {}\r\n", scope_demo.snake_case_ok);
    log::info!(
        "Scope demo UPPER_CASE constant limit: {}\r\n",
        scope_demo.constant_limit
    );
    log::info!("Scope demo CamelCase type name: {}\r\n", scope_demo.type_name);

    // Main firmware loop: blink onboard LED forever.
    // This is the same superloop pattern commonly used in embedded C projects.
    loop {
        // LED ON phase.
        // `set_high()` drives GPIO2 high and turns LED on (board-dependent active level).
        led.set_high();
        log::info!("LED status: ON\r\n");
        let start_on = Instant::now();
        while start_on.elapsed() < Duration::from_millis(1000) {}

        // LED OFF phase.
        // `set_low()` drives GPIO2 low and turns LED off.
        led.set_low();
        log::info!("LED status: OFF\r\n");
        let start_off = Instant::now();
        while start_off.elapsed() < Duration::from_millis(1000) {}
    }
}
