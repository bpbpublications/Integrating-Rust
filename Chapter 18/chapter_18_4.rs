use arduino_uno::prelude::*;

fn main() {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut pins = dp.PORTB.split();
    let mut led = pins.d13.into_output();

    // Toggle the LED in a loop
    loop {
        led.toggle().unwrap();
        arduino_uno::delay_ms(1000);
    }
}
