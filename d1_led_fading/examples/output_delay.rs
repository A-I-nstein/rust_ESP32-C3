#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    main,
};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    let mut led_pin = Output::new(peripherals.GPIO1, Level::Low);
    let frequency_hz = 1000_u32;
    let period_seconds = 10_000 / frequency_hz;

    loop {
        for intensity in 0..100 {
            let on_time_seconds = (period_seconds * intensity) / 100;
            let off_time_seconds = period_seconds - on_time_seconds;

            led_pin.set_high();
            delay.delay_millis(on_time_seconds);
            led_pin.set_low();
            delay.delay_millis(off_time_seconds);
        }
        for intensity in (0..100).rev() {
            let on_time_seconds = (period_seconds * intensity) / 100;
            let off_time_seconds = period_seconds - on_time_seconds;

            led_pin.set_high();
            delay.delay_millis(on_time_seconds);
            led_pin.set_low();
            delay.delay_millis(off_time_seconds);
        }
    }
}
