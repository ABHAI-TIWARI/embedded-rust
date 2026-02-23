#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    main,
    time::{Duration, Instant},
};
use embedded_rust::memory_safety::memory_safety_features;

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
