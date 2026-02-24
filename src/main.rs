#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    main,
    time::{Duration, Instant},
};
use embedded_rust::byte_literal::{byte_literal_topics, run_byte_literal_demo};
use embedded_rust::char_tutorial::{char_topics, run_char_demo};
use embedded_rust::memory_safety::memory_safety_features;
use embedded_rust::staticvariable::{run_staticvariable_demo, staticvariable_topics};
use embedded_rust::variaabl_datatype::{run_variable_datatype_demo, variable_datatype_features};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    esp_println::logger::init_logger_from_env();

    for feature in memory_safety_features() {
        log::info!("Memory Safety Feature: {}\r\n", feature.title);
        log::info!("Rust guarantee: {}\r\n", feature.rust_guarantee);
        log::info!("C/C++ risk: {}\r\n", feature.c_cpp_risk);
    }

    for topic in variable_datatype_features() {
        log::info!("Variable/DataType Topic: {}\r\n", topic.topic);
        log::info!("Rust example: {}\r\n", topic.rust_example);
        log::info!("C/C++ note: {}\r\n", topic.c_cpp_note);
    }

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

    for topic in byte_literal_topics() {
        log::info!("Byte Literal Topic: {}\r\n", topic.title);
        log::info!("Rust example: {}\r\n", topic.rust_example);
        log::info!("Explanation: {}\r\n", topic.explanation);
    }

    let byte_demo = run_byte_literal_demo();
    log::info!("Byte demo b'A' value: {}\r\n", byte_demo.ascii_a);
    log::info!("Byte demo b'\\n' value: {}\r\n", byte_demo.newline);
    log::info!("Byte demo b\"HELLO\" length: {}\r\n", byte_demo.bytes_len);
    log::info!("Byte demo first byte: {}\r\n", byte_demo.bytes_first);
    log::info!("Byte demo HELLO sum: {}\r\n", byte_demo.hello_sum);

    for topic in char_topics() {
        log::info!("Char Topic: {}\r\n", topic.title);
        log::info!("Rust example: {}\r\n", topic.rust_example);
        log::info!("Explanation: {}\r\n", topic.explanation);
    }

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

    for topic in staticvariable_topics() {
        log::info!("Static/Scope Topic: {}\r\n", topic.title);
        log::info!("Rust example: {}\r\n", topic.rust_example);
        log::info!("Explanation: {}\r\n", topic.explanation);
    }

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

    loop {
        led.set_high();
        log::info!("LED status: ON\r\n");
        let start_on = Instant::now();
        while start_on.elapsed() < Duration::from_millis(1000) {}

        led.set_low();
        log::info!("LED status: OFF\r\n");
        let start_off = Instant::now();
        while start_off.elapsed() < Duration::from_millis(1000) {}
    }
}
