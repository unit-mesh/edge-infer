use rust_gpiozero::*;

fn main() {
    // Create a new LED attached to Pin 17
    let mut led = LED::new(17);

    led.blink(1.0, 1.0);

    led.wait();
}
