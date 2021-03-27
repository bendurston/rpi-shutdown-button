extern crate rust_gpiozero;
use std::process;
use std::env;
use rust_gpiozero::{Button, LED};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        println!("Program takes exactly 2 arguments.");
        process::exit(1);
    }

    let led_pin: u8 = args[2].parse::<u8>().unwrap();
    let button_pin: u8 = args[3].parse::<u8>().unwrap();

    set_led(led_pin);
    wait_for_button_press(button_pin);
    shutdown();
}

fn set_led(pin: u8) {
    let led = LED::new(pin);
    led.on();
}

fn wait_for_button_press(pin: u8) {
    let mut button = Button::new(pin);
    Button::wait_for_press(&mut button, None);

}

fn shutdown() {
    let _output = process::Command::new("shutdown")
            .args(&["-h", "now"])
            .output()
            .expect("Failed to shutdown.");
}

