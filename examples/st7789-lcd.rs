//!
//! Demonstrates use of the Flexible Static Memory Controller to interface with an ST7789 LCD
//! controller
//!
//! Hardware required: an STM32F412G-DISCO board
//!
//! Procedure: Compile this example, load it onto the microcontroller, and run it.
//!
//! Example run command: `cargo run --release --features stm32f412,rt,fsmc_lcd --example st7789-lcd`
//!
//! Expected behavior: The display shows a black background with four colored circles. Periodically,
//! the color of each circle changes.
//!
//! Each circle takes a noticeable amount of time to draw, from top to bottom. Because
//! embedded-graphics by default does not buffer anything in memory, it sends one pixel at a time
//! to the LCD controller. The LCD interface can transfer rectangular blocks of pixels more quickly.
//!

#![no_std]
#![no_main]

use core::iter::{Cloned, Cycle};
use core::slice::Iter;

use cortex_m_rt::entry;
use panic_semihosting as _;

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;

use embedded_graphics::primitives::{Circle, PrimitiveStyle};
use st7789::ST7789;
use stm32f4xx_hal::fsmc_lcd::{ChipSelect1, FsmcLcd, LcdPins, Timing};
use stm32f4xx_hal::pac::{CorePeripherals, Peripherals};
use stm32f4xx_hal::prelude::*;

#[entry]
fn main() -> ! {
    let cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    // Make HCLK faster to allow updating the display more quickly
    let clocks = rcc.cfgr.hclk(100.MHz()).freeze();

    let mut delay = cp.SYST.delay(&clocks);

    let gpiod = dp.GPIOD.split();
    let gpioe = dp.GPIOE.split();
    let gpiof = dp.GPIOF.split();

    // Pins connected to the LCD on the 32F412GDISCOVERY board
    let lcd_pins = LcdPins {
        data: (
            gpiod.pd14.into_alternate(),
            gpiod.pd15.into_alternate(),
            gpiod.pd0.into_alternate(),
            gpiod.pd1.into_alternate(),
            gpioe.pe7.into_alternate(),
            gpioe.pe8.into_alternate(),
            gpioe.pe9.into_alternate(),
            gpioe.pe10.into_alternate(),
            gpioe.pe11.into_alternate(),
            gpioe.pe12.into_alternate(),
            gpioe.pe13.into_alternate(),
            gpioe.pe14.into_alternate(),
            gpioe.pe15.into_alternate(),
            gpiod.pd8.into_alternate(),
            gpiod.pd9.into_alternate(),
            gpiod.pd10.into_alternate(),
        ),
        address: gpiof.pf0.into_alternate(),
        read_enable: gpiod.pd4.into_alternate(),
        write_enable: gpiod.pd5.into_alternate(),
        chip_select: ChipSelect1(gpiod.pd7.into_alternate()),
    };
    let lcd_reset = gpiod.pd11.into_push_pull_output();
    let mut backlight_control = gpiof.pf5.into_push_pull_output();

    // Speed up timing settings, assuming HCLK is 100 MHz (1 cycle = 10 nanoseconds)
    // These read timings work to read settings, but slower timings are needed to read from the
    // frame memory.
    // Read timing: RD can go low at the same time as D/C changes and CS goes low.
    // RD must be low for at least 45 ns -> DATAST=8
    // Also, a read cycle must take at least 160 nanoseconds, so set ADDSET=8. This means
    // that a whole read takes 16 HCLK cycles (160 nanoseconds).
    // Bus turnaround time is zero, because no particular interval is required between transactions.
    let read_timing = Timing::default().data(8).address_setup(8).bus_turnaround(0);
    // Write timing: Minimum 10 nanoseconds from when WR goes high to CS goes high, so
    // HCLK can't be faster than 100 MHz.
    // NWE must be low for at least 15 ns -> DATAST=3
    // A write cycle must take at least 66 nanoseconds, so ADDSET=3. This means that a whole
    // write cycle takes 7 HCLK cycles (70 nanoseconds) (an extra HCLK cycle is added after NWE
    // goes high).
    // Bus turnaround time is zero, because no particular interval is required between transactions.
    let write_timing = Timing::default().data(3).address_setup(3).bus_turnaround(0);

    let (_fsmc, interface) = FsmcLcd::new(dp.FSMC, lcd_pins, &read_timing, &write_timing);

    // The 32F412GDISCOVERY board has an FRD154BP2902-CTP LCD. There is no easily available
    // datasheet, so the behavior of this code is based on the working demonstration C code:
    // https://github.com/STMicroelectronics/STM32CubeF4/blob/e084518f363e04344dc37822210a75e87377b200/Drivers/BSP/STM32412G-Discovery/stm32412g_discovery_lcd.c
    // https://github.com/STMicroelectronics/STM32CubeF4/blob/e084518f363e04344dc37822210a75e87377b200/Drivers/BSP/Components/st7789h2/st7789h2.c

    // Add LCD controller driver
    let mut lcd = ST7789::new(interface, lcd_reset, 240, 240);
    // Initialise the display and clear the screen
    lcd.init(&mut delay).unwrap();
    lcd.clear(Rgb565::BLACK).unwrap();

    // Turn on backlight
    backlight_control.set_high();

    // Draw some circles
    let test_colors = [
        Rgb565::new(0x4e >> 3, 0x79 >> 2, 0xa7 >> 3),
        Rgb565::new(0xf2 >> 3, 0x8e >> 2, 0x2b >> 3),
        Rgb565::new(0xe1 >> 3, 0x57 >> 2, 0x59 >> 3),
        Rgb565::new(0x76 >> 3, 0xb7 >> 2, 0xb2 >> 3),
        Rgb565::new(0x59 >> 3, 0xa1 >> 2, 0x4f >> 3),
        Rgb565::new(0xed >> 3, 0xc9 >> 2, 0x48 >> 3),
        Rgb565::new(0xb0 >> 3, 0x7a >> 2, 0xa1 >> 3),
        Rgb565::new(0xff >> 3, 0x9d >> 2, 0xa7 >> 3),
        Rgb565::new(0x9c >> 3, 0x75 >> 2, 0x5f >> 3),
        Rgb565::new(0xba >> 3, 0xb0 >> 2, 0xac >> 3),
    ];
    let center_points = [
        Point::new(70, 70),
        Point::new(170, 70),
        Point::new(170, 170),
        Point::new(70, 170),
    ];
    let mut drawer = ColoredCircleDrawer::new(&center_points, &test_colors);
    loop {
        drawer.draw(&mut lcd).unwrap();
        delay.delay_ms(100u16);
    }
}

/// Draws colored circles of various locations and colors
struct ColoredCircleDrawer<'a> {
    /// Infinite iterator over circle center points
    centers: Cloned<Cycle<Iter<'a, Point>>>,
    /// Infinite iterator over Rgb565 colors
    colors: Cloned<Cycle<Iter<'a, Rgb565>>>,
}

impl<'a> ColoredCircleDrawer<'a> {
    pub fn new(centers: &'a [Point], colors: &'a [Rgb565]) -> Self {
        ColoredCircleDrawer {
            centers: centers.iter().cycle().cloned(),
            colors: colors.iter().cycle().cloned(),
        }
    }

    /// Draws one circle onto a target
    pub fn draw<T>(&mut self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = Rgb565>,
    {
        let center = self.centers.next().unwrap();
        let color = self.colors.next().unwrap();

        Circle::new(center, 50)
            .into_styled(PrimitiveStyle::with_fill(color))
            .draw(target)
    }
}
