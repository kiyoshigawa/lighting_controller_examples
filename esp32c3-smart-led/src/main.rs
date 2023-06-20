#![no_std]
#![no_main]

use esp32c3_hal::{
    clock::{ClockControl},
    IO,
    peripherals::Peripherals,
    prelude::*,
    PulseControl,
    pulse_control::ClockSource,
    Rtc,
    systimer::SystemTimer,
    timer::TimerGroup,
};
use esp_backtrace as _;
use esp_hal_smartled::{smartLedAdapter, SmartLedsAdapter};
use smart_leds::{brightness, gamma, SmartLedsWrite, colors::*};
use lc::animations::{Animatable, Animation};
use lc::{default_animations, LightingController, LogicalStrip};
use lighting_controller as lc;

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

    let mut led = <smartLedAdapter!(16)>::new(pulse.channel0, io.pins.gpio9);

    let frame_rate = embedded_time::rate::Extensions::Hz(60);
    let frame_rate_in_ticks = SystemTimer::TICKS_PER_SECOND / frame_rate.0 as u64;
    let color_buffer = &mut [BLACK; 16];
    let mut ls = LogicalStrip::new(color_buffer);
    let a1 = &mut Animation::<16>::new(default_animations::ANI_TEST, frame_rate);
    let animations: [&mut dyn Animatable; 1] = [a1];
    let mut lc = LightingController::new(animations, frame_rate);

    esp_println::println!("Peripherals configured, entering main loop.");

    let mut last_update_time = SystemTimer::now();
    loop {
        if SystemTimer::now() > (last_update_time + frame_rate_in_ticks) {
            last_update_time = SystemTimer::now();
            lc.update(&mut ls);
            led.write(brightness(
                gamma(ls.color_buffer.iter().copied()),
                50,
            )).unwrap();
        }
    }
}
