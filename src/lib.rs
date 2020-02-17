#![no_std]
// #![deny(missing_docs)]

pub extern crate l3gd20;
pub extern crate lsm303dlhc;
pub extern crate stm32f4xx_hal as hal;

use hal::gpio::gpioa::{PA5, PA6, PA7};
use hal::gpio::gpiob::{PB6, PB9};
use hal::gpio::gpioe::{PE3};
use hal::gpio::{Output, PushPull, Alternate, AF4, AF5};
use hal::i2c::I2c;
use hal::spi::Spi;
use hal::stm32::{I2C1, SPI1};

pub mod led;

/// On board L3GD20 connected to the SPI1 bus via the pins PA5, PA6, PA7 and PE3
pub type L3gd20 = l3gd20::L3gd20<Spi<SPI1, (PA5<AF5>, PA6<AF5>, PA7<AF5>)>, PE3<Output<PushPull>>>;

/// On board LSM303DLHC connected to the I2C1 bus via the PB6 and PB9 pins
pub type Lsm303dlhc = lsm303dlhc::Lsm303dlhc<I2c<I2C1, (PB6<Alternate<AF4>>, PB9<Alternate<AF4>>)>>;
