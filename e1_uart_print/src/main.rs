#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    main,
    uart::{ClockSource, Config, DataBits, Parity, StopBits, Uart},
};
use esp_println::println;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    let uart_config = Config::default()
        .with_baudrate(115200)
        .with_data_bits(DataBits::_8)
        .with_parity(Parity::None)
        .with_stop_bits(StopBits::_1)
        .with_clock_source(ClockSource::Apb);
    let mut uart_driver = Uart::new(peripherals.UART1, uart_config)
        .unwrap()
        .with_tx(peripherals.GPIO5)
        .with_rx(peripherals.GPIO6);

    esp_println::print!("\x1b[20h");

    loop {
        println!("esp_println output");
        uart_driver
            .write_bytes("write method output".as_bytes())
            .unwrap();
        delay.delay_millis(1000u32);
    }
}
