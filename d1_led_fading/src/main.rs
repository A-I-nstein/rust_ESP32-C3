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

    let max_duty = 100_u8;
    let min_duty = 0_u8;

    loop {
        for duty in min_duty..max_duty {
            channel.set_duty(duty).unwrap();
            delay.delay_millis(10_u32);
        }
        for duty in (min_duty..max_duty).rev() {
            channel.set_duty(duty).unwrap();
            delay.delay_millis(10_u32);
        }
    }
}
