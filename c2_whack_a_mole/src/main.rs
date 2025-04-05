#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::MicrosDurationU64,
    gpio::{Input, Level, Output, Pull},
    main,
    rng::Rng,
    timer::{timg::TimerGroup, Timer as OtherTimer},
};
use esp_println::println;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut led = Output::new(peripherals.GPIO4, Level::Low);
    let button = Input::new(peripherals.GPIO0, Pull::Up);
    let mut range = Rng::new(peripherals.RNG);
    let timer_group_0 = TimerGroup::new(peripherals.TIMG0);
    let timer_0 = timer_group_0.timer0;
    let timer_group_1 = TimerGroup::new(peripherals.TIMG1);
    let timer_1 = timer_group_1.timer0;
    let mut best_response_time = u64::MAX;
    println!("Game begins! Press button B when the led glows.");

    loop {
        let mut mole_whacked = false;
        let mut response_time = 0_u64;
        let init_delay_s = (range.random() % 5 + 1) as u64;
        println!("{}", init_delay_s);
        timer_0
            .load_value(MicrosDurationU64::secs(init_delay_s))
            .unwrap();
        timer_0.start();
        while !timer_0.is_interrupt_set() {}
        timer_0.clear_interrupt();
        led.toggle();
        let blink_delay_ms = (range.random() % (1500 - 500 + 1) + 500) as u64;
        println!("{}", blink_delay_ms);
        timer_0
            .load_value(MicrosDurationU64::millis(blink_delay_ms))
            .unwrap();
        timer_0.start();
        timer_1.load_value(MicrosDurationU64::secs(10)).unwrap();
        timer_1.start();
        while !timer_0.is_interrupt_set() {
            if button.is_low() {
                response_time = timer_1.now().ticks() / 1_000_u64;
                if response_time < best_response_time {
                    best_response_time = response_time;
                }
                timer_1.stop();
                timer_1.clear_interrupt();
                mole_whacked = true;
                break;
            }
        }
        timer_0.clear_interrupt();
        led.toggle();
        if !mole_whacked {
            println!("The mole escaped");
        } else {
            println!(
                "You have whacked the mole! Response Time: {}, Best Response Time: {}",
                response_time, best_response_time
            );
        }
    }
}
