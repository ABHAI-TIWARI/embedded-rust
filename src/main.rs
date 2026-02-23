#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    main,
    time::{Duration, Instant},
};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    esp_println::logger::init_logger_from_env();

    loop {
        led.set_high();
        log::info!("LED status: ON");
        let start_on = Instant::now();
        while start_on.elapsed() < Duration::from_millis(1000) {}

        led.set_low();
        log::info!("LED status: OFF");
        let start_off = Instant::now();
        while start_off.elapsed() < Duration::from_millis(1000) {}
    }
}
