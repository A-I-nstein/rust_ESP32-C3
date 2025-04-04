#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    ledc::{
        channel::{self, ChannelIFace},
        timer::{self, TimerIFace},
        LSGlobalClkSource, Ledc, LowSpeed,
    },
    main,
    time::RateExtU32,
    Config,
};

fn map(x: u32, in_min: u32, in_max: u32, out_min: u32, out_max: u32) -> u32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(Config::default());
    let led = peripherals.GPIO7;
    let mut ledc = Ledc::new(peripherals.LEDC);
    let mut timer = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    let mut channel = ledc.channel::<LowSpeed>(channel::Number::Channel0, led);

    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    timer
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty14Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: RateExtU32::kHz(1),
        })
        .unwrap();
    channel
        .configure(channel::config::Config {
            timer: &timer,
            duty_pct: 0,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

    let max_duty = 100_000_u32;
    let min_duty = 0_u32;

    loop {
        for duty in min_duty..max_duty {
            let new_duty = map(duty, 0, 100_000, 0, 100);
            channel.set_duty(new_duty.try_into().unwrap()).unwrap();
        }
        for duty in (min_duty..max_duty).rev() {
            let new_duty = map(duty, 0, 100_000, 0, 100);
            channel.set_duty(new_duty.try_into().unwrap()).unwrap();
        }
    }
}
