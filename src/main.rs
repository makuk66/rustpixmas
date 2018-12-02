extern crate rand;
extern crate rppal;

use std::thread;
use std::time::Duration;

use rand::prelude::*;
use rand::Rng;

use rppal::gpio::{Gpio, Level, Mode};
use rppal::system::DeviceInfo;

fn main() {
    let device_info = DeviceInfo::new().unwrap();
    println!(
        "Model: {} (SoC: {})",
        device_info.model(),
        device_info.soc()
    );

    let mut gpio = Gpio::new().unwrap();

    println!("configuring gpio");
    for gpio_led_number in 2..27 {
        gpio.set_mode(gpio_led_number, Mode::Output);
    }

    println!("switching off the lights");
    for gpio_led_number in 2..27 {
        gpio.write(gpio_led_number, Level::Low);
    }

    thread::sleep(Duration::from_secs(1));

    println!("switching on the lights");
    for gpio_led_number in 2..27 {
        gpio.write(gpio_led_number, Level::High);
    }

    thread::sleep(Duration::from_secs(2));

    println!("starting twinkling");
    let seed: [u8; 32] = *b"jingle_bells_jingle_all_the_way!";
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    loop {
        for gpio_led_number in 2..27 {
            let value = match rng.gen_ratio(18, 20) {
                true => Level::High,
                false => Level::Low,
            };
            gpio.write(gpio_led_number, value);
        }
        thread::sleep(Duration::from_millis(200));
    }
}
