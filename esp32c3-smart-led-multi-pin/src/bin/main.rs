#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::gpio::{Input, InputConfig, Level};
use esp_hal::time::{Duration, Instant, Rate};
use esp_hal::{clock::CpuClock, main, rmt::Rmt, rng::Rng};
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
    // // number of LEDs on each wall of the room
    #[cfg(feature = "office_lights")]
    const NUM_LEDS_CLOSET_WALL: usize = 202;
    #[cfg(feature = "office_lights")]
    const NUM_LEDS_WINDOW_WALL: usize = 293;
    #[cfg(feature = "office_lights")]
    const NUM_LEDS_DOOR_WALL: usize = 292;
    #[cfg(feature = "office_lights")]
    const NUM_LEDS_NORTH_WALL: usize = 202;

    //number of LEDs on the test board
    #[cfg(feature = "test_strip")]
    const NUM_LEDS_CLOSET_WALL: usize = 55;
    #[cfg(feature = "test_strip")]
    const NUM_LEDS_WINDOW_WALL: usize = 55;
    #[cfg(feature = "test_strip")]
    const NUM_LEDS_DOOR_WALL: usize = 51;
    #[cfg(feature = "test_strip")]
    const NUM_LEDS_NORTH_WALL: usize = 51;

    // index for LED strip in logical array
    const START_CLOSET_INDEX: usize = 0;
    const START_WINDOW_INDEX: usize = NUM_LEDS_CLOSET_WALL;
    const START_DOOR_INDEX: usize = START_WINDOW_INDEX + NUM_LEDS_WINDOW_WALL;
    #[cfg(feature = "office_lights")]
    const START_NORTH_INDEX: usize = START_DOOR_INDEX + NUM_LEDS_DOOR_WALL;

    const NUM_LEDS_STRIP_CLOSET_WINDOW: usize = NUM_LEDS_CLOSET_WALL + NUM_LEDS_WINDOW_WALL; //strip 1, GPIO6
    const NUM_LEDS_STRIP_DOOR_NORTH: usize = NUM_LEDS_DOOR_WALL + NUM_LEDS_NORTH_WALL; //strip 2, GPIO5
    const NUM_LEDS: usize = NUM_LEDS_STRIP_CLOSET_WINDOW + NUM_LEDS_STRIP_DOOR_NORTH;

    const STRIP_BRIGHTNESS: u8 = 255;

    const BLACK_RAINBOW: &[RGB8] = &[BLACK];
    const TYPICAL_RGB_RAINBOW: &[RGB8] = &[RED, YELLOW, GREEN, DARK_BLUE, DARK_MAGENTA];
    const HOMEMADE_OKLCH_RAINBOW: &[RGB8] = &[
        RGB8 { r: 252, g: 059, b: 041 },
        RGB8 { r: 252, g: 059, b: 041 },
        RGB8 { r: 244, g: 075, b: 028 },
        RGB8 { r: 236, g: 087, b: 019 },
        RGB8 { r: 228, g: 097, b: 017 },
        RGB8 { r: 219, g: 107, b: 000 },
        RGB8 { r: 208, g: 116, b: 000 },
        RGB8 { r: 197, g: 125, b: 000 },
        RGB8 { r: 185, g: 132, b: 000 },
        RGB8 { r: 172, g: 139, b: 013 },
        RGB8 { r: 163, g: 144, b: 000 },
        RGB8 { r: 149, g: 150, b: 000 },
        RGB8 { r: 129, g: 156, b: 000 },
        RGB8 { r: 103, g: 163, b: 000 },
        RGB8 { r: 059, g: 170, b: 024 },
        RGB8 { r: 000, g: 157, b: 106 },
        RGB8 { r: 000, g: 132, b: 146 },
        RGB8 { r: 000, g: 099, b: 163 },
        RGB8 { r: 002, g: 063, b: 155 },
        RGB8 { r: 074, g: 053, b: 165 },
        RGB8 { r: 115, g: 042, b: 157 },
        RGB8 { r: 150, g: 028, b: 133 },
        RGB8 { r: 179, g: 005, b: 094 },
        RGB8 { r: 205, g: 022, b: 084 },
        RGB8 { r: 229, g: 040, b: 069 },
    ];

    const TWELVE_BIT_OKLCH_RAINBOW: &[RGB8] = &[
        RGB8 { r: 137, g: 019, b: 120 },
        RGB8 { r: 137, g: 019, b: 120 },
        RGB8 { r: 147, g: 026, b: 111 },
        RGB8 { r: 156, g: 034, b: 102 },
        RGB8 { r: 164, g: 043, b: 094 },
        RGB8 { r: 170, g: 051, b: 085 },
        RGB8 { r: 179, g: 064, b: 088 },
        RGB8 { r: 187, g: 077, b: 092 },
        RGB8 { r: 196, g: 089, b: 096 },
        RGB8 { r: 203, g: 101, b: 101 },
        RGB8 { r: 214, g: 112, b: 093 },
        RGB8 { r: 224, g: 124, b: 084 },
        RGB8 { r: 231, g: 137, b: 075 },
        RGB8 { r: 236, g: 151, b: 066 },
        RGB8 { r: 242, g: 166, b: 047 },
        RGB8 { r: 245, g: 182, b: 022 },
        RGB8 { r: 243, g: 200, b: 000 },
        RGB8 { r: 236, g: 219, b: 000 },
        RGB8 { r: 217, g: 220, b: 028 },
        RGB8 { r: 196, g: 220, b: 049 },
        RGB8 { r: 174, g: 220, b: 067 },
        RGB8 { r: 151, g: 219, b: 083 },
        RGB8 { r: 133, g: 220, b: 097 },
        RGB8 { r: 115, g: 221, b: 111 },
        RGB8 { r: 094, g: 221, b: 124 },
        RGB8 { r: 068, g: 221, b: 136 },
        RGB8 { r: 044, g: 218, b: 153 },
        RGB8 { r: 023, g: 213, b: 166 },
        RGB8 { r: 017, g: 208, b: 177 },
        RGB8 { r: 032, g: 203, b: 186 },
        RGB8 { r: 013, g: 199, b: 191 },
        RGB8 { r: 000, g: 195, b: 195 },
        RGB8 { r: 000, g: 190, b: 199 },
        RGB8 { r: 000, g: 185, b: 202 },
        RGB8 { r: 000, g: 177, b: 203 },
        RGB8 { r: 000, g: 169, b: 204 },
        RGB8 { r: 000, g: 161, b: 204 },
        RGB8 { r: 000, g: 152, b: 203 },
        RGB8 { r: 013, g: 140, b: 201 },
        RGB8 { r: 028, g: 127, b: 197 },
        RGB8 { r: 040, g: 115, b: 192 },
        RGB8 { r: 050, g: 101, b: 186 },
        RGB8 { r: 069, g: 089, b: 182 },
        RGB8 { r: 082, g: 076, b: 175 },
        RGB8 { r: 093, g: 064, b: 165 },
        RGB8 { r: 101, g: 050, b: 152 },
        RGB8 { r: 111, g: 044, b: 147 },
        RGB8 { r: 120, g: 037, b: 139 },
        RGB8 { r: 129, g: 029, b: 130 },
    ];

    const TWELVE_BIT_OKLCH_RAINBOW_WEIGHTED: &[RGB8] = &[
        RGB8 { r: 137, g: 019, b: 120 },
        RGB8 { r: 137, g: 019, b: 120 },
        RGB8 { r: 147, g: 026, b: 111 },
        RGB8 { r: 156, g: 034, b: 102 },
        RGB8 { r: 164, g: 043, b: 094 },
        RGB8 { r: 170, g: 051, b: 085 },
        RGB8 { r: 179, g: 064, b: 088 },
        RGB8 { r: 187, g: 077, b: 092 },
        RGB8 { r: 196, g: 089, b: 096 },
        RGB8 { r: 203, g: 101, b: 101 },
        RGB8 { r: 212, g: 110, b: 095 },
        RGB8 { r: 220, g: 119, b: 088 },
        RGB8 { r: 227, g: 129, b: 080 },
        RGB8 { r: 232, g: 140, b: 073 },
        RGB8 { r: 236, g: 151, b: 066 },
        RGB8 { r: 241, g: 161, b: 054 },
        RGB8 { r: 243, g: 171, b: 040 },
        RGB8 { r: 245, g: 182, b: 022 },
        RGB8 { r: 244, g: 194, b: 000 },
        RGB8 { r: 241, g: 207, b: 000 },
        RGB8 { r: 236, g: 219, b: 000 },
        RGB8 { r: 223, g: 220, b: 019 },
        RGB8 { r: 210, g: 220, b: 036 },
        RGB8 { r: 196, g: 220, b: 049 },
        RGB8 { r: 181, g: 220, b: 061 },
        RGB8 { r: 167, g: 220, b: 072 },
        RGB8 { r: 151, g: 219, b: 083 },
        RGB8 { r: 137, g: 220, b: 094 },
        RGB8 { r: 122, g: 221, b: 106 },
        RGB8 { r: 106, g: 221, b: 116 },
        RGB8 { r: 089, g: 221, b: 126 },
        RGB8 { r: 068, g: 221, b: 136 },
        RGB8 { r: 036, g: 216, b: 158 },
        RGB8 { r: 016, g: 210, b: 174 },
        RGB8 { r: 032, g: 203, b: 186 },
        RGB8 { r: 000, g: 195, b: 195 },
        RGB8 { r: 000, g: 185, b: 202 },
        RGB8 { r: 000, g: 152, b: 203 },
        RGB8 { r: 050, g: 101, b: 186 },
        RGB8 { r: 082, g: 076, b: 175 },
        RGB8 { r: 101, g: 050, b: 152 },
        RGB8 { r: 114, g: 042, b: 145 },
        RGB8 { r: 126, g: 032, b: 134 },
    ];
    let r_trig = [BLACK];

    let rainbows = [
        &TYPICAL_RGB_RAINBOW[..],
        &HOMEMADE_OKLCH_RAINBOW[..],
        &TWELVE_BIT_OKLCH_RAINBOW[..],
        &TWELVE_BIT_OKLCH_RAINBOW_WEIGHTED[..],
        &BLACK_RAINBOW[..],
    ];

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let mut rng = Rng::new(peripherals.RNG);
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
    let initial_rainbow = rainbow_iter.nth(3).expect("Iterates forever.");

    #[cfg(feature = "office_lights")]
    // closet wall
    let a1 = &mut Animation::<NUM_LEDS_CLOSET_WALL>::new(ANI_DEFAULT, frame_rate)
        .set_translation_array(default_translation_array(START_CLOSET_INDEX))
        // .set_bg_rainbow(&[RED, DARK_RED], true) //debug colors different for each wall
        .set_bg_rainbow(initial_rainbow, RainbowDir::Forward)
        .set_bg_duration_ns(20_000_000_000, frame_rate)
        .set_bg_subdivisions(2)
        .set_trig_duration_ns(5_000_000_000, frame_rate)
        .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
        .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

    #[cfg(feature = "office_lights")]
    // window wall
    let a2 = &mut Animation::<NUM_LEDS_WINDOW_WALL>::new(ANI_DEFAULT, frame_rate)
        .set_translation_array(default_translation_array(START_WINDOW_INDEX))
        // .set_bg_rainbow(&[BLUE, BLUE_VIOLET], true) //debug colors different for each wall
        .set_bg_rainbow(initial_rainbow, RainbowDir::Forward)
        .set_bg_duration_ns(20_000_000_000, frame_rate)
        .set_bg_subdivisions(2)
        .set_trig_duration_ns(5_000_000_000, frame_rate)
        .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
        .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

    #[cfg(feature = "office_lights")]
    // door wall
    let a3 = &mut Animation::<NUM_LEDS_DOOR_WALL>::new(ANI_DEFAULT, frame_rate)
        .set_translation_array(core::array::from_fn(|i| (START_NORTH_INDEX - 1) - i))
        // .set_bg_rainbow(&[YELLOW, ORANGE], true) //debug colors different for each wall
        .set_bg_rainbow(initial_rainbow, RainbowDir::Forward)
        .set_bg_duration_ns(20_000_000_000, frame_rate)
        .set_bg_subdivisions(2)
        .set_trig_duration_ns(5_000_000_000, frame_rate)
        .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
        .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

    #[cfg(feature = "office_lights")]
    // north wall
    let a4 = &mut Animation::<NUM_LEDS_NORTH_WALL>::new(ANI_DEFAULT, frame_rate)
        .set_translation_array(core::array::from_fn(|i| (NUM_LEDS) - 1 - i))
        // .set_bg_rainbow(&[GREEN, DARK_GREEN], true) //debug colors different for each wall
        .set_bg_rainbow(initial_rainbow, RainbowDir::Forward)
        .set_bg_duration_ns(20_000_000_000, frame_rate)
        .set_bg_subdivisions(2)
        .set_trig_duration_ns(5_000_000_000, frame_rate)
        .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
        .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

    #[cfg(feature = "test_strip")]
    // closet wall
    let a1 = &mut Animation::<NUM_LEDS_CLOSET_WALL>::new(ANI_DEFAULT, frame_rate)
        .set_translation_array(default_translation_array(START_CLOSET_INDEX))
        // .set_bg_rainbow(&[RED, DARK_RED], true) //debug colors different for each wall
        .set_bg_rainbow(rainbows[0], RainbowDir::Forward)
        .set_bg_duration_ns(3_000_000_000, frame_rate)
        .set_bg_subdivisions(1)
        .set_trig_duration_ns(5_000_000_000, frame_rate)
        .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
        .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

    #[cfg(feature = "test_strip")]
    // window wall
    let a2 = &mut Animation::<NUM_LEDS_WINDOW_WALL>::new(ANI_DEFAULT, frame_rate)
        .set_translation_array(core::array::from_fn(|i| (START_DOOR_INDEX - 1) - i))
        // .set_bg_rainbow(&[BLUE, BLUE_VIOLET], true) //debug colors different for each wall
        .set_bg_rainbow(rainbows[1], RainbowDir::Forward)
        .set_bg_duration_ns(3_000_000_000, frame_rate)
        .set_bg_subdivisions(1)
        .set_trig_duration_ns(5_000_000_000, frame_rate)
        .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
        .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

    #[cfg(feature = "test_strip")]
    // door wall
    let a3 = &mut Animation::<NUM_LEDS_DOOR_WALL>::new(ANI_DEFAULT, frame_rate)
        .set_translation_array(default_translation_array(START_DOOR_INDEX))
        // .set_bg_rainbow(&[YELLOW, ORANGE], true) //debug colors different for each wall
        .set_bg_rainbow(rainbows[2], RainbowDir::Forward)
        .set_bg_duration_ns(3_000_000_000, frame_rate)
        .set_bg_subdivisions(1)
        .set_trig_duration_ns(5_000_000_000, frame_rate)
        .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
        .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

    #[cfg(feature = "test_strip")]
    // north wall
    let a4 = &mut Animation::<NUM_LEDS_NORTH_WALL>::new(ANI_DEFAULT, frame_rate)
        .set_translation_array(core::array::from_fn(|i| (NUM_LEDS) - 1 - i))
        // .set_bg_rainbow(&[GREEN, DARK_GREEN], true) //debug colors different for each wall
        .set_bg_rainbow(rainbows[3], RainbowDir::Forward)
        .set_bg_duration_ns(3_000_000_000, frame_rate)
        .set_bg_subdivisions(1)
        .set_trig_duration_ns(5_000_000_000, frame_rate)
        .set_trig_fade_rainbow(&r_trig, RainbowDir::Forward)
        .set_trig_incremental_rainbow(&r_trig, RainbowDir::Forward);

    let animations: [&mut dyn Animatable; _] = [a1, a2, a3, a4];
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
