extern crate rust_gpiozero;
use std::process;
use std::env;
use std::{thread, time};
use rust_gpiozero::{Button, LED};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        println!("Program takes exactly 2 arguments.");
        process::exit(1);
    }

    {
        thread::sleep(time::Duration::from_millis(100));
    }
    
    let led = LED::new(args[1].parse::<u8>().unwrap());
    let mut button = Button::new(args[2].parse::<u8>().unwrap());

    set_led(&led, true);
    wait_for_button_press(&mut button);
    set_led(&led, false);
    {
    shutdown();
    }
}

fn set_led(led: &rust_gpiozero::LED, is_on: bool) {
    if is_on {
        led.on();
    } else {
        led.off();
    }
}

fn wait_for_button_press(button: &mut rust_gpiozero::Button) {
    Button::wait_for_press(button, None);
}

fn shutdown() {
    let _output = process::Command::new("shutdown")
            .args(&["-h", "now"])
            .output()
            .expect("Failed to shutdown.");
}

