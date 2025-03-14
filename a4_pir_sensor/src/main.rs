#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, Level, Output, Pull},
    main,
};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    let mut led_pin = Output::new(peripherals.GPIO4, Level::Low);
    let _sensor_pin = Output::new(peripherals.GPIO1, Level::High);
    let sensor_output = Input::new(peripherals.GPIO0, Pull::Down);

    loop {
        if sensor_output.is_high() {
            led_pin.set_high();
            delay.delay_millis(5000u32);
            led_pin.set_low();
            delay.delay_millis(2000u32);
        }
    }
}
