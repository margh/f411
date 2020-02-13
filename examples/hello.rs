#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};

use f411::{
    hal::{prelude::*, stm32},
};

#[entry]
fn main() -> ! {
    hprintln!("hello world");
    loop {}
}
