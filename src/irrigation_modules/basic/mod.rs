use esp_backtrace as _;
use esp_hal::{gpio, prelude::*, Delay, Rng};
use esp_println::println;
use esp_storage::FlashStorage;

use crate::ERROR_LED_PIN;

use crate::pumps;
mod germination_module;

// define global constants
pub const DEFAULT_PROGRAM: u32 = 1;
pub const PROGRAM_RUN_DELAY: u32 = 5000; // milliseconds
const PROGRAM_COUNT: usize = 4;
const LOOP_SENTINEL: usize = 10000;
const TEST_DELAY: u32 = 4000; // milliseconds

pub struct Program<'a> {
    pub id: &'a mut u32,
    pub rng: &'a mut Rng,
    pub delay: &'a mut Delay,
    pub pumps: &'a mut pumps::Pumps<'a>,
    pub err_led: &'a mut gpio::GpioPin<gpio::Output<gpio::PushPull>, ERROR_LED_PIN>,
    pub flash: &'a mut FlashStorage,
}

impl Program<'_> {
    // return a random millisecond delay between the given range
    pub fn get_random_ms_delay(&mut self, start: u32, stop: u32) -> u32 {
        if start >= stop {
            panic!("Start value is greater than or equal to stop value\r");
        }

        let mut d: u32 = self.rng.random() % (stop - 1);
        if d < start {
            d = start;
        } else if d > stop {
            d = stop;
        }
        d
    }

    // read the selector to determine what program to run
    pub fn update_program_id(&mut self, id: &u32) {
        match id {
            1 => {
                println!("[*] Instructed to run Program #1\r");
                *self.id = 1;
            }
            _ => panic!("[!!!] Invalid ID passed\r"),
        }
    }

    // dynamically run a selected program
    pub fn run(&mut self) {
        match self.id {
            1 => self.run_germination_module(),
            _ => panic!("invalid id selected\r"),
        }
    }
}
