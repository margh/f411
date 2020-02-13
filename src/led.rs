// # On-board user LEDs

use core::ops;

use hal::prelude::*;

// use hal::gpio::gpioa::{self, PA, PA9};
use hal::gpio::gpiod::{self, PD, PD5, PD12, PD13, PD14, PD15};
use hal::gpio::{Output, PushPull};

/// ## USB LEDs

/// `LD7:PA9` Green / indicates when VBUS is present CN5
// pub type LD7 = PA9<Output<PushPull>>;

/// `LD8:PD5` Red / indicates an overcurrent from VBUS of CN5
pub type LD8 = PD5<Output<PushPull>>;

/// ## User LEDs

/// `LD3:PD13` Orange / North
pub type LD3 = PD13<Output<PushPull>>;

/// `LD5:PD14` Red / East
pub type LD5 = PD14<Output<PushPull>>;

/// `LD6:PD15` Blue / South
pub type LD6 = PD15<Output<PushPull>>;

/// `LD4:PD12` Green / West
pub type LD4 = PD12<Output<PushPull>>;

/// Cardinal directions. Each one matches one of the user LEDs.
pub enum Direction {
    North,  // LD3
    East,   // LD5
    South,  // LD6
    West,   // LD4
}

/// One of the on-board user LEDs
pub struct Led {
    pex: PD<Output<PushPull>>,
}

/// Array of all the user LEDs on the board
pub struct Leds {
    leds: [Led; 4],
}


impl Leds {
    /// Initializes all the user LEDs
    pub fn new(gpiod: gpiod::Parts) -> Self {
        let n = gpiod.pd13.into_push_pull_output();
        let e = gpiod.pd14.into_push_pull_output();
        let s = gpiod.pd15.into_push_pull_output();
        let w = gpiod.pd12.into_push_pull_output();

        Leds {
            leds: [
                n.into(),
                e.into(),
                s.into(),
                w.into(),
            ],
        }
    }
}

pub struct Compass {
    pub n: Led,
    pub e: Led,
    pub s: Led,
    pub w: Led,
}

impl Compass {
    pub fn new(gpiod: gpiod::Parts) -> Self {
        let n = gpiod.pd13.into_push_pull_output();
        let e = gpiod.pd14.into_push_pull_output();
        let s = gpiod.pd15.into_push_pull_output();
        let w = gpiod.pd12.into_push_pull_output();

        Compass {
            n: n.into(),
            e: e.into(),
            s: s.into(),
            w: w.into(),
        }
    }
}

impl ops::Deref for Leds {
    type Target = [Led];

    fn deref(&self) -> &[Led] {
        &self.leds
    }
}

impl ops::DerefMut for Leds {
    fn deref_mut(&mut self) -> &mut [Led] {
        &mut self.leds
    }
}

impl ops::Index<usize> for Leds {
    type Output = Led;

    fn index(&self, i: usize) -> &Led {
        &self.leds[i]
    }
}

impl ops::Index<Direction> for Leds {
    type Output = Led;

    fn index(&self, d: Direction) -> &Led {
        &self.leds[d as usize]
    }
}

impl ops::IndexMut<usize> for Leds {
    fn index_mut(&mut self, i: usize) -> &mut Led {
        &mut self.leds[i]
    }
}

impl ops::IndexMut<Direction> for Leds {
    fn index_mut(&mut self, d: Direction) -> &mut Led {
        &mut self.leds[d as usize]
    }
}

macro_rules! ctor {
    ($($ldx:ident),+) => {
        $(
            impl Into<Led> for $ldx {
                fn into(self) -> Led {
                    Led {
                        pex: self.downgrade(),
                    }
                }
            }
        )+
    }
}

ctor!(LD3, LD4, LD5, LD6, LD8);

// these return Result<(), Infallible>
// not sure what the correct way to handle a failure is
impl Led {
    /// Turns the LED off
    pub fn off(&mut self) {
        self.pex.set_low().unwrap();
    }

    /// Turns the LED on
    pub fn on(&mut self) {
        self.pex.set_high().unwrap();
    }
}
