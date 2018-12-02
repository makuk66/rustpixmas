
extern crate rppal;
extern crate rand;

use std::thread;
use std::time::Duration;

use rand::{thread_rng, Rng};

use rppal::gpio::{Gpio, Mode, Level};
use rppal::system::DeviceInfo;

fn main() {
    let device_info = DeviceInfo::new().unwrap();
    println!("Model: {} (SoC: {})", device_info.model(), device_info.soc());

    let mut gpio = Gpio::new().unwrap();

    for gpio_led_number in 2..27 {
        gpio.set_mode(gpio_led_number, Mode::Output);
    }

    let mut rng = thread_rng();
    loop {
        for gpio_led_number in 2..27 {
            let value = match rng.gen_ratio(18, 20) {
                true => Level::High,
                false => Level::Low
            };
            gpio.write(gpio_led_number, value);
        }
        thread::sleep(Duration::from_millis(200));
    }
}
