#![no_std]
#![no_main]

use core::cell::{Cell, RefCell};

use critical_section::{with, Mutex};
use esp_backtrace as _;
use esp_hal::{
    delay::MicrosDurationU64,
    gpio::{Event, Input, Io, Pull},
    handler, init,
    interrupt::InterruptConfigurable,
    main,
    timer::{
        timg::{Timer, TimerGroup},
        Timer as OtherTimer,
    },
    Config,
};
use esp_println::println;

static START_STOP_BUTTON: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));
static RESET_BUTTON: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));
static G_TIMER: Mutex<RefCell<Option<Timer>>> = Mutex::new(RefCell::new(None));
static TIMER_FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static IS_TIMER_RUNNING: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static TIMER_RESET_FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

struct TimerUi {
    seconds: u32,
    minutes: u32,
    hours: u32,
}

impl TimerUi {
    fn new() -> Self {
        return TimerUi {
            seconds: 0_u32,
            minutes: 0_u32,
            hours: 0_u32,
        };
    }

    fn increment(&mut self) {
        self.seconds += 1;
        if self.seconds > 59 {
            self.minutes += 1;
            self.seconds = 0;
        }
        if self.minutes > 59 {
            self.hours += 1;
            self.minutes = 0;
        }
        if self.hours > 23 {
            self.reset();
        }
    }

    fn show_time(&self) {
        println!(
            "Elapsed Time: {:0>2}:{:0>2}:{:0>2}",
            self.hours, self.minutes, self.seconds
        );
    }

    fn reset(&mut self) {
        self.seconds = 0;
        self.minutes = 0;
        self.hours = 0;
    }
}

#[main]
fn main() -> ! {
    let peripherals = init(Config::default());
    let mut start_stop_button = Input::new(peripherals.GPIO0, Pull::Up);
    let mut reset_button = Input::new(peripherals.GPIO1, Pull::Up);
    let mut io = Io::new(peripherals.IO_MUX);
    let timer_group_0 = TimerGroup::new(peripherals.TIMG0);
    let mut timer_0 = timer_group_0.timer0;
    let mut time = TimerUi::new();

    io.set_interrupt_handler(gpio);
    start_stop_button.listen(Event::FallingEdge);
    reset_button.listen(Event::FallingEdge);
    timer_0.set_interrupt_handler(timer_interrupt);

    with(|cs| {
        START_STOP_BUTTON
            .borrow(cs)
            .borrow_mut()
            .replace(start_stop_button);
        RESET_BUTTON.borrow(cs).borrow_mut().replace(reset_button);
        G_TIMER.borrow(cs).borrow_mut().replace(timer_0);
    });

    loop {
        with(|cs| {
            if TIMER_FLAG.borrow(cs).get() {
                TIMER_FLAG.borrow(cs).set(false);
                time.increment();
                time.show_time();
            }
            if TIMER_RESET_FLAG.borrow(cs).get() {
                TIMER_RESET_FLAG.borrow(cs).set(false);
                IS_TIMER_RUNNING.borrow(cs).set(false);
                time.reset();
            }
        });
    }
}

#[handler]
fn gpio() {
    with(|cs| {
        if let Some(start_stop_button) = START_STOP_BUTTON.borrow_ref_mut(cs).as_mut() {
            if let Some(timer_0) = G_TIMER.borrow(cs).borrow().as_ref() {
                if start_stop_button.is_interrupt_set() {
                    if IS_TIMER_RUNNING.borrow(cs).get() {
                        IS_TIMER_RUNNING.borrow(cs).set(false);
                        println!("Timer stopped.");
                        timer_0.stop();
                    } else {
                        println!("Timer started.");
                        timer_0
                            .load_value(MicrosDurationU64::micros(1_000_000))
                            .unwrap();
                        timer_0.enable_interrupt(true);
                        timer_0.reset();
                        IS_TIMER_RUNNING.borrow(cs).set(true);
                    }
                    start_stop_button.clear_interrupt();
                }
            }
        }
        if let Some(reset_button) = RESET_BUTTON.borrow_ref_mut(cs).as_mut() {
            if let Some(timer_0) = G_TIMER.borrow(cs).borrow().as_ref() {
                if reset_button.is_interrupt_set() {
                    println!("Timer Restarted.");
                    timer_0.stop();
                    TIMER_RESET_FLAG.borrow(cs).set(true);
                    reset_button.clear_interrupt();
                }
            }
        }
    });
}

#[handler]
fn timer_interrupt() {
    with(|cs| {
        G_TIMER
            .borrow(cs)
            .borrow_mut()
            .as_ref()
            .unwrap()
            .clear_interrupt();
        TIMER_FLAG.borrow(cs).set(true);
    });
}
