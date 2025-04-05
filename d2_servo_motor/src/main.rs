#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    ledc::{
        channel::{self, ChannelIFace},
        timer::{self, TimerIFace},
        LSGlobalClkSource, Ledc, LowSpeed,
    },
    main,
    time::RateExtU32,
    Config,
};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(Config::default());
    let delay = Delay::new();
    let led = peripherals.GPIO7;
    let mut ledc = Ledc::new(peripherals.LEDC);
    let mut timer = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    let mut channel = ledc.channel::<LowSpeed>(channel::Number::Channel0, led);

    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    timer
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty14Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: RateExtU32::Hz(50),
        })
        .unwrap();
    channel
        .configure(channel::config::Config {
            timer: &timer,
            duty_pct: 0,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

    loop {
        channel.set_duty(3).unwrap(); // pos 0 degree
        delay.delay_millis(1000);
        channel.set_duty(12).unwrap(); // pos 180 degree
        delay.delay_millis(1000);
    }
}
