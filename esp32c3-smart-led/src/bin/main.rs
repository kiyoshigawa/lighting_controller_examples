#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::time::{Duration, Instant};
use esp_hal::{clock::CpuClock, main, rmt::Rmt, time::Rate};
use esp_hal_smartled::{smart_led_buffer, SmartLedsAdapter};
use esp_println::println;
use lc::animations::{Animatable, Animation};
use lc::{default_animations, LightingController, LogicalStrip};
use lighting_controller as lc;
use smart_leds::{brightness, colors::*, gamma, SmartLedsWrite as _};

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let frequency = Rate::from_mhz(80);
    let rmt = Rmt::new(peripherals.RMT, frequency).expect("Failed to initialize RMT0");

    let mut led_strip =
        SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO9, smart_led_buffer!(16));
    const STRIP_BRIGHTNESS: u8 = 150;

    let frame_rate = embedded_time::rate::Extensions::Hz(144);
    let frame_rate_in_ticks = Duration::from_micros(6900u64);
    let color_buffer = &mut [BLACK; 16];
    let mut ls = LogicalStrip::new(color_buffer);
    let a1 = &mut Animation::<16>::new(default_animations::ANI_DEFAULT, frame_rate);
    let animations: [&mut dyn Animatable; 1] = [a1];
    let mut lc = LightingController::new(animations, frame_rate);

    println!("Peripherals configured, entering main loop.");

    let mut last_update_time = Instant::now();
    loop {
        if Instant::now() > (last_update_time + frame_rate_in_ticks) {
            last_update_time = Instant::now();
            lc.update(&mut ls);
            led_strip
                .write(brightness(
                    gamma(ls.color_buffer.iter().copied()),
                    STRIP_BRIGHTNESS,
                ))
                .unwrap();
        }
    }
}
