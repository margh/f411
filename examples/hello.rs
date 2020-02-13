#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

// supplies the interrupt vectors for rt
extern crate stm32f4;

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};

#[entry]
fn main() -> ! {
    hprintln!("hello world");
    loop {}
}
