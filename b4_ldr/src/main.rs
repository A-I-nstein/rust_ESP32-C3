#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    delay::Delay,
    main,
};
use esp_println::println;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    let mut adc_config = AdcConfig::new();
    let mut adc_pin = adc_config.enable_pin(peripherals.GPIO0, Attenuation::_11dB);
    let mut adc = Adc::new(peripherals.ADC1, adc_config);

    loop {
        let sample: u16 = adc.read_oneshot(&mut adc_pin).unwrap();
        let condition = match sample {
            sample if sample >= 4000 => "Full moon",
            sample if sample >= 3940 => "Deep twilight",
            sample if sample >= 3412 => "Twilight",
            sample if sample >= 2532 => "Computer monitor",
            sample if sample >= 2044 => "Stairway lighting",
            sample if sample >= 1124 => "Office lighting",
            sample if sample >= 680 => "Overcast day",
            sample if sample >= 156 => "Full daylight",
            sample if sample >= 32 => "Direct sunlight",
            _ => "Unknown",
        };
        println!("Condition: {}", condition);
        delay.delay_millis(500_u32);
    }
}
