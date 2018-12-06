use std::thread;
use std::time::Duration;

use rand::prelude::*;
use rand::Rng;

use rppal::gpio::{Gpio, Level, Mode};
use rppal::system::DeviceInfo;

fn switch_all_leds(gpio: &mut Gpio, leds: &[u8], level: Level) {
    println!("switching the lights to {}", level);
    for gpio_led_number in leds {
        gpio.write(*gpio_led_number, level);
    }
}

fn configure_leds(gpio: &mut Gpio, leds: &[u8]) {
    println!("configuring gpio");
    for gpio_led_number in leds {
        gpio.set_mode(*gpio_led_number, Mode::Output);
    }
}

fn random_leds(rng: &mut StdRng, gpio: &mut Gpio, leds: &[u8]) {
    for gpio_led_number in leds {
        let value = if rng.gen_ratio(18, 20) {
            Level::High
        } else {
            Level::Low
        };
        gpio.write(*gpio_led_number, value);
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

    let leds: Vec<u8> = (2..27).collect();

    configure_leds(&mut gpio, &leds);

    switch_all_leds(&mut gpio, &leds, Level::Low);

    thread::sleep(Duration::from_secs(1));

    switch_all_leds(&mut gpio, &leds, Level::High);

    thread::sleep(Duration::from_secs(2));

    println!("starting twinkling");
    let mut rng: StdRng = SeedableRng::from_seed(*b"jingle_bells_jingle_all_the_way!");

    loop {
        random_leds(&mut rng, &mut gpio, &leds);
        thread::sleep(Duration::from_millis(200));
    }
}
