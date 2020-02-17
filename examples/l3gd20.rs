//! Interfacing the on-board L3GD20 (gyroscope)
#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m::asm;
use cortex_m_rt::entry;
use f411::{
    hal::{prelude::*, stm32, spi::Spi},
    l3gd20,L3gd20,
};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze();
    // TRY the other clock configuration
    // let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.acr);

    let mut gpioa = p.GPIOA.split();
    let mut gpioe = p.GPIOE.split();

    let mut nss = gpioe.pe3.into_push_pull_output();
    nss.set_high();

    let sck = gpioa.pa5.into_alternate_af5();
    let miso = gpioa.pa6.into_alternate_af5();
    let mosi = gpioa.pa7.into_alternate_af5();

    let spi = Spi::spi1(
        p.SPI1,
        (sck, miso, mosi),
        l3gd20::MODE,
        1.mhz().into(),
        clocks,
    );

    // let mut l3gd20 = L3gd20::new(spi, nss).unwrap();

    // // sanity check: the WHO_AM_I register always contains this value
    // assert_eq!(l3gd20.who_am_i().unwrap(), 0xD4);

    // let _m = l3gd20.all().unwrap();

    // // when you reach this breakpoint you'll be able to inspect the variable `_m` which contains the
    // // gyroscope and the temperature sensor readings
    // asm::bkpt();

    loop {}
}
