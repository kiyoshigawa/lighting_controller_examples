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
    const NUM_LEDS_PER_STRIP: usize = 8;
    const NUM_STRIPS: usize = 2;
    const NUM_LEDS: usize = NUM_STRIPS * NUM_LEDS_PER_STRIP;
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let frequency = Rate::from_mhz(80);
    let rmt = Rmt::new(peripherals.RMT, frequency).expect("Failed to initialize RMT0");

    let mut led_strip_1 = SmartLedsAdapter::new(
        rmt.channel0,
        peripherals.GPIO6,
        smart_led_buffer!(NUM_LEDS_PER_STRIP),
    );
    let mut led_strip_2 = SmartLedsAdapter::new(
        rmt.channel1,
        peripherals.GPIO5,
        smart_led_buffer!(NUM_LEDS_PER_STRIP),
    );

    // These index arrays may the LED strip positions to the ls.color_array values, so you can order the physical LEDs however you want.
    let led_strip_1_index: [usize; NUM_LEDS_PER_STRIP] = [0, 1, 2, 3, 4, 5, 6, 7];
    let led_strip_2_index: [usize; NUM_LEDS_PER_STRIP] = [15, 14, 13, 12, 11, 10, 9, 8];

    const STRIP_BRIGHTNESS: u8 = 150;

    let frame_rate = embedded_time::rate::Extensions::Hz(144);
    let frame_rate_in_ticks = Duration::from_micros(6900u64);
    let color_buffer = &mut [BLACK; NUM_LEDS];
    let mut ls = LogicalStrip::new(color_buffer);
    let a1 = &mut Animation::<NUM_LEDS>::new(default_animations::ANI_TEST, frame_rate);
    let animations: [&mut dyn Animatable; 1] = [a1];
    let mut lc = LightingController::new(animations, frame_rate);

    println!("Peripherals configured, entering main loop.");

    let mut last_update_time = Instant::now();
    loop {
        if Instant::now() > (last_update_time + frame_rate_in_ticks) {
            last_update_time = Instant::now();
            lc.update(&mut ls);

            led_strip_1
                .write(brightness(
                    gamma(
                        led_strip_1_index
                            .map(|i| ls.color_buffer[i])
                            .iter()
                            .copied(),
                    ),
                    STRIP_BRIGHTNESS,
                ))
                .unwrap();

            led_strip_2
                .write(brightness(
                    gamma(
                        led_strip_2_index
                            .map(|i| ls.color_buffer[i])
                            .iter()
                            .copied(),
                    ),
                    STRIP_BRIGHTNESS,
                ))
                .unwrap();
        }
    }
}
