#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    gpio::{Input, Level, Output, Pull},
    main,
};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut led = Output::new(peripherals.GPIO4, Level::High);
    let button = Input::new(peripherals.GPIO0, Pull::Up);
    let mut blinkdelay = 1_000_000_u32;

    led.set_low();

    loop {
        for _i in 1..blinkdelay {
            if button.is_low() {
                blinkdelay = blinkdelay - 2_5000_u32;
                if blinkdelay < 2_5000 {
                    blinkdelay = 1_000_000_u32;
                }
            }
        }
        led.toggle();
    }
}
