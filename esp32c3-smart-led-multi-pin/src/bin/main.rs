#![no_std]
#![no_main]

use embassy_executor::Spawner;
use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::gpio::{Input, InputConfig, Level};
use esp_hal::time::{Duration, Instant, Rate};
use esp_hal::{clock::CpuClock, rmt::Rmt, rng::Rng};
use esp_hal_smartled::{SmartLedsAdapter, smart_led_buffer};
use esp_println::println;
use lc::animations::{Animatable, Animation, RainbowDir};
use lc::{LightingController, LogicalStrip, utility::default_translation_array};
use lighting_controller::default_animations::ANI_DEFAULT;
use lighting_controller::{self as lc, animations};
use rgb::RGB8;
use smart_leds::{SmartLedsWrite as _, brightness, colors::*, gamma};

#[cfg(feature = "office_lights")]
use esp32c3_smart_led_multi_pin::office_lights::*;

#[cfg(feature = "test_strip")]
use esp32c3_smart_led_multi_pin::test_strip::*;

use esp32c3_smart_led_multi_pin::default_consts::*;

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    const NUM_LEDS_STRIP_CLOSET_WINDOW: usize = NUM_LEDS_CLOSET_WALL + NUM_LEDS_WINDOW_WALL; //strip 1, GPIO6
    const NUM_LEDS_STRIP_DOOR_NORTH: usize = NUM_LEDS_DOOR_WALL + NUM_LEDS_NORTH_WALL; //strip 2, GPIO5
    const NUM_LEDS: usize = NUM_LEDS_STRIP_CLOSET_WINDOW + NUM_LEDS_STRIP_DOOR_NORTH;

    const STRIP_BRIGHTNESS: u8 = 255;

    let r_trig = [BLACK];

    let mut r_adjustable_color = RGB8 { r: 0xff, g: 0xb5, b: 0x65 };
    let r_adjustable = &[r_adjustable_color];

    let rainbows = [
        &TYPICAL_RGB_RAINBOW[..],
        &HOMEMADE_OKLCH_RAINBOW[..],
        &TWELVE_BIT_OKLCH_RAINBOW[..],
        &TWELVE_BIT_OKLCH_RAINBOW_WEIGHTED[..],
        &KELVIN_2500_RAINBOW[..],
        &r_adjustable[..],
        &BLACK_RAINBOW[..],
    ];

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let rng = Rng::new();
    lc::utility::set_random_seed(rng.random().into()); //set random seed using hardware peripheral

    let frequency = Rate::from_mhz(80);
    let rmt = Rmt::new(peripherals.RMT, frequency).expect("Failed to initialize RMT0");

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

    let mut led_strip_1_buffer = smart_led_buffer!(NUM_LEDS_STRIP_CLOSET_WINDOW);
    let mut led_strip_2_buffer = smart_led_buffer!(NUM_LEDS_STRIP_DOOR_NORTH);

    // This strip has CLOSET_WALL and WINDOW_WALL
    let mut led_strip_1 =
        SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO6, &mut led_strip_1_buffer);

    // This strip has DOOR_WALL and NORTH_WALL
    let mut led_strip_2 =
        SmartLedsAdapter::new(rmt.channel1, peripherals.GPIO5, &mut led_strip_2_buffer);

    let frame_rate = embedded_time::rate::Extensions::Hz(60);
    let frame_rate_in_ticks = Duration::from_micros(16_667_u64);

    let color_buffer = &mut [BLACK; NUM_LEDS];
    let mut ls = LogicalStrip::new(color_buffer);

    let mut bg_durations = [
        20_000_000_000,
        15_000_000_000,
        10_000_000_000,
        5_000_000_000,
        4_000_000_000,
        3_000_000_000,
        2_000_000_000,
        1_000_000_000,
    ]
    .iter()
    .cycle()
    .copied();

    let mut rainbow_iter = rainbows.iter().cycle().copied();
    #[cfg(feature = "office_lights")]
    let (mut a1, mut a2, mut a3, mut a4) = {
        let initial_rainbow = rainbow_iter.nth(3).expect("Iterates forever.");
        // closet wall
        let a1 = Animation::<NUM_LEDS_CLOSET_WALL>::new(ANI_DEFAULT, frame_rate)
            .set_translation_array(default_translation_array(START_CLOSET_INDEX))
            // .set_bg_rainbow(&[RED, DARK_RED], true) //debug colors different for each wall
            .set_bg_rainbow(initial_rainbow, RainbowDir::Forward)
            .set_bg_duration_ns(20_000_000_000, frame_rate)
            .set_bg_subdivisions(2)
            .set_trig_duration_ns(5_000_000_000, frame_rate)
            .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
            .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

        // window wall
        let a2 = Animation::<NUM_LEDS_WINDOW_WALL>::new(ANI_DEFAULT, frame_rate)
            .set_translation_array(default_translation_array(START_WINDOW_INDEX))
            // .set_bg_rainbow(&[BLUE, BLUE_VIOLET], true) //debug colors different for each wall
            .set_bg_rainbow(initial_rainbow, RainbowDir::Forward)
            .set_bg_duration_ns(20_000_000_000, frame_rate)
            .set_bg_subdivisions(2)
            .set_trig_duration_ns(5_000_000_000, frame_rate)
            .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
            .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

        // door wall
        let a3 = Animation::<NUM_LEDS_DOOR_WALL>::new(ANI_DEFAULT, frame_rate)
            .set_translation_array(core::array::from_fn(|i| (START_NORTH_INDEX - 1) - i))
            // .set_bg_rainbow(&[YELLOW, ORANGE], true) //debug colors different for each wall
            .set_bg_rainbow(initial_rainbow, RainbowDir::Forward)
            .set_bg_duration_ns(20_000_000_000, frame_rate)
            .set_bg_subdivisions(2)
            .set_trig_duration_ns(5_000_000_000, frame_rate)
            .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
            .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

        // north wall
        let a4 = Animation::<NUM_LEDS_NORTH_WALL>::new(ANI_DEFAULT, frame_rate)
            .set_translation_array(core::array::from_fn(|i| (NUM_LEDS) - 1 - i))
            // .set_bg_rainbow(&[GREEN, DARK_GREEN], true) //debug colors different for each wall
            .set_bg_rainbow(initial_rainbow, RainbowDir::Forward)
            .set_bg_duration_ns(20_000_000_000, frame_rate)
            .set_bg_subdivisions(2)
            .set_trig_duration_ns(5_000_000_000, frame_rate)
            .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
            .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

        (a1, a2, a3, a4)
    };

    #[cfg(feature = "test_strip")]
    let (mut a1, mut a2, mut a3, mut a4) = {
        // closet wall
        let a1 = Animation::<NUM_LEDS_CLOSET_WALL>::new(ANI_DEFAULT, frame_rate)
            .set_translation_array(default_translation_array(START_CLOSET_INDEX))
            // .set_bg_rainbow(&[RED, DARK_RED], true) //debug colors different for each wall
            .set_bg_rainbow(rainbows[4], RainbowDir::Forward)
            .set_bg_duration_ns(3_000_000_000, frame_rate)
            .set_bg_subdivisions(1)
            .set_trig_duration_ns(5_000_000_000, frame_rate)
            .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
            .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

        // window wall
        let a2 = Animation::<NUM_LEDS_WINDOW_WALL>::new(ANI_DEFAULT, frame_rate)
            .set_translation_array(core::array::from_fn(|i| (START_DOOR_INDEX - 1) - i))
            // .set_bg_rainbow(&[BLUE, BLUE_VIOLET], true) //debug colors different for each wall
            .set_bg_rainbow(rainbows[1], RainbowDir::Forward)
            .set_bg_duration_ns(3_000_000_000, frame_rate)
            .set_bg_subdivisions(1)
            .set_trig_duration_ns(5_000_000_000, frame_rate)
            .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
            .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

        // door wall
        let a3 = Animation::<NUM_LEDS_DOOR_WALL>::new(ANI_DEFAULT, frame_rate)
            .set_translation_array(default_translation_array(START_DOOR_INDEX))
            // .set_bg_rainbow(&[YELLOW, ORANGE], true) //debug colors different for each wall
            .set_bg_rainbow(rainbows[2], RainbowDir::Forward)
            .set_bg_duration_ns(3_000_000_000, frame_rate)
            .set_bg_subdivisions(1)
            .set_trig_duration_ns(5_000_000_000, frame_rate)
            .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
            .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

        // north wall
        let a4 = Animation::<NUM_LEDS_NORTH_WALL>::new(ANI_DEFAULT, frame_rate)
            .set_translation_array(core::array::from_fn(|i| (NUM_LEDS) - 1 - i))
            // .set_bg_rainbow(&[GREEN, DARK_GREEN], true) //debug colors different for each wall
            .set_bg_rainbow(rainbows[3], RainbowDir::Forward)
            .set_bg_duration_ns(3_000_000_000, frame_rate)
            .set_bg_subdivisions(1)
            .set_trig_duration_ns(5_000_000_000, frame_rate)
            .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
            .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

        (a1, a2, a3, a4)
    };

    let animations: [&mut dyn Animatable; _] = [&mut a1, &mut a2, &mut a3, &mut a4];
    let mut lc = LightingController::new(animations, frame_rate);

    println!("Peripherals configured, entering main loop.");

    let mut last_update_time = Instant::now();

    loop {
        // Button 0 Updates:
        if Instant::now() > (last_button_0_sample_time + BUTTON_DEBOUNCE_TIME) {
            let current_button_0_level = button_0.level();
            if (current_button_0_level == Level::Low) && (last_button_0_level == Level::High) {
                println!("New Rainbow!");
                let next_rainbow = rainbow_iter.next().expect("Iterates forever.");
                lc.animations[0].update_bg_rainbow(next_rainbow, RainbowDir::Forward);
                lc.animations[1].update_bg_rainbow(next_rainbow, RainbowDir::Forward);
                lc.animations[2].update_bg_rainbow(next_rainbow, RainbowDir::Forward);
                lc.animations[3].update_bg_rainbow(next_rainbow, RainbowDir::Forward);
            }
            last_button_0_level = current_button_0_level;
            last_button_0_sample_time = Instant::now();
        }

        // Button 1 Updates:
        if Instant::now() > (last_button_1_sample_time + BUTTON_DEBOUNCE_TIME) {
            let current_button_1_level = button_1.level();
            if (current_button_1_level == Level::Low) && (last_button_1_level == Level::High) {
                let rand_num: u16 = (rng.random() & 0xFFFF) as u16;
                println!("Random Number Trigger Point: {:X}", rand_num);
                let tp = animations::trigger::Parameters {
                    mode: animations::trigger::Mode::ColorShotFade,
                    direction: animations::Direction::Positive,
                    fade_in_time_ns: 250_000_000_u64,
                    fade_out_time_ns: 1_000_000_000_u64,
                    starting_offset: 0,
                    pixels_per_pixel_group: 1,
                };
                lc.trigger(0, &tp);
                lc.trigger(1, &tp);
                lc.trigger(2, &tp);
                lc.trigger(3, &tp);
            }
            last_button_1_level = current_button_1_level;
            last_button_1_sample_time = Instant::now();
        }

        // Button 2 Updates:
        if Instant::now() > (last_button_2_sample_time + BUTTON_DEBOUNCE_TIME) {
            let current_button_2_level = button_2.level();
            if (current_button_2_level == Level::Low) && (last_button_2_level == Level::High) {
                println!("Press 2!");
                let dur = bg_durations.next().expect("Iterates forever.");
                println!("New Duration: {: >2}s", dur / 1_000_000_000);
                lc.animations[0].update_bg_duration_ns(dur, frame_rate);
                lc.animations[1].update_bg_duration_ns(dur, frame_rate);
                lc.animations[2].update_bg_duration_ns(dur, frame_rate);
                lc.animations[3].update_bg_duration_ns(dur, frame_rate);
            }
            last_button_2_level = current_button_2_level;
            last_button_2_sample_time = Instant::now();
        }

        // Lighting Updates:
        if Instant::now() > (last_update_time + frame_rate_in_ticks) {
            last_update_time = Instant::now();
            lc.update(&mut ls);

            led_strip_1
                .write(brightness(
                    gamma(
                        ls.color_buffer
                            .iter()
                            .take(NUM_LEDS_STRIP_CLOSET_WINDOW)
                            .copied(),
                    ),
                    STRIP_BRIGHTNESS,
                ))
                .unwrap();

            led_strip_2
                .write(brightness(
                    gamma(
                        ls.color_buffer
                            .iter()
                            .skip(NUM_LEDS_STRIP_CLOSET_WINDOW)
                            .copied(),
                    ),
                    STRIP_BRIGHTNESS,
                ))
                .unwrap();
        }
    }
}
