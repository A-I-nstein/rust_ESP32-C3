#![no_std]
#![no_main]

use core::cell::{Cell, RefCell};
use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::handler;
use esp_hal::interrupt::InterruptConfigurable;
use esp_hal::{
    gpio::{Event, Input, Io, Pull},
    main,
};
use esp_println::println;

static G_PIN: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));
static G_FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

#[handler]
fn gpio() {
    critical_section::with(|cs| {
        G_PIN.borrow_ref_mut(cs).as_mut().unwrap().clear_interrupt();
        G_FLAG.borrow(cs).set(true);
    });
}

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let mut io = Io::new(peripherals.IO_MUX);

    io.set_interrupt_handler(gpio);
    let mut some_pin = Input::new(peripherals.GPIO0, Pull::Up);
    some_pin.listen(Event::FallingEdge);
    critical_section::with(|cs| G_PIN.borrow_ref_mut(cs).replace(some_pin));

    let mut count = 0_u32;
    loop {
        critical_section::with(|cs| {
            if G_FLAG.borrow(cs).get() {
                G_FLAG.borrow(cs).set(false);
                count += 1;
                println!("Button press count = {}", count);
            }
        });
    }
}
