#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    main,
};
use esp_println::println;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut adc_config = AdcConfig::new();
    let mut hor_pin = adc_config.enable_pin(peripherals.GPIO0, Attenuation::_11dB);
    let mut ver_pin = adc_config.enable_pin(peripherals.GPIO1, Attenuation::_11dB);
    let mut sel_pin = adc_config.enable_pin(peripherals.GPIO3, Attenuation::_11dB);
    let mut adc = Adc::new(peripherals.ADC1, adc_config);

    loop {
        let hor_val: u16 = adc.read_oneshot(&mut hor_pin).unwrap();
        let ver_val: u16 = adc.read_oneshot(&mut ver_pin).unwrap();
        let sel_val: u16 = adc.read_oneshot(&mut sel_pin).unwrap();

        let position_name = match (ver_val, hor_val) {
            (4095, 4095) => "Top-Left",
            (4095, 2048) => "Top",
            (4095, 0) => "Top-Right",
            (2048, 4095) => "Left",
            (2048, 2048) => "Center",
            (2048, 0) => "Right",
            (0, 4095) => "Bottom-Left",
            (0, 2048) => "Bottom",
            (0, 0) => "Bottom-Right",
            _ => "Unknown",
        };
        println!("Position: {}", position_name);
        if sel_val == 0 {
            println! {"Button Pressed!"};
        }
    }
}
