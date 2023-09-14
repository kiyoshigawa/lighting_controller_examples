#![no_std]
#![no_main]

use esp32c3_hal::{
    clock::ClockControl, peripherals::Peripherals, prelude::*, pulse_control::ClockSource,
    systimer::SystemTimer, timer::TimerGroup, PulseControl, Rng, Rtc, IO,
};
use esp_backtrace as _;
use esp_hal_smartled::{smartLedAdapter, SmartLedsAdapter};
use lc::animations::{
    background, foreground, trigger, Animatable, Animation, AnimationParameters, Direction,
};
use lc::colors::*;
use lc::{LightingController, LogicalStrip};
use lighting_controller as lc;
use rgb::RGB8;
use smart_leds::{brightness, gamma, SmartLedsWrite};

pub const BG: background::Parameters = background::Parameters {
    mode: background::Mode::FillRainbowRotate,
    rainbow: R_ROYGBIV,
    direction: Direction::Positive,
    is_rainbow_forward: true,
    duration_ns: 5_000_000_000,
    subdivisions: 0,
};

pub const FG: foreground::Parameters = foreground::Parameters {
    mode: foreground::Mode::NoForeground,
    rainbow: R_ROYGBIV,
    direction: Direction::Positive,
    is_rainbow_forward: false,
    duration_ns: 5_000_000_000,
    step_time_ns: 250_000_000,
    subdivisions: 1,
    pixels_per_pixel_group: 1,
};

pub const TRIGGER_GLOBAL_PARAMS: trigger::GlobalParameters = trigger::GlobalParameters {
    rainbow: R_BLACK,
    is_rainbow_forward: true,
    duration_ns: 10_000_000_000,
};

pub const ANI: AnimationParameters = AnimationParameters {
    bg: BG,
    fg: FG,
    trigger: TRIGGER_GLOBAL_PARAMS,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT,
    // the RTC WDT, and the TIMG WDTs.
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // Set up GPIO peripheral doe use with LED output
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Configure RMT peripheral globally
    let pulse = PulseControl::new(
        peripherals.RMT,
        &mut system.peripheral_clock_control,
        ClockSource::APB,
        0,
        0,
        0,
    )
    .unwrap();

    // Set up RNG peripheral for triger RNG seed input:
    let mut rand = Rng::new(peripherals.RNG);

    let mut led = <smartLedAdapter!(16)>::new(pulse.channel0, io.pins.gpio9);

    let frame_rate = embedded_time::rate::Extensions::Hz(144);
    let frame_rate_in_ticks = SystemTimer::TICKS_PER_SECOND / frame_rate.0 as u64;
    let color_buffer = &mut [BLACK_A; 16];
    let mut ls = LogicalStrip::new(color_buffer);
    let a1 = &mut Animation::<16>::new(ANI, frame_rate);
    let animations: [&mut dyn Animatable; 1] = [a1];
    let mut lc = LightingController::new(animations, frame_rate);

    esp_println::println!("Peripherals configured, entering main loop.");

    let mut last_frame_update_time = SystemTimer::now();
    let mut last_trigger_time = SystemTimer::now();

    loop {
        // You need to limit the update calls to match the framerate using hardware timers to
        // get accurate framerates and timing on the animations.
        // 16_000_000 clock cycles is one second
        if SystemTimer::now() > (last_trigger_time + 16_000_000) {
            last_trigger_time = SystemTimer::now();

            let trigger_params: trigger::Parameters = trigger::Parameters {
                mode: trigger::Mode::ColorPulseRainbow,
                direction: Direction::Negative,
                fade_in_time_ns: 1_000_000_000,
                fade_out_time_ns: 1_000_000_000,
                starting_offset: rand.random() as u16,
                pixels_per_pixel_group: 1,
            };

            lc.trigger(0, &trigger_params);
        }
        if SystemTimer::now() > (last_frame_update_time + frame_rate_in_ticks) {
            last_frame_update_time = SystemTimer::now();
            lc.update(&mut ls);
            led.write(brightness(
                gamma(
                    ls.color_buffer
                        .iter()
                        .copied()
                        .map(|c| RGB8::new(c.r, c.g, c.b)),
                ),
                50,
            ))
            .unwrap();
        }
    }
}
