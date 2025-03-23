#![no_std]
#![no_main]

use core::cell::{Cell, RefCell};
use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::{
    delay::MicrosDurationU64,
    timer::timg::{Timer, TimerGroup},
    timer::Timer as OtherTimer,
    {handler, main}
};
use esp_hal::interrupt::InterruptConfigurable;
use esp_println::println;

static G_TIMER: Mutex<RefCell<Option<Timer>>> = Mutex::new(RefCell::new(None));
static G_FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

struct Time {
    seconds: u32,
    minutes: u32,
    hours: u32,
}

#[handler]
fn tg0_to_level() {
    critical_section::with(|cs| {
        G_TIMER.borrow_ref_mut(cs).as_mut().unwrap().clear_interrupt();
        G_FLAG.borrow(cs).set(true);
    });
}

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let timer_group_0 = TimerGroup::new(peripherals.TIMG0);
    let mut timer_0 = timer_group_0.timer0;

    timer_0
        .load_value(MicrosDurationU64::micros(1_000_000))
        .unwrap();
    timer_0.set_interrupt_handler(tg0_to_level);
    timer_0.enable_interrupt(true);
    timer_0.start();
    critical_section::with(|cs| G_TIMER.borrow_ref_mut(cs).replace(timer_0));
    let mut time = Time {
        seconds: 0_u32,
        minutes: 0_u32,
        hours: 0_u32,
    };

    loop {
        critical_section::with(|cs| {
            if G_FLAG.borrow(cs).get() {
                G_FLAG.borrow(cs).set(false);
                time.seconds = time.seconds.wrapping_add(1);
                if time.seconds > 59 {
                    time.minutes += 1;
                    time.seconds = 0
                }
                if time.minutes > 59 {
                    time.hours += 1;
                    time.minutes = 0;
                }
                if time.hours > 23 {
                    time.seconds = 0;
                    time.minutes = 0;
                    time.hours = 0;
                }
                println!(
                    "Elapsed Time {:0>2}:{:0>2}:{:0>2}",
                    time.hours, time.minutes, time.seconds
                );
            }
        });
    }
}