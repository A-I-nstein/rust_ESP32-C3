#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_hal::{timer::timg::TimerGroup, Config};
use esp_hal_embassy::main;
use esp_println::println;
use esp_backtrace as _;

struct Time {
    seconds: u32,
    minutes: u32,
    hours: u32,
}

#[main]
async fn main(_spawner: Spawner) {
    let mut time = Time {
        seconds: 0_u32,
        minutes: 0_u32,
        hours: 0_u32,
    };

    let peripherals = esp_hal::init(Config::default());
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    loop {
        Timer::after_millis(1000).await;
        time.seconds = time.seconds.wrapping_add(1);

        if time.seconds > 59 {
            time.seconds = 0;
            time.minutes += 1;
        }
        if time.minutes > 59 {
            time.minutes = 0;
            time.hours += 1;
        }
        if time.hours > 23 {
            time.seconds = 0;
            time.minutes = 0;
            time.hours = 0;
        }
        println!("Elapsed Time {:0>2}:{:0>2}:{:0>2}", time.hours, time.minutes, time.seconds)
    }
}
