#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    gpio::{Level, Output},
    main,
};
use esp_println::println;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut adc_config = AdcConfig::new();
    let mut adc_pin = adc_config.enable_pin(peripherals.GPIO0, Attenuation::_11dB);
    let mut adc = Adc::new(peripherals.ADC1, adc_config);
    let mut led_pins = [
        Output::new(peripherals.GPIO19, Level::Low),
        Output::new(peripherals.GPIO18, Level::Low),
        Output::new(peripherals.GPIO4, Level::Low),
        Output::new(peripherals.GPIO5, Level::Low),
        Output::new(peripherals.GPIO6, Level::Low),
        Output::new(peripherals.GPIO7, Level::Low),
        Output::new(peripherals.GPIO8, Level::Low),
        Output::new(peripherals.GPIO9, Level::Low),
    ];

    let mut led_array: [u32; 8];

    loop {
        let sample: u16 = adc.read_oneshot(&mut adc_pin).unwrap();
        println!("{}", sample);
        match sample {
            sample if sample < 1000 => led_array = [1, 1, 0, 0, 0, 0, 0, 0],
            sample if sample < 2000 => led_array = [1, 1, 1, 1, 0, 0, 0, 0],
            sample if sample < 3000 => led_array = [1, 1, 1, 1, 1, 1, 0, 0],
            sample if sample < 4000 => led_array = [1, 1, 1, 1, 1, 1, 1, 1],
            _ => led_array = [1, 1, 1, 1, 1, 1, 1, 1],
        }
        for (index, &value) in led_array.iter().enumerate() {
            if value == 1 {
                led_pins[index].set_high();
            } else {
                led_pins[index].set_low();
            }
        }
    }
}
