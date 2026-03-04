//! CDC-ACM serial port example using USB OTG FS.
//!
//! Creates a virtual serial port that echoes received characters back.
//! Connect via USB cable to the micro-USB OTG connector on the board.
//!
//! # Usage
//!
//! ```bash
//! # Build and flash
//! cargo run --release --example usb_cdc_serial --features usb_fs
//!
//! # Connect to serial port (macOS/Linux)
//! screen /dev/tty.usbmodem* 115200
//! # or
//! picocom /dev/ttyACM0 -b 115200
//! ```
//!
//! Type characters and they will be echoed back in uppercase.

#![no_std]
#![no_main]

use panic_probe as _;

use cortex_m_rt::entry;
use defmt_rtt as _;
use static_cell::ConstStaticCell;
use stm32f469i_disc::{hal, hal::pac, hal::prelude::*, usb};

use hal::otg_fs::UsbBus;
use usb_device::prelude::*;

// Statically allocate endpoint memory for USB peripheral
// 1024 words (4KB) is sufficient for CDC-ACM
static EP_MEMORY: ConstStaticCell<[u32; 1024]> = ConstStaticCell::new([0; 1024]);

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks: 8MHz HSE -> 168MHz SYSCLK, 48MHz PLL48CLK for USB
    // The 48MHz PLL48CLK is REQUIRED for USB operation
    let mut rcc = dp.RCC.freeze(
        hal::rcc::Config::hse(8.MHz())
            .sysclk(168.MHz())
            .require_pll48clk(),
    );

    defmt::info!("USB CDC Serial Example Starting");

    let gpioa = dp.GPIOA.split(&mut rcc);

    // Initialize USB using BSP helper
    // PA11 = USB DM, PA12 = USB DP
    let usb = usb::init(
        (dp.OTG_FS_GLOBAL, dp.OTG_FS_DEVICE, dp.OTG_FS_PWRCLK),
        gpioa.pa11,
        gpioa.pa12,
        &rcc.clocks,
    );

    defmt::info!("USB peripheral initialized");

    // Create USB bus with endpoint memory
    let usb_bus = UsbBus::new(usb, EP_MEMORY.take());

    // Create CDC-ACM serial port
    let mut serial = usbd_serial::SerialPort::new(&usb_bus);

    // Create USB device with VID/PID for CDC-ACM
    // 0x16c0:0x27dd is a common VID/PID for test devices (van Ooijen Technische Informatica)
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .device_class(usbd_serial::USB_CLASS_CDC)
        .strings(&[StringDescriptors::default()
            .manufacturer("STM32F469")
            .product("CDC Serial")
            .serial_number("DISCO1")])
        .unwrap()
        .build();

    defmt::info!("USB device created, waiting for connection");

    let mut buf = [0u8; 64];

    loop {
        // Poll USB device - returns true if there is activity
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        // Try to read data from the serial port
        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                defmt::trace!("Received {} bytes", count);

                // Echo back in uppercase
                for byte in buf[..count].iter_mut() {
                    if byte.is_ascii_lowercase() {
                        *byte &= !0x20; // Convert to uppercase
                    }
                }

                // Write back - may need multiple attempts if buffer is full
                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
