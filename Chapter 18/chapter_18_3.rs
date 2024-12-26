use arduino_uno::prelude::*;

// Entry point
fn main() {
    // Initialize peripherals
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut pins = dp.PORTB.split();

    // Configure pin 13 as an output
    let mut led = pins.d13.into_output();

    // Main application loop
    loop {
        // Toggle the LED
        led.toggle().unwrap();
        // Delay for a while
        arduino_uno::delay_ms(1000);
    }
}
