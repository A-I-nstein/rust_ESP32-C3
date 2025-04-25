#![no_std]
#![no_main]

use core::cell::{Cell, RefCell};
use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::gpio::{Input, Pull};
use esp_hal::interrupt::InterruptConfigurable;
use esp_hal::{
    Config,
    delay::{Delay, MicrosDurationU64},
    gpio::{Level, Output},
    handler, init, main,
    timer::{
        Timer as OtherTimer,
        timg::{Timer, TimerGroup},
    },
};
use esp_println::println;

static TX_TIMER: Mutex<RefCell<Option<Timer>>> = Mutex::new(RefCell::new(None));
static TX_FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

static RX_TIMER: Mutex<RefCell<Option<Timer>>> = Mutex::new(RefCell::new(None));
static RX_FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

#[handler]
fn tg0_to_level() {
    critical_section::with(|cs| {
        TX_TIMER
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();
        TX_FLAG.borrow(cs).set(true);
    });
}

#[handler]
fn tg1_to_level() {
    critical_section::with(|cs| {
        RX_TIMER
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();
        RX_FLAG.borrow(cs).set(true);
    });
}

#[main]
fn main() -> ! {
    let data: [u8; 12] = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
    let mut data_iter = data.iter();

    let peripherals = init(Config::default());
    let delay = Delay::new();

    let timer_group_0 = TimerGroup::new(peripherals.TIMG0);
    let mut timer_0 = timer_group_0.timer0;
    timer_0
        .load_value(MicrosDurationU64::micros(1_000_000))
        .unwrap();
    timer_0.set_interrupt_handler(tg0_to_level);
    timer_0.enable_interrupt(true);
    timer_0.start();
    critical_section::with(|cs| TX_TIMER.borrow_ref_mut(cs).replace(timer_0));

    let timer_group_1 = TimerGroup::new(peripherals.TIMG1);
    let mut timer_1 = timer_group_1.timer0;
    timer_1
        .load_value(MicrosDurationU64::micros(1_000_000))
        .unwrap();
    timer_1.set_interrupt_handler(tg1_to_level);
    timer_1.enable_interrupt(true);

    let mut tx_pin = Output::new(peripherals.GPIO2, Level::High);
    let rx_pin = Input::new(peripherals.GPIO3, Pull::Up);

    delay.delay_millis(2000_u32);
    tx_pin.set_low();
    delay.delay_millis(1000_u32);

    delay.delay_millis(500_u32);
    timer_1.start();
    critical_section::with(|cs| RX_TIMER.borrow_ref_mut(cs).replace(timer_1));

    loop {
        critical_section::with(|cs| {
            if TX_FLAG.borrow(cs).get() {
                TX_FLAG.borrow(cs).set(false);
                let transmission_data = data_iter.next();
                if transmission_data != None {
                    if *transmission_data.unwrap() == 0_u8 {
                        tx_pin.set_low();
                    } else {
                        tx_pin.set_high();
                    }
                }
            }
            if RX_FLAG.borrow(cs).get() {
                RX_FLAG.borrow(cs).set(false);
                if rx_pin.is_high() {
                    println!("1");
                } else {
                    println!("0");
                }
            }
        });
    }
}
