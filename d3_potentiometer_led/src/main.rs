#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    delay::Delay,
    ledc::{
        channel::{self, ChannelIFace},
        timer::{self, TimerIFace},
        LSGlobalClkSource, Ledc, LowSpeed,
    },
    main,
    time::RateExtU32
};

fn map(x: u32, in_min: u32, in_max: u32, out_min: u32, out_max: u32) -> u32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    let mut adc_config = AdcConfig::new();
    let mut adc_pin = adc_config.enable_pin(peripherals.GPIO0, Attenuation::_11dB);
    let mut adc = Adc::new(peripherals.ADC1, adc_config);
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

    loop {
        let sample: u16 = adc.read_oneshot(&mut adc_pin).unwrap();
        channel.set_duty(map(sample.into(), 0, 4095, 0, 100).try_into().unwrap()).unwrap();
        delay.delay_millis(500_u32);
    }
}
