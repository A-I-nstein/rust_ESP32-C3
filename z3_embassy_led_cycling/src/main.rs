#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::Timer;
use esp_backtrace as _;
use esp_hal::{
    Config,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    timer::timg::TimerGroup,
};

type ButtonType = Mutex<CriticalSectionRawMutex, Option<Input<'static>>>;

static BLINK_DELAY: AtomicU32 = AtomicU32::new(200_u32);
static BUTTON: ButtonType = Mutex::new(None);

#[embassy_executor::task]
async fn press_button(button: &'static ButtonType) {
    loop {
        {
            let mut button_unlocked = button.lock().await;
            if let Some(button_ref) = button_unlocked.as_mut() {
                button_ref.wait_for_rising_edge().await;
                esp_println::println!("Button Pressed!");
            }
        }
        let del = BLINK_DELAY.load(Ordering::Relaxed);
        if del <= 50_u32 {
            BLINK_DELAY.store(200_u32, Ordering::Relaxed);
            esp_println::println!("Delay is now 200ms");
        } else {
            BLINK_DELAY.store(del - 50_u32, Ordering::Relaxed);
            esp_println::println!("Delay is now {}ms", del - 50_u32);
        }
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(Config::default());
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let delay_but_config = InputConfig::default().with_pull(Pull::Up);
    let delay_but = Input::new(peripherals.GPIO3, delay_but_config);
    {
        *(BUTTON.lock().await) = Some(delay_but);
    }

    let led_array_config = OutputConfig::default();
    let mut leds: [Output; 4] = [
        Output::new(peripherals.GPIO4, Level::Low, led_array_config),
        Output::new(peripherals.GPIO5, Level::Low, led_array_config),
        Output::new(peripherals.GPIO6, Level::Low, led_array_config),
        Output::new(peripherals.GPIO7, Level::Low, led_array_config),
    ];

    spawner.spawn(press_button(&BUTTON)).unwrap();

    loop {
        for led in &mut leds {
            led.set_high();
            Timer::after_millis(BLINK_DELAY.load(Ordering::Relaxed) as u64).await;
            led.set_low();
            Timer::after_millis(100).await;
        }
    }
}
