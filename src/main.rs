extern crate rand;
extern crate rppal;

use std::thread;
use std::time::Duration;

use rand::prelude::*;
use rand::Rng;

use rppal::gpio::{Gpio, Level, Mode};
use rppal::system::DeviceInfo;

const LED_START: u8 = 2;
const LED_END: u8 = 27;

fn switch_all_leds(gpio: &mut Gpio, level: Level) {
    println!("switching the lights to {}", level);
    for gpio_led_number in LED_START..LED_END {
        gpio.write(gpio_led_number, level);
    }
}

fn configure_leds(gpio: &mut Gpio) {
    println!("configuring gpio");
    for gpio_led_number in LED_START..LED_END {
        gpio.set_mode(gpio_led_number, Mode::Output);
    }
}

fn random_leds(rng: &mut StdRng, gpio: &mut Gpio) {
    for gpio_led_number in LED_START..LED_END {
        let value = match rng.gen_ratio(18, 20) {
            true => Level::High,
            false => Level::Low,
        };
        gpio.write(gpio_led_number, value);
    }
}

fn print_device_info() {
    let device_info = DeviceInfo::new().unwrap();
    println!(
        "Model: {} (SoC: {})",
        device_info.model(),
        device_info.soc()
    );
}

fn main() {
    print_device_info();

    let mut gpio = Gpio::new().unwrap();

    configure_leds(&mut gpio);

    switch_all_leds(&mut gpio, Level::Low);

    thread::sleep(Duration::from_secs(1));

    switch_all_leds(&mut gpio, Level::High);

    thread::sleep(Duration::from_secs(2));

    println!("starting twinkling");
    let mut rng: StdRng = SeedableRng::from_seed(*b"jingle_bells_jingle_all_the_way!");

    loop {
        random_leds(&mut rng, &mut gpio);
        thread::sleep(Duration::from_millis(200));
    }
}
