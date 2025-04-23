#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    Config,
    delay::Delay,
    i2c::{self, master::I2c},
    init, main,
    time::RateExtU32,
};
use esp_println::println;
use nobcd::BcdNumber;

const DS1307_ADDR: u8 = 0x68;

#[repr(u8)]
enum DS1307 {
    Seconds,
    Minutes,
    Hours,
    Day,
    Date,
    Month,
    Year,
}

enum DAY {
    Sun = 1,
    Mon = 2,
    Tues = 3,
    Wed = 4,
    Thurs = 5,
    Fri = 6,
}

struct DateTime {
    sec: u8,
    min: u8,
    hrs: u8,
    day: u8,
    date: u8,
    mnth: u8,
    yr: u8,
}

fn ds1307_write<'a>(ds1307: &mut I2c<'a, esp_hal::Blocking>, data_addr: DS1307, data: u8) {
    let data_bcd: [u8; 1] = BcdNumber::new(data).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[data_addr as u8, data_bcd[0]])
        .unwrap();
}

fn bcd_conversion(data: [u8; 1]) -> u8 {
    BcdNumber::from_bcd_bytes(data).unwrap().value::<u8>()
}

#[main]
fn main() -> ! {
    let peripherals = init(Config::default());
    let delay = Delay::new();
    let mut ds1307: I2c<'_, esp_hal::Blocking> = I2c::new(
        peripherals.I2C0,
        i2c::master::Config::default().with_frequency(RateExtU32::kHz(100)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO2)
    .with_sda(peripherals.GPIO3);

    let start_dt = DateTime {
        sec: 30,
        min: 30,
        hrs: 10,
        day: DAY::Wed as u8,
        date: 23,
        mnth: 4,
        yr: 25,
    };

    ds1307_write(&mut ds1307, DS1307::Seconds, start_dt.sec);
    ds1307_write(&mut ds1307, DS1307::Minutes, start_dt.min);
    ds1307_write(&mut ds1307, DS1307::Hours, start_dt.hrs);
    ds1307_write(&mut ds1307, DS1307::Day, start_dt.day);
    ds1307_write(&mut ds1307, DS1307::Date, start_dt.date);
    ds1307_write(&mut ds1307, DS1307::Month, start_dt.mnth);
    ds1307_write(&mut ds1307, DS1307::Year, start_dt.yr);

    loop {
        let mut data: [u8; 7] = [0_u8; 7];
        ds1307.write_read(DS1307_ADDR, &[0_u8], &mut data).unwrap();

        println!("{:?}", data);

        let secs = bcd_conversion([data[0] & 0x7f]);
        let mins = bcd_conversion([data[1]]);
        let hrs = bcd_conversion([data[2] & 0x3f]);
        let dom = bcd_conversion([data[4]]);
        let mnth = bcd_conversion([data[5]]);
        let yr = bcd_conversion([data[6]]);
        let dow = match bcd_conversion([data[3]]) {
            1 => "Sunday",
            2 => "Monday",
            3 => "Tuesday",
            4 => "Wednesday",
            5 => "Thursday",
            6 => "Friday",
            7 => "Saturday",
            _ => "",
        };

        println!(
            "{}, {}/{}/20{}, {:02}:{:02}:{:02}",
            dow, dom, mnth, yr, hrs, mins, secs
        );

        delay.delay_millis(1000_u32);
    }
}
