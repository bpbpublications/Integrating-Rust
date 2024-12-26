// Import necessary libraries
use avr_hal::prelude::*;

// Entry point
fn main() {
    // Initialize peripherals
    let dp = atmega328p::Peripherals::take().unwrap();
    let mut pins = atmega328p::Pins::new(dp.PORTB, dp.DDRB, dp.PINB);

    // Configure pin 13 as an output
    let mut led = pins.d13.into_output(&mut pins.ddrb);

    // Main application loop
    loop {
        // Toggle the LED
        led.toggle().unwrap();
        // Delay for a while
        cortex_m::asm::delay(1000000);
    }
}
