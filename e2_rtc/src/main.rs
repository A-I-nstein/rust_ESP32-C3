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

#[main]
fn main() -> ! {
    let peripherals = init(Config::default());
    let delay = Delay::new();
    let mut ds1307 = I2c::new(
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

    let secs: [u8; 1] = BcdNumber::new(start_dt.sec).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Seconds as u8, secs[0]])
        .unwrap();
    let mins: [u8; 1] = BcdNumber::new(start_dt.min).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Minutes as u8, mins[0]])
        .unwrap();
    let hrs: [u8; 1] = BcdNumber::new(start_dt.hrs).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Hours as u8, hrs[0]])
        .unwrap();
    let dow: [u8; 1] = BcdNumber::new(start_dt.day).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Day as u8, dow[0]])
        .unwrap();
    let dom: [u8; 1] = BcdNumber::new(start_dt.date).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Date as u8, dom[0]])
        .unwrap();
    let mnth: [u8; 1] = BcdNumber::new(start_dt.mnth).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Month as u8, mnth[0]])
        .unwrap();
    let yr: [u8; 1] = BcdNumber::new(start_dt.yr).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Year as u8, yr[0]])
        .unwrap();

    loop {
        let mut data: [u8; 7] = [0_u8; 7];

        ds1307.write(DS1307_ADDR, &[0_u8]).unwrap();
        ds1307.read(DS1307_ADDR, &mut data).unwrap();

        println!("{:?}", data);

        let secs = BcdNumber::from_bcd_bytes([data[0] & 0x7f])
            .unwrap()
            .value::<u8>();
        let mins = BcdNumber::from_bcd_bytes([data[1]]).unwrap().value::<u8>();
        let hrs = BcdNumber::from_bcd_bytes([data[2] & 0x3f])
            .unwrap()
            .value::<u8>();
        let dom = BcdNumber::from_bcd_bytes([data[4]]).unwrap().value::<u8>();
        let mnth = BcdNumber::from_bcd_bytes([data[5]]).unwrap().value::<u8>();
        let yr = BcdNumber::from_bcd_bytes([data[6]]).unwrap().value::<u8>();
        let dow = match BcdNumber::from_bcd_bytes([data[3]]).unwrap().value::<u8>() {
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
