//! On-board user LEDs

use crate::hal::gpio::{
    gpiod::{self, PD4, PD5},
    gpiog::{self, PG6},
    gpiok::{self, PK3},
    ErasedPin,
};

use crate::hal::gpio::{Output, PushPull};

/// Green LED
pub type LD1 = PG6<Output<PushPull>>;

/// Orange LED
pub type LD2 = PD4<Output<PushPull>>;

/// Red LED
pub type LD3 = PD5<Output<PushPull>>;

/// Blue LED
pub type LD4 = PK3<Output<PushPull>>;

/// User LED colors
pub enum LedColor {
    Green,
    Orange,
    Red,
    Blue,
}

// All user LEDs
pub struct Leds {
    leds: [Led; 4],
}

impl Leds {
    pub fn new(
        gpiod: gpiod::Parts,
        gpiog: gpiog::Parts,
        gpiok: gpiok::Parts,
    ) -> Self {
        let green = gpiog.pg6.into_push_pull_output();
        let orange = gpiod.pd4.into_push_pull_output();
        let red = gpiod.pd5.into_push_pull_output();
        let blue = gpiok.pk3.into_push_pull_output();

        Leds {
            leds: [green.into(), orange.into(), red.into(), blue.into()],
        }
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Led> {
        self.leds.iter_mut()
    }
}

impl core::ops::Deref for Leds {
    type Target = [Led];

    fn deref(&self) -> &[Led] {
        &self.leds
    }
}

impl core::ops::DerefMut for Leds {
    fn deref_mut(&mut self) -> &mut [Led] {
        &mut self.leds
    }
}

impl core::ops::Index<usize> for Leds {
    type Output = Led;

    fn index(&self, i: usize) -> &Led {
        &self.leds[i]
    }
}

impl core::ops::Index<LedColor> for Leds {
    type Output = Led;

    fn index(&self, c: LedColor) -> &Led {
        &self.leds[c as usize]
    }
}

impl core::ops::IndexMut<usize> for Leds {
    fn index_mut(&mut self, i: usize) -> &mut Led {
        &mut self.leds[i]
    }
}

impl core::ops::IndexMut<LedColor> for Leds {
    fn index_mut(&mut self, c: LedColor) -> &mut Led {
        &mut self.leds[c as usize]
    }
}

/// Individual LED
pub struct Led {
    pin: ErasedPin<Output<PushPull>>,
}

macro_rules! ctor {
	($($ldx:ident),+) => {
		$(
			impl Into<Led> for $ldx {
				fn into(self) -> Led {
					Led {
						pin: self.erase().into_mode::<Output<PushPull>>(),
					}
				}
			}
		)+
	}
}

ctor!(LD1, LD2, LD3, LD4);

impl Led {
    /// Turns the LED off
    pub fn off(&mut self) {
        self.pin.set_low();
    }

    /// Turns the LED on
    pub fn on(&mut self) {
        self.pin.set_high();
    }

    /// Toggles the LED
    pub fn toggle(&mut self) {
        self.pin.toggle();
    }
}
