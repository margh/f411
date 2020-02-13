//! Blinks an LED
#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_std]
#![no_main]

// Panic handler
extern crate panic_semihosting;

use cortex_m_rt::entry;
use f411::{
    hal::{delay::Delay, prelude::*, stm32},
    led::Led,
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();

    // stm32 in this case internally refers to the stm32f411 lib inside the HAL
    let mut peripherals = stm32::Peripherals::take().unwrap();

    let gpiod = peripherals.GPIOD.split();

    let mut led = gpiod.pd12.into_push_pull_output();

    led.set_high();

    loop {}
}
