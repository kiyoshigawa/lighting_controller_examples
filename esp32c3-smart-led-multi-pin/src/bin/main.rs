#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::gpio::{Input, InputConfig, Level};
use esp_hal::time::{Duration, Instant};
use esp_hal::{clock::CpuClock, main, rmt::Rmt, time::Rate};
use esp_hal_smartled::{smart_led_buffer, SmartLedsAdapter};
use esp_println::println;
use lc::animations::{Animatable, Animation};
use lc::{utility::default_translation_array, LightingController, LogicalStrip};
use lighting_controller::default_animations::ANI_DEFAULT;
use lighting_controller::{self as lc, animations};
use smart_leds::{brightness, colors::*, gamma, SmartLedsWrite as _};

#[main]
fn main() -> ! {
    // // number of LEDs on each wall of the room
    // const NUM_LEDS_CLOSET_WALL: usize = 202;
    // const NUM_LEDS_WINDOW_WALL: usize = 293;
    // const NUM_LEDS_DOOR_WALL: usize = 292;
    // const NUM_LEDS_NORTH_WALL: usize = 202;

    //number of LEDs on the test board
    const NUM_LEDS_CLOSET_WALL: usize = 55;
    const NUM_LEDS_WINDOW_WALL: usize = 55;
    const NUM_LEDS_DOOR_WALL: usize = 51;
    const NUM_LEDS_NORTH_WALL: usize = 51;

    // index for LED strip in logical array
    const START_CLOSET_INDEX: usize = 0;
    const START_WINDOW_INDEX: usize = NUM_LEDS_CLOSET_WALL;
    const START_DOOR_INDEX: usize = START_WINDOW_INDEX + NUM_LEDS_WINDOW_WALL;
    const START_NORTH_INDEX: usize = START_DOOR_INDEX + NUM_LEDS_DOOR_WALL;

    const NUM_LEDS_STRIP_CLOSET_WINDOW: usize = NUM_LEDS_CLOSET_WALL + NUM_LEDS_WINDOW_WALL; //strip 1, GPIO6
    const NUM_LEDS_STRIP_DOOR_NORTH: usize = NUM_LEDS_DOOR_WALL + NUM_LEDS_NORTH_WALL; //strip 2, GPIO5
    const NUM_LEDS: usize = NUM_LEDS_STRIP_CLOSET_WINDOW + NUM_LEDS_STRIP_DOOR_NORTH;

    const STRIP_BRIGHTNESS: u8 = 255;

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

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

    // This strip has CLOSET_WALL and WINDOW_WALL
    let mut led_strip_1 = SmartLedsAdapter::new(
        rmt.channel0,
        peripherals.GPIO6,
        smart_led_buffer!(NUM_LEDS_STRIP_CLOSET_WINDOW),
    );

    // This strip has DOOR_WALL and NORTH_WALL
    let mut led_strip_2 = SmartLedsAdapter::new(
        rmt.channel1,
        peripherals.GPIO5,
        smart_led_buffer!(NUM_LEDS_STRIP_DOOR_NORTH),
    );

    let frame_rate = embedded_time::rate::Extensions::Hz(60);
    let frame_rate_in_ticks = Duration::from_micros(16_667_u64);
    let color_buffer = &mut [BLACK; NUM_LEDS];
    let mut ls = LogicalStrip::new(color_buffer);

    // closet wall
    let a1 = &mut Animation::<NUM_LEDS_CLOSET_WALL>::new(ANI_DEFAULT, frame_rate)
        .set_translation_array(default_translation_array(START_CLOSET_INDEX))
        .set_bg_rainbow(&[RED, DARK_RED], true) //debug colors different for each wall
        // .set_bg_rainbow(&[RED, YELLOW, GREEN, DARK_BLUE, DARK_MAGENTA], true)
        .set_bg_duration_ns(20_000_000_000, frame_rate)
        .set_bg_subdivisions(2);

    // window wall
    let a2 = &mut Animation::<NUM_LEDS_WINDOW_WALL>::new(ANI_DEFAULT, frame_rate)
        .set_translation_array(default_translation_array(START_WINDOW_INDEX))
        // .set_bg_rainbow(&[BLUE, BLUE_VIOLET], true) //debug colors different for each wall
        .set_bg_rainbow(&[RED, YELLOW, GREEN, DARK_BLUE, DARK_MAGENTA], true)
        .set_bg_duration_ns(20_000_000_000, frame_rate)
        .set_bg_subdivisions(2);

    // door wall
    let a3 = &mut Animation::<NUM_LEDS_DOOR_WALL>::new(ANI_DEFAULT, frame_rate)
        .set_translation_array(core::array::from_fn(|i| (START_NORTH_INDEX - 1) - i))
        // .set_bg_rainbow(&[YELLOW, ORANGE], true) //debug colors different for each wall
        .set_bg_rainbow(&[RED, YELLOW, GREEN, DARK_BLUE, DARK_MAGENTA], true)
        .set_bg_duration_ns(20_000_000_000, frame_rate)
        .set_bg_subdivisions(2);

    // north wall
    let a4 = &mut Animation::<NUM_LEDS_NORTH_WALL>::new(ANI_DEFAULT, frame_rate)
        .set_translation_array(core::array::from_fn(|i| (NUM_LEDS) - 1 - i))
        // .set_bg_rainbow(&[GREEN, DARK_GREEN], true) //debug colors different for each wall
        .set_bg_rainbow(&[RED, YELLOW, GREEN, DARK_BLUE, DARK_MAGENTA], true)
        .set_bg_duration_ns(20_000_000_000, frame_rate)
        .set_bg_subdivisions(2);

    let animations: [&mut dyn Animatable; _] = [a1, a2, a3, a4];
    let mut lc = LightingController::new(animations, frame_rate);

    println!("Peripherals configured, entering main loop.");

    let mut last_update_time = Instant::now();

    loop {
        // Button 0 Updates:
        if Instant::now() > (last_button_0_sample_time + BUTTON_DEBOUNCE_TIME) {
            let current_button_0_level = button_0.level();
            if (current_button_0_level == Level::Low) && (last_button_0_level == Level::High) {
                println!("Press 0!");
                lc.animations[0].update_bg_duration_ns(5_000_000_000, frame_rate);
            }
            last_button_0_level = current_button_0_level;
            last_button_0_sample_time = Instant::now();
        }

        // Button 1 Updates:
        if Instant::now() > (last_button_1_sample_time + BUTTON_DEBOUNCE_TIME) {
            let current_button_1_level = button_1.level();
            if (current_button_1_level == Level::Low) && (last_button_1_level == Level::High) {
                println!("Press 1!");
            }
            last_button_1_level = current_button_1_level;
            last_button_1_sample_time = Instant::now();
        }

        // Button 2 Updates:
        if Instant::now() > (last_button_2_sample_time + BUTTON_DEBOUNCE_TIME) {
            let current_button_2_level = button_2.level();
            if (current_button_2_level == Level::Low) && (last_button_2_level == Level::High) {
                println!("Press 2!");
                let tp = animations::trigger::Parameters {
                    mode: animations::trigger::Mode::ColorPulseFade,
                    direction: animations::Direction::Positive,
                    fade_in_time_ns: 1_000_000_000_u64,
                    fade_out_time_ns: 1_000_000_000_u64,
                    starting_offset: 0_u16,
                    pixels_per_pixel_group: 2_usize,
                };
                lc.trigger(0, &tp);
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
