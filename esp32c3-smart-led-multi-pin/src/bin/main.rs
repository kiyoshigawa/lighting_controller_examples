#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::time::{Duration, Instant};
use esp_hal::{clock::CpuClock, main, rmt::Rmt, time::Rate};
use esp_hal_smartled::{smart_led_buffer, SmartLedsAdapter};
use esp_println::println;
use lc::animations::{background, Animatable, Animation, Direction};
use lc::{
    colors, default_animations, utility::default_translation_array, LightingController,
    LogicalStrip,
};
use lighting_controller::{self as lc, animations};
use smart_leds::{brightness, colors::*, gamma, SmartLedsWrite as _};

#[main]
fn main() -> ! {
    // number of LEDs on each wall of the room
    const NUM_LEDS_CLOSET_WALL: usize = 202;
    const NUM_LEDS_WINDOW_WALL: usize = 293;
    const NUM_LEDS_DOOR_WALL: usize = 292;
    const NUM_LEDS_NORTH_WALL: usize = 202;

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

    let mut a1_params = default_animations::ANI_TEST;
    a1_params.bg = background::Parameters {
        mode: background::Mode::FillRainbowRotate,
        // rainbow: &[RED, DARK_RED], //debug colors different for each wall
        rainbow: colors::R_ROYGBIV,
        direction: Direction::Positive,
        is_rainbow_forward: true,
        duration_ns: 15_000_000_000,
        subdivisions: 2,
    };

    let mut a2_params = default_animations::ANI_TEST;
    a2_params.bg = background::Parameters {
        mode: background::Mode::FillRainbowRotate,
        // rainbow: &[BLUE, BLUE_VIOLET], //debug colors different for each wall
        rainbow: colors::R_ROYGBIV,
        direction: Direction::Positive,
        is_rainbow_forward: true,
        duration_ns: 15_000_000_000,
        subdivisions: 2,
    };

    let mut a3_params = default_animations::ANI_TEST;
    a3_params.bg = background::Parameters {
        mode: background::Mode::FillRainbowRotate,
        // rainbow: &[YELLOW, ORANGE], //debug colors different for each wall
        rainbow: colors::R_ROYGBIV,
        direction: Direction::Positive,
        is_rainbow_forward: true,
        duration_ns: 15_000_000_000,
        subdivisions: 2,
    };

    let mut a4_params = default_animations::ANI_TEST;
    a4_params.bg = background::Parameters {
        mode: background::Mode::FillRainbowRotate,
        // rainbow: &[GREEN, DARK_GREEN], //debug colors different for each wall
        rainbow: colors::R_ROYGBIV,
        direction: Direction::Positive,
        is_rainbow_forward: true,
        duration_ns: 15_000_000_000,
        subdivisions: 2,
    };

    let frame_rate = embedded_time::rate::Extensions::Hz(60);
    let frame_rate_in_ticks = Duration::from_micros(16_667_u64);
    let color_buffer = &mut [BLACK; NUM_LEDS];
    let mut ls = LogicalStrip::new(color_buffer);
    let a1 = &mut Animation::<NUM_LEDS_CLOSET_WALL>::new(a1_params, frame_rate)
        .set_translation_array(default_translation_array(START_CLOSET_INDEX));
    let a2 = &mut Animation::<NUM_LEDS_WINDOW_WALL>::new(a2_params, frame_rate)
        .set_translation_array(default_translation_array(START_WINDOW_INDEX));
    let a3 = &mut Animation::<NUM_LEDS_DOOR_WALL>::new(a3_params, frame_rate)
        .set_translation_array(core::array::from_fn(|i| (START_NORTH_INDEX - 1) - i));
    let a4 = &mut Animation::<NUM_LEDS_NORTH_WALL>::new(a4_params, frame_rate)
        .set_translation_array(core::array::from_fn(|i| (NUM_LEDS) - 1 - i));

    let animations: [&mut dyn Animatable; _] = [a1, a2, a3, a4];
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
