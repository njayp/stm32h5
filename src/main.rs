#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::Config;
use embassy_time::{Duration, Timer};
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize the peripherals
    let p = embassy_stm32::init(Config::default());

    // Configure the LED pin as a push-pull output
    let mut led = Output::new(p.PB7, Level::High, Speed::Low);

    loop {
        // Toggle the LED state
        led.toggle();

        // Wait for 500 milliseconds
        Timer::after(Duration::from_millis(500)).await;
    }
}
