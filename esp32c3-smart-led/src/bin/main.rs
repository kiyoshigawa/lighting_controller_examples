#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::gpio::{Input, InputConfig, Level};
use esp_hal::rng::Rng;
use esp_hal::time::{Duration, Instant};
use esp_hal::{clock::CpuClock, main, rmt::Rmt, time::Rate};
use esp_hal_smartled::{smart_led_buffer, SmartLedsAdapter};
use esp_println::println;
use lc::animations::{Animatable, Animation, RainbowDir};
use lc::{utility::default_translation_array, LightingController, LogicalStrip};
use lighting_controller::default_animations::ANI_DEFAULT;
use lighting_controller::{self as lc, animations};
use rgb::RGB8;
use smart_leds::{brightness, colors::*, gamma, SmartLedsWrite as _};

#[main]
fn main() -> ! {
    const NUM_LEDS: usize = 46;

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let frequency = Rate::from_mhz(80);
    let rmt = Rmt::new(peripherals.RMT, frequency).expect("Failed to initialize RMT0");
    let mut rng = Rng::new(peripherals.RNG);

    // Setup GPIO Pins for buttons:
    const BUTTON_DEBOUNCE_TIME: Duration = Duration::from_millis(20);
    let button_config = InputConfig::default().with_pull(esp_hal::gpio::Pull::Up);

    let button_0 = Input::new(peripherals.GPIO0, button_config);
    let button_1 = Input::new(peripherals.GPIO1, button_config);
    let button_2 = Input::new(peripherals.GPIO2, button_config);

    let mut last_button_0_level = button_0.level();
    let mut last_button_1_level = button_1.level();
    let mut last_button_2_level = button_2.level();

    let mut last_button_0_sample_time = Instant::now();
    let mut last_button_1_sample_time = Instant::now();
    let mut last_button_2_sample_time = Instant::now();

    let mut led_strip =
        SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO6, smart_led_buffer!(NUM_LEDS));
    const STRIP_BRIGHTNESS: u8 = 255;

    let frame_rate = embedded_time::rate::Extensions::Hz(144);
    let frame_rate_in_ticks = Duration::from_micros(6900u64);

    let color_buffer = &mut [BLACK; NUM_LEDS];
    let mut ls = LogicalStrip::new(color_buffer);

    let mut bg_durations = [
        30_000_000_000,
        20_000_000_000,
        10_000_000_000,
        5_000_000_000,
        1_000_000_000,
    ]
    .iter()
    .cycle()
    .copied();

    let mut subdivisions = [1, 2, 3, NUM_LEDS / 5].iter().cycle().copied();

    let r1 = [RED, YELLOW, LIME, BLUE, MAGENTA];
    let r2 = [RED, BLACK, LIME, BLACK, BLUE, BLACK];
    let r3 = [CYAN, PURPLE, MAGENTA, PURPLE];
    let r4 = [MAGENTA, BLACK, YELLOW, BLACK, CYAN, BLACK, ORANGE, BLACK];
    let r5 = [BLACK];
    let r6 = [
        RGB8 {
            r: 208,
            g: 168,
            b: 0,
        },
        RGB8 {
            r: 0,
            g: 170,
            b: 191,
        },
        RGB8 {
            r: 140,
            g: 83,
            b: 162,
        },
    ];
    let r_trig = [GHOST_WHITE];
    let rainbows = [&r1[..], &r2[..], &r3[..], &r4[..], &r5[..], &r6[..]];

    let mut rainbow_iter = rainbows.iter().cycle().copied();

    let a1 = &mut Animation::<NUM_LEDS>::new(ANI_DEFAULT, frame_rate)
        .set_translation_array(default_translation_array(0))
        // .set_bg_rainbow(&[RED, DARK_RED], true) //debug colors different for each wall
        .set_bg_rainbow(
            rainbow_iter.next().expect("Iterates forever."),
            RainbowDir::Forward,
        )
        .set_bg_duration_ns(bg_durations.nth(2).expect("Iterates forever."), frame_rate)
        .set_bg_subdivisions(subdivisions.nth(0).expect("Iterates forever."))
        .set_trig_duration_ns(5_000_000_000, frame_rate)
        .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
        .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

    let animations: [&mut dyn Animatable; 1] = [a1];
    let mut lc = LightingController::new(animations, frame_rate);

    println!("Peripherals configured, entering main loop.");

    let mut last_update_time = Instant::now();

    loop {
        // Button 0 Updates:
        if Instant::now() > (last_button_0_sample_time + BUTTON_DEBOUNCE_TIME) {
            let current_button_0_level = button_0.level();
            if (current_button_0_level == Level::Low) && (last_button_0_level == Level::High) {
                let dur = bg_durations.next().expect("Iterates forever.");
                println!("New Duration: {: >2}s", dur / 1_000_000_000);
                lc.animations[0].update_bg_duration_ns(dur, frame_rate);
            }
            last_button_0_level = current_button_0_level;
            last_button_0_sample_time = Instant::now();
        }

        // Button 1 Updates:
        if Instant::now() > (last_button_1_sample_time + BUTTON_DEBOUNCE_TIME) {
            let current_button_1_level = button_1.level();
            if (current_button_1_level == Level::Low) && (last_button_1_level == Level::High) {
                // let sub = subdivisions.next().expect("Iterates forever.");
                // println!("New Subdivisions: {:?}", sub);
                // lc.animations[0].update_bg_subdivisions(sub);
                let rand_num: u16 = (rng.random() & 0xFFFF) as u16;
                println!("Random Number Trigger Point: {:X}", rand_num);
                let tp = animations::trigger::Parameters {
                    mode: animations::trigger::Mode::ColorPulseRainbow,
                    direction: animations::Direction::Positive,
                    fade_in_time_ns: 250_000_000,
                    fade_out_time_ns: 500_000_000,
                    starting_offset: rand_num,
                    pixels_per_pixel_group: 1,
                };
                lc.trigger(0, &tp);
            }
            last_button_1_level = current_button_1_level;
            last_button_1_sample_time = Instant::now();
        }

        // Button 2 Updates:
        if Instant::now() > (last_button_2_sample_time + BUTTON_DEBOUNCE_TIME) {
            let current_button_2_level = button_2.level();
            if (current_button_2_level == Level::Low) && (last_button_2_level == Level::High) {
                println!("New Rainbow!");
                lc.animations[0].update_bg_rainbow(
                    rainbow_iter.next().expect("Iterates forever."),
                    RainbowDir::Forward,
                );
            }
            last_button_2_level = current_button_2_level;
            last_button_2_sample_time = Instant::now();
        }

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
