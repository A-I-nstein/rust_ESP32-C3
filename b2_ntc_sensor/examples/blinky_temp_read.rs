#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    delay::Delay,
    gpio::{Level, Output},
    main,
};
use esp_println::println;
use libm::log;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    let mut adc_config = AdcConfig::new();
    let mut adc_pin = adc_config.enable_pin(peripherals.GPIO4, Attenuation::_11dB);
    let mut adc = Adc::new(peripherals.ADC1, adc_config);
    let mut led_pin = Output::new(peripherals.GPIO5, Level::High);

    const B: f64 = 3950.0;
    const VMAX: f64 = 4095.0;
    let mut blinkdelay = 1_000_u32;

    loop {
        let sample: u16 = adc.read_oneshot(&mut adc_pin).unwrap();
        let temperature = 1. / (log(1. / (VMAX / sample as f64 - 1.)) / B + 1.0 / 298.15) - 273.15;
        match temperature {
            temperature if temperature < 0.0 => blinkdelay = 1_000_u32,
            temperature if temperature < 20.0 => blinkdelay = 800_u32,
            temperature if temperature < 40.0 => blinkdelay = 600_u32,
            temperature if temperature < 60.0 => blinkdelay = 400_u32,
            temperature if temperature < 80.0 => blinkdelay = 200_u32,
            _ => blinkdelay = 100_u32,
        }
        println!("Temperature {:02} Celcius\r", temperature);
        led_pin.toggle();
        delay.delay_millis(blinkdelay);
    }
}
