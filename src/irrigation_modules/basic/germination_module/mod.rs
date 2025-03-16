use esp_backtrace as _;
use esp_hal::prelude::*;
use esp_println::println;

use crate::irrigation_modules::basic::{Program, LOOP_SENTINEL, PROGRAM_RUN_DELAY, TEST_DELAY};

impl Program<'_> {
    pub fn run_germination_module(&mut self) {
        // Program Notes
        //  - (future) wait until 8am
        //  - run the pump for 7 minutes
        //  - disable the pump
        //  - (future) wait until 12pm
        //  - run the pump for 7 minutes
        //  - disable the pump
        //  - (future) wait until 4pm
        //  - run the pump for 7 minutes
        //  - disable the pump
        //  - loop

        // make sure all pumps are off before continuing
        println!("[*] Turning off all pumps\r");
        self.pumps.disable_all_pumps();

        // log
        println!("[*] Basic - Germination Module Starting\r");

        // delay before starting program
        self.delay.delay_ms(PROGRAM_RUN_DELAY);

        let watering_duration: u32 = 7 * 60 * 1000; // 7 minutes in milliseconds
        let daytime_watering_delay: u32 = 4 * 60 * 60 * 1000; // 4 hours in milliseconds
        let nighttime_watering_delay: u32 = 16 * 60 * 60 * 1000; // 4 hours in milliseconds

        // TODO: need to figure out how to determine time of day so this delay is less arbitrary
        loop {
            // turn on the pump
            println!("[*] \t08:00 watering starting\r");
            self.pumps.enable_pump(1);

            // allow the pump to run for the defined watering duration
            self.delay.delay_ms(watering_duration);

            // disable the pump
            println!("[*] \t08:00 watering complete\r");
            self.pumps.disable_pump(1);

            // allow the pump to run for the defined daytime delay
            self.delay.delay_ms(daytime_watering_delay);

            // turn on the pump
            println!("[*] \t12:00 watering starting\r");
            self.pumps.enable_pump(1);

            // allow the pump to run for the defined watering duration
            self.delay.delay_ms(watering_duration);

            // disable the pump
            println!("[*] \t12:00 watering complete\r");
            self.pumps.disable_pump(1);

            // allow the pump to run for the defined daytime delay
            self.delay.delay_ms(daytime_watering_delay);

            // turn on the pump
            println!("[*] \t16:00 watering starting\r");
            self.pumps.enable_pump(1);

            // allow the pump to run for the defined watering duration
            self.delay.delay_ms(watering_duration);

            // disable the pump
            println!("[*] \t16:00 watering complete\r");
            self.pumps.disable_pump(1);

            // allow the pump to run for the defined daytime delay
            self.delay.delay_ms(nighttime_watering_delay);
        }

        // log
        println!("[*] Basic - Germination Module Complete\r");
    }
}
