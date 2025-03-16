#![no_std]
#![no_main]

use core::cell::{Cell, RefCell};
use critical_section::Mutex;
use embedded_storage::{ReadStorage, Storage};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::{Event, Input, PullDown, PullUp},
    interrupt,
    peripherals::{self, Peripherals},
    prelude::*,
    Delay, Rng, IO,
};
use esp_println::println;
use esp_storage::FlashStorage;

mod irrigation_modules;
mod pumps;

enum IrrigationModule {
    Basic,
}

//
// Configuration: TODO: offload this to a better configuration setup at some point
//
const IRRIGATION_MODULE: IrrigationModule = IrrigationModule::Basic;
pub const ERROR_LED_PIN: u8 = 8;
pub enum PumpPin {
    One = 15,
    Two = 23,
    Three = 22,
    Four = 21,
    Five = 3,
    Six = 18,
    Seven = 19,
    Eight = 20,
}

const err_size: usize = 0x01;
const err_addr: u32 = 0x9000;
const reset_count_size: usize = 0x01;
const reset_count_addr: u32 = 0x9001;

// entry point
#[entry]
fn main() -> ! {
    // debug display banner
    println!("--------------------------------------------------------------------------------\r");
    println!("-                               Green  Thumb                                   -\r");
    println!("--------------------------------------------------------------------------------\r");
    println!("\r");

    //
    // always increment the reset count first so we have the best chance of catching it on every attempt
    //
    let mut bytes = [0u8; reset_count_size];
    let mut flash = FlashStorage::new();

    // read old reset count
    flash.read(reset_count_addr, &mut bytes).unwrap();

    // increment the reset count
    bytes[0x00] = bytes[0x00].wrapping_add(1);

    // write the new reset count
    flash.write(reset_count_addr, &bytes).unwrap();

    // setup peripherals
    let peripherals = Peripherals::take();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);
    let mut rng = Rng::new(peripherals.RNG);
    let mut err_led = &mut io.pins.gpio8.into_push_pull_output();

    // define pumps
    let mut pumps = pumps::Pumps {
        one: &mut io.pins.gpio15.into_push_pull_output(),
        two: &mut io.pins.gpio23.into_push_pull_output(),
        three: &mut io.pins.gpio22.into_push_pull_output(),
        four: &mut io.pins.gpio21.into_push_pull_output(),
        five: &mut io.pins.gpio3.into_push_pull_output(),
        six: &mut io.pins.gpio18.into_push_pull_output(),
        seven: &mut io.pins.gpio19.into_push_pull_output(),
        eight: &mut io.pins.gpio20.into_push_pull_output(),
    };

    // define program select rotary position
    let psel1 = io.pins.gpio4.into_pull_down_input();
    let psel2 = io.pins.gpio5.into_pull_down_input();
    let psel3 = io.pins.gpio6.into_pull_down_input();
    let psel4 = io.pins.gpio7.into_pull_down_input();
    let psel5 = io.pins.gpio0.into_pull_down_input();
    let psel6 = io.pins.gpio1.into_pull_down_input();
    let psel7 = io.pins.gpio10.into_pull_down_input();
    let psel8 = io.pins.gpio11.into_pull_down_input();
    let psel9 = io.pins.gpio2.into_pull_down_input();

    // ensure that all pumps start out in an off state
    println!("[*] Turning Off All Pumps \r");
    pumps.disable_all_pumps();

    // read the reset count
    flash.read(reset_count_addr, &mut bytes).unwrap();
    println!("[*] Reset Count Before Decision: {:02x?}\r", &bytes[0]);

    // when reset count is greater than or equal to 5, enable all pumps
    if bytes[0x00] >= 0x05 {
        // log the abnormal behavior
        println!("[!] Maintenance mode detected. Turning on all pumps\r");

        // turn on all pumps
        pumps.enable_all_pumps();

        // delay for ten seconds before resetting to allow some wiggle room when quickly pressing the reset
        println!("[*] Delaying for three seconds\r");
        delay.delay_ms(3000u32);

        // set the reset count back to zero
        bytes[0x00] = 0x00;
        flash.write(reset_count_addr, &bytes).unwrap();
        println!("[*] Reset count set back to zero\r");
        println!("[*] Sitting in Maintenance Mode Until Reset");

        // just loop infinitely so the pumps stay on
        loop {}
    } else {
        // delay for ten seconds before running the main irrigation program
        // this is to allow for turning on all pumps via the trigger code
        println!("[*] Delaying for three seconds\r");
        delay.delay_ms(3000u32);

        // set the reset count back to zero when the maintenance code wasn't entered
        bytes[0x00] = 0x00;
        flash.write(reset_count_addr, &bytes).unwrap();
        println!("[*] Reset count set back to zero\r");

        // initialize the selector position to the first program
        let mut sel_pos = 1;

        // run training module based off of the configuration
        match IRRIGATION_MODULE {
            IrrigationModule::Basic => {
                let mut p = irrigation_modules::basic::Program {
                    id: &mut irrigation_modules::basic::DEFAULT_PROGRAM,
                    rng: &mut rng,
                    delay: &mut delay.clone(),
                    pumps: &mut pumps,
                    err_led: &mut err_led,
                    flash: &mut flash,
                };

                //loop {
                // check the selector position and pass the requested program id to the module
                ////if psel1.is_high().unwrap() {
                ////sel_pos = 1;
                ////} else if psel2.is_high().unwrap() {
                ////sel_pos = 2;
                ////} else if psel3.is_high().unwrap() {
                ////sel_pos = 3;
                ////} else if psel4.is_high().unwrap() {
                ////sel_pos = 4;
                ////} else if psel5.is_high().unwrap() {
                ////sel_pos = 5;
                ////} else if psel6.is_high().unwrap() {
                ////sel_pos = 6;
                ////} else if psel7.is_high().unwrap() {
                ////sel_pos = 7;
                ////} else if psel8.is_high().unwrap() {
                ////sel_pos = 8;
                ////} else if psel9.is_high().unwrap() {
                ////sel_pos = 9;
                ////} else {
                ////err_led.set_high();
                ////let err = "[!!!] No selector position was detected. Dying. You may want to create a different default behavior here\r";
                ////println!("{}", err);
                ////panic!("{}", err);
                ////}
                println!("[*] Selector Position: {}\r", sel_pos);

                // check the program flags to see if a new program was selected during the last run
                &p.update_program_id(&sel_pos);

                // run the currently selected program
                &p.run();

                err_led.set_high();
                println!("[*] Setting end of program indicator");
                loop {}
            }
            _ => {
                println!("No Irrigation Module Selected\r");
                loop {}
            }
        }
    }
}
