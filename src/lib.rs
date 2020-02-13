#![no_std]
// #![deny(missing_docs)]

pub extern crate l3gd20;
pub extern crate lsm303dlhc;
pub extern crate stm32f4xx_hal as hal;

pub mod led;

// // /// On board L3GD20 connected to the SPI1 bus via the pins PA5, PA6, PA7 and PE3
// // pub type L3gd20 = l3gd20::L3gd20<Spi<SPI1, (PA5<AF5>, PA6<AF5>, PA7<AF5>)>, PE3<Output<PushPull>>>;

// // /// On board LSM303DLHC connected to the I2C1 bus via the PB6 and PB7 pins
// // pub type Lsm303dlhc = lsm303dlhc::Lsm303dlhc<I2c<I2C1, (PB6<AF4>, PB7<AF4>)>>;
