#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    gpio::{Level, Output},
    main,
};
use esp_println::println;
use libm::log;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut adc_config = AdcConfig::new();
    let mut adc_pin = adc_config.enable_pin(peripherals.GPIO4, Attenuation::_11dB);
    let mut adc = Adc::new(peripherals.ADC1, adc_config);
    let mut led_pins = [
        Output::new(peripherals.GPIO9, Level::Low),
        Output::new(peripherals.GPIO8, Level::Low),
        Output::new(peripherals.GPIO7, Level::Low),
        Output::new(peripherals.GPIO6, Level::Low),
        Output::new(peripherals.GPIO5, Level::Low),
    ];

    const B: f64 = 3950.0;
    const VMAX: f64 = 4095.0;
    let mut led_array: [u32; 5];

    loop {
        let sample: u16 = adc.read_oneshot(&mut adc_pin).unwrap();
        let temperature = 1. / (log(1. / (VMAX / sample as f64 - 1.)) / B + 1.0 / 298.15) - 273.15;
        match temperature {
            temperature if temperature < 0.0 => led_array = [1, 0, 0, 0, 0],
            temperature if temperature < 20.0 => led_array = [1, 1, 0, 0, 0],
            temperature if temperature < 40.0 => led_array = [1, 1, 1, 0, 0],
            temperature if temperature < 60.0 => led_array = [1, 1, 1, 1, 0],
            temperature if temperature < 80.0 => led_array = [1, 1, 1, 1, 1],
            _ => led_array = [1, 1, 1, 1, 1],
        }
        println!("Temperature {:02} Celcius\r", temperature);
        for (index, &value) in led_array.iter().enumerate() {
            if value == 1 {
                led_pins[index].set_high();
            } else {
                led_pins[index].set_low();
            }
        }
    }
}
