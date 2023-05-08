//!  
//! Demonstrates use of I2C bus to configure and use FT6x06 touchscreen controller
//!
//! Hardware Required: STM32F412G-DISCO board or STM32F413H-DISCO board
//!
//! Procedure: Compile this example, load it onto the microcontroller, and run it.
//!
//! Example run command: `cargo run --release --features stm32f412,fsmc_lcd --example display_touch`
//!
//! Expected behavior: The display draws circle with its center around the touch. The co-ordinates of the touch
//! are printed on screen.

#![no_main]
#![no_std]
#![allow(unused_variables)]

use cortex_m;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    fsmc_lcd::{FsmcLcd, LcdPins, Timing},
    gpio::Speed,
    pac,
    prelude::*,
    rcc::Rcc,
};

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};

#[cfg(feature = "stm32f413")]
use stm32f4xx_hal::fmpi2c::FMPI2c;
#[cfg(feature = "stm32f412")]
use stm32f4xx_hal::i2c::I2c;

#[allow(unused_imports)]
use panic_semihosting;

use ft6x06::{long_hard_reset, Ft6X06};

use st7789::*;

/// A simple example to connect to the FT6x06 crate and get the values for
/// x and y positions of touch.
#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Started");

    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc: Rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(100.MHz()).freeze();
    let mut delay = cp.SYST.delay(&clocks);

    let gpiob = p.GPIOB.split();
    let gpioc = p.GPIOC.split();
    let gpiod = p.GPIOD.split();
    let gpioe = p.GPIOE.split();
    let gpiof = p.GPIOF.split();
    let gpiog = p.GPIOG.split();

    // Pins connected to the LCD on the board
    use stm32f4xx_hal::gpio::alt::fsmc as alt;
    let lcd_pins = LcdPins {
        data: (
            gpiod.pd14.into(),
            gpiod.pd15.into(),
            gpiod.pd0.into(),
            gpiod.pd1.into(),
            gpioe.pe7.into(),
            gpioe.pe8.into(),
            gpioe.pe9.into(),
            gpioe.pe10.into(),
            gpioe.pe11.into(),
            gpioe.pe12.into(),
            gpioe.pe13.into(),
            gpioe.pe14.into(),
            gpioe.pe15.into(),
            gpiod.pd8.into(),
            gpiod.pd9.into(),
            gpiod.pd10.into(),
        ),
        address: alt::Address::from(gpiof.pf0),
        read_enable: gpiod.pd4.into(),
        write_enable: gpiod.pd5.into(),
        #[cfg(feature = "stm32f413")]
        chip_select: alt::ChipSelect3::from(gpiog.pg10),
        #[cfg(feature = "stm32f412")]
        chip_select: alt::ChipSelect1::from(gpiod.pd7),
    };

    // Enable backlight
    #[cfg(feature = "stm32f413")]
    let mut backlight_control = gpioe.pe5.into_push_pull_output();

    #[cfg(feature = "stm32f412")]
    let mut backlight_control = gpiof.pf5.into_push_pull_output();

    backlight_control.set_high();

    // Setup the RESET pin
    #[cfg(feature = "stm32f413")]
    let mut lcd_reset = gpiob.pb13.into_push_pull_output().speed(Speed::VeryHigh);

    #[cfg(feature = "stm32f412")]
    let lcd_reset = gpiod.pd11.into_push_pull_output().speed(Speed::VeryHigh);

    #[cfg(feature = "stm32f412")]
    let mut ts_reset = gpiof.pf12.into_push_pull_output().speed(Speed::VeryHigh);

    // Workaround on STM32F413:
    // - On the STM32F413 the touchscreen shares the reset GPIO pin w/ the LCD.
    // - The ST7789 driver uses a fast (10uS) reset.
    // - The touchscreen controller needs 5mS:
    //   https://www.displayfuture.com/Display/datasheet/controller/FT6206.pdf
    //
    // Perform a longer reset here first.
    //
    #[cfg(feature = "stm32f412")]
    long_hard_reset(&mut ts_reset, &mut delay).expect("long hard reset");
    #[cfg(feature = "stm32f413")]
    long_hard_reset(&mut lcd_reset, &mut delay).expect("long hard reset");

    // We're not using the "tearing" signal from the display
    let mut _te = gpiob.pb14.into_floating_input();

    // Set up timing
    let write_timing = Timing::default().data(3).address_setup(3).bus_turnaround(0);
    let read_timing = Timing::default().data(8).address_setup(8).bus_turnaround(0);

    // Initialise FSMC memory provider
    let (_fsmc, interface) = FsmcLcd::new(p.FSMC, lcd_pins, &read_timing, &write_timing);

    // Pass display-interface instance ST7789 driver to setup a new display
    let mut disp = ST7789::new(
        interface,
        Some(lcd_reset),
        Some(backlight_control),
        240,
        240,
    );

    // Initialise the display and clear the screen
    disp.init(&mut delay).unwrap();
    disp.clear(Rgb565::BLACK).unwrap();

    // Orientation set is default. The touch coordinates data changes as per orientation.
    // The touch coordinates and shapes are adjusted accordingly.
    rprintln!("The orientation set is {}", disp.orientation() as u8);

    // Initializing the i2c bus for touchscreen
    rprintln!("Connecting to I2c");

    // Declare the pins for i2c address bus on each board.
    // STM32F412 uses I2c1 type for i2c bus.
    // The pins are mentioned in documentation -um2135-discovery-kit-with-stm32f412zg-mcu-stmicroelectronics
    #[cfg(feature = "stm32f412")]
    let mut i2c = { I2c::new(p.I2C1, (gpiob.pb6, gpiob.pb7), 400.kHz(), &clocks) };

    // STM32F413 uses FMPI2C1 type.
    // The pins are mentioned in documentation -um2135-discovery-kit-with-stm32f413zh-mcu-stmicroelectronics
    #[cfg(feature = "stm32f413")]
    let mut i2c = { FMPI2c::new(p.FMPI2C1, (gpioc.pc6, gpioc.pc7), 400.kHz()) };

    #[cfg(feature = "stm32f412")]
    let ts_int = gpiog.pg5.into_pull_down_input();
    #[cfg(feature = "stm32f413")]
    let ts_int = gpioc.pc1.into_pull_down_input();

    // Create a struct of ft6x06 driver for touchscreen.
    let mut touch = Ft6X06::new(&i2c, 0x38, ts_int).unwrap();

    // Run internal calibration of touchscreen
    let tsc = touch.ts_calibration(&mut i2c, &mut delay);
    match tsc {
        Err(e) => rprintln!("Error {} from ts_calibration", e),
        Ok(u) => rprintln!("ts_calibration returned {}", u),
    }
    rprintln!("If nothing happens - touch the screen!");

    // Loop to get the touch data
    loop {
        let t = touch.detect_touch(&mut i2c);
        let mut num: u8 = 0;
        match t {
            Err(_e) => rprintln!("Error from fetching number of touches"),
            Ok(n) => {
                num = n;
                if num != 0 {
                    rprintln!("Number of touches: {}", num)
                };
            }
        }

        if num > 0 {
            let t = touch.get_touch(&mut i2c, 1);

            match t {
                Err(_e) => rprintln!("Error fetching touch data"),
                Ok(n) => {
                    // Prints the touch coordinates.
                    rprintln!(
                        "Touch: {:>3}x{:>3} - weight: {:>3} misc: {}",
                        n.x,
                        n.y,
                        n.weight,
                        n.misc
                    );

                    // Circle with 1 pixel wide white stroke with top-left point at (10, 20) with a diameter of 3
                    // The touchscreen coordinates are flipped and have an offset. The following conversion gives correct orientation.
                    Circle::new(Point::new(i32::from(n.y), 240 - i32::from(n.x)), 20)
                        .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
                        .draw(&mut disp)
                        .unwrap();
                }
            }
        }
    }
}
