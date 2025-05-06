#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    Config,
    gpio::{Level, Output, OutputConfig},
    init,
    timer::timg::TimerGroup,
};

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    let peripherals = init(Config::default());
    let timg0 = TimerGroup::new(peripherals.TIMG0);

    esp_hal_embassy::init(timg0.timer0);

    let led_config = OutputConfig::default();
    let mut led = Output::new(peripherals.GPIO1, Level::High, led_config);

    loop {
        led.set_high();
        Timer::after(Duration::from_millis(1_000)).await;
        led.set_low();
        Timer::after(Duration::from_millis(1_000)).await;
    }
}
