#![no_std]
#![no_main]

use core::panic::PanicInfo;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} // Enter an infinite loop to prevent undefined behavior
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PA5, Level::Low, Speed::Low);

    loop {
        led.set_high();
        Timer::after_millis(500).await;
        led.set_low();
        Timer::after_millis(500).await;
    }
}
