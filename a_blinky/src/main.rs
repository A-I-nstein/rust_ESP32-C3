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

    loop {
        led_pin.set_high();
        delay.delay_millis(1000u32);
        led_pin.set_low();
        delay.delay_millis(1000u32);
    }
}
