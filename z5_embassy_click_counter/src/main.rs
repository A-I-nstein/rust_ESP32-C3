#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use esp_backtrace as _;
use esp_hal::{
    Config,
    gpio::{Input, InputConfig, Pull},
    timer::timg::TimerGroup,
};

type ButtonType = Mutex<CriticalSectionRawMutex, Option<Input<'static>>>;

static BUTTON: ButtonType = Mutex::new(None);
static PRESS_COUNTER: AtomicU32 = AtomicU32::new(0_u32);

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(Config::default());
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let button_config = InputConfig::default().with_pull(Pull::Up);
    let counter_button = Input::new(peripherals.GPIO3, button_config);

    *BUTTON.lock().await = Some(counter_button);
    spawner.spawn(press_button(&BUTTON)).unwrap();
}

#[embassy_executor::task]
async fn press_button(button: &'static ButtonType) {
    loop {
        let mut button_unlocked = button.lock().await;
        if let Some(button_ref) = button_unlocked.as_mut() {
            button_ref.wait_for_falling_edge().await;
            let mut count = PRESS_COUNTER.load(Ordering::Relaxed);
            count += 1;
            PRESS_COUNTER.store(count, Ordering::Relaxed);
            esp_println::println!("Button pressed {} times", count);
        }
    }
}
