#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::MicrosDurationU64,
    gpio::{Input, Level, Output, Pull},
    main,
    timer::timg::TimerGroup,
    timer::Timer as OtherTimer,
};
use esp_println::println;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut led = Output::new(peripherals.GPIO4, Level::High);
    let button = Input::new(peripherals.GPIO0, Pull::Up);
    let mut blinkdelay = 1_000_000_u32;
    let timer_group_0 = TimerGroup::new(peripherals.TIMG0);
    let timer_0 = timer_group_0.timer0;

    led.set_low();

    loop {
        let now = timer_0.now();
        timer_0
            .load_value(MicrosDurationU64::micros(u64::MAX))
            .unwrap();
        timer_0.start();
        let start_time = timer_0.now();
        for _i in 1..blinkdelay {
            if button.is_low() {
                blinkdelay = blinkdelay - 2_5000_u32;
                if blinkdelay < 2_5000 {
                    blinkdelay = 1_000_000_u32;
                }
            }
        }
        led.toggle();
        let end_time = timer_0.now();
        let difference = end_time - start_time;
        println!("difference-{}", difference);
    }
}
