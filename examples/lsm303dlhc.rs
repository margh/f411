#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};

use f411::{
    hal::{prelude::*, stm32, i2c::I2c},
    Lsm303dlhc,
};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze();

    let gpiob = p.GPIOB.split();
    let scl = gpiob.pb6.into_alternate_af4();
    let sda = gpiob.pb9.into_alternate_af4();

    let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), clocks);

    let mut lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

    hprintln!("initialised lsm303dlhc").unwrap();

    loop {
        let accel = lsm303dlhc.accel().unwrap();
        let mag = lsm303dlhc.mag().unwrap();
        let temp = lsm303dlhc.temp().unwrap();

        hprintln!("accel={:?}", accel).unwrap();
        hprintln!("mag={:?}", mag).unwrap();
        hprintln!("temp={:?}", temp).unwrap();
    }
}
