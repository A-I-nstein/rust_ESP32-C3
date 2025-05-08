#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pipe::Pipe};
use esp_backtrace as _;
use esp_hal::{
    Async, Config, init,
    timer::timg::TimerGroup,
    uart::{self, AtCmdConfig, RxConfig, Uart, UartRx, UartTx},
};
use esp_println::println;


const READ_BUF_SIZE: usize = 64;
const AT_CMD: u8 = 0x0D;

static DATAPIPE: Pipe<CriticalSectionRawMutex, READ_BUF_SIZE> = Pipe::new();

#[embassy_executor::task]
async fn uart_writer(mut tx: UartTx<'static, Async>) {
    let mut wbuf: [u8; READ_BUF_SIZE] = [0u8; READ_BUF_SIZE];
    loop {
        DATAPIPE.read(&mut wbuf).await;
        println!("Sending Letter");
        tx.write_async(&wbuf).await.unwrap();
        println!("Sending New Line");
        tx.write_async(&[0x0D, 0x0A]).await.unwrap();
        println!("Flushing");
        tx.flush_async().await.unwrap();
    }
}

#[embassy_executor::task]
async fn uart_reader(mut rx: UartRx<'static, Async>) {
    let mut rbuf: [u8; READ_BUF_SIZE] = [0u8; READ_BUF_SIZE];
    loop {
        let r = rx.read_async(&mut rbuf[0..]).await;
        match r {
            Ok(len) => {
                DATAPIPE.write_all(&rbuf[..len]).await;
            }
            Err(e) => {
                println!("RX Error: {:?}", e);
            }
        }
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = init(Config::default());
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let (tx_pin, rx_pin) = (peripherals.GPIO21, peripherals.GPIO20);
    let config = uart::Config::default()
        .with_rx(RxConfig::default().with_fifo_full_threshold(READ_BUF_SIZE as u16));
    let mut uart0 = Uart::new(peripherals.UART0, config)
        .unwrap()
        .with_tx(tx_pin)
        .with_rx(rx_pin)
        .into_async();
    uart0.set_at_cmd(AtCmdConfig::default().with_cmd_char(AT_CMD));

    let (rx, tx) = uart0.split();
    spawner.spawn(uart_reader(rx)).ok();
    spawner.spawn(uart_writer(tx)).ok();
}
