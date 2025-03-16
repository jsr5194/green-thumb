use esp_backtrace as _;
use esp_hal::{gpio, prelude::*};

use crate::PumpPin;

pub struct Pumps<'a> {
    pub one: &'a mut gpio::GpioPin<gpio::Output<gpio::PushPull>, { PumpPin::One as u8 }>,
    pub two: &'a mut gpio::GpioPin<gpio::Output<gpio::PushPull>, { PumpPin::Two as u8 }>,
    pub three: &'a mut gpio::GpioPin<gpio::Output<gpio::PushPull>, { PumpPin::Three as u8 }>,
    pub four: &'a mut gpio::GpioPin<gpio::Output<gpio::PushPull>, { PumpPin::Four as u8 }>,
    pub five: &'a mut gpio::GpioPin<gpio::Output<gpio::PushPull>, { PumpPin::Five as u8 }>,
    pub six: &'a mut gpio::GpioPin<gpio::Output<gpio::PushPull>, { PumpPin::Six as u8 }>,
    pub seven: &'a mut gpio::GpioPin<gpio::Output<gpio::PushPull>, { PumpPin::Seven as u8 }>,
    pub eight: &'a mut gpio::GpioPin<gpio::Output<gpio::PushPull>, { PumpPin::Eight as u8 }>,
}

impl Pumps<'_> {
    pub fn get_pump_count(&self) -> u32 {
        8
    }

    pub fn enable_all_pumps(&mut self) {
        self.one.set_high().unwrap();
        self.two.set_high().unwrap();
        self.three.set_high().unwrap();
        self.four.set_high().unwrap();
        self.five.set_high().unwrap();
        self.six.set_high().unwrap();
        self.seven.set_high().unwrap();
        self.eight.set_high().unwrap();
    }

    pub fn disable_all_pumps(&mut self) {
        self.one.set_low().unwrap();
        self.two.set_low().unwrap();
        self.three.set_low().unwrap();
        self.four.set_low().unwrap();
        self.five.set_low().unwrap();
        self.six.set_low().unwrap();
        self.seven.set_low().unwrap();
        self.eight.set_low().unwrap();
    }

    pub fn enable_pump(&mut self, pump_id: u32) {
        match pump_id {
            1 => self.one.set_high().unwrap(),
            2 => self.two.set_high().unwrap(),
            3 => self.three.set_high().unwrap(),
            4 => self.four.set_high().unwrap(),
            5 => self.five.set_high().unwrap(),
            6 => self.six.set_high().unwrap(),
            7 => self.seven.set_high().unwrap(),
            8 => self.eight.set_high().unwrap(),
            _ => panic!("Invalid pump id provided"),
        }
    }

    pub fn disable_pump(&mut self, pump_id: u32) {
        match pump_id {
            1 => self.one.set_low().unwrap(),
            2 => self.two.set_low().unwrap(),
            3 => self.three.set_low().unwrap(),
            4 => self.four.set_low().unwrap(),
            5 => self.five.set_low().unwrap(),
            6 => self.six.set_low().unwrap(),
            7 => self.seven.set_low().unwrap(),
            8 => self.eight.set_low().unwrap(),
            _ => panic!("Invalid pump id provided"),
        }
    }
}
