#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    gpio::{Level, Output},
    main,
    timer::{timg::TimerGroup, Timer},
};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut led_pin = Output::new(peripherals.GPIO0, Level::Low);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0);
    let timer0 = timer_group0.timer0;

    let mut start = timer0.now();
    timer0.start();

    loop {
        if timer0
            .now()
            .checked_duration_since(start)
            .unwrap()
            .to_secs()
            >= 1
        {
            led_pin.toggle();
            start = timer0.now();
        }
    }
}
