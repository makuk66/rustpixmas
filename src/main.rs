use std::thread;
use std::time::Duration;

use rand::prelude::*;
use rand::Rng;

use rppal::gpio::{Gpio, Level, Mode};
use rppal::system::DeviceInfo;

struct Led {
    gpio_number: u8,
    level: Level,
}

impl Led {
    fn new(gpio: &mut Gpio, gpio_number: u8, mode: Mode) -> Led {
        gpio.set_mode(gpio_number, mode);
        Led {
            gpio_number,
            level: Level::Low,
        }
    }

    fn switch_led(&mut self, gpio: &mut Gpio, level: Level) {
        if level == self.level {
            return;
        }
        gpio.write(self.gpio_number, level);
        self.level = level
    }
}

fn switch_all_leds(gpio: &mut Gpio, leds: &mut Vec<Led>, level: Level) {
    println!("switching the lights to {}", level);
    for led in leds {
        led.switch_led(gpio, level);
    }
}

fn twinkle_all_leds(gpio: &mut Gpio, leds: &mut Vec<Led>) -> ! {
    println!("starting twinkling");
    let mut rng: StdRng = SeedableRng::from_seed(*b"jingle_bells_jingle_all_the_way!");

    loop {
        for led in leds.iter_mut() {
            let level = if rng.gen_ratio(18, 20) {
                Level::High
            } else {
                Level::Low
            };
            led.switch_led(gpio, level)
        }
        thread::sleep(Duration::from_millis(200));
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

    let mut leds = Vec::<Led>::new();
    for gpio_number in 2..27 {
        leds.push(Led::new(&mut gpio, gpio_number, Mode::Output))
    }

    switch_all_leds(&mut gpio, &mut leds, Level::Low);

    thread::sleep(Duration::from_secs(1));

    switch_all_leds(&mut gpio, &mut leds, Level::High);

    thread::sleep(Duration::from_secs(2));

    twinkle_all_leds(&mut gpio, &mut leds);
}
