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

// Musical Notes and their frequency in Hz
// C    C#  D   D#  E   F   F#  G   G#  A   A#  B
// 262  277 294 311 330 349 370 392 415 440 466 494

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(Config::default());
    let delay = Delay::new();
    let mut led = peripherals.GPIO7;
    let mut ledc = Ledc::new(peripherals.LEDC);

    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let happy_birthday = [
        262, 262, 294, 262, 349, 330, 262, 262, 294, 262, 392, 349, 262, 262, 523, 440, 349, 330,
        294, 466, 466, 440, 349, 392, 349,
    ];

    loop {
        let mut timer = ledc.timer::<LowSpeed>(timer::Number::Timer0);
        for frequency in happy_birthday {
            timer
                .configure(timer::config::Config {
                    duty: timer::config::Duty::Duty14Bit,
                    clock_source: timer::LSClockSource::APBClk,
                    frequency: RateExtU32::Hz(frequency),
                })
                .unwrap();
            let mut channel = ledc.channel::<LowSpeed>(channel::Number::Channel0, &mut led);
            channel
                .configure(channel::config::Config {
                    timer: &timer,
                    duty_pct: 0,
                    pin_config: channel::config::PinConfig::PushPull,
                })
                .unwrap();
            channel.set_duty(20).unwrap();
            delay.delay_millis(200);
            channel.set_duty(0).unwrap();
            delay.delay_millis(50);
        }
        delay.delay_millis(1000);
    }
}
