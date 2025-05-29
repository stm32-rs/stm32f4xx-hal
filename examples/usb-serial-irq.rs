//! CDC-ACM serial port example using interrupts.
//! Target board: any STM32F4 with a OTG FS peripheral and a 8MHz HSE crystal
#![no_std]
#![no_main]

use panic_halt as _;
use stm32f4xx_hal::rcc::Config;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
use stm32f4xx_hal::pac::{interrupt, Interrupt};
use stm32f4xx_hal::{pac, prelude::*};
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::SerialPort;

// Make USB serial device globally available
static G_USB_SERIAL: Mutex<RefCell<Option<SerialPort<UsbBus<USB>>>>> =
    Mutex::new(RefCell::new(None));

// Make USB device globally available
static G_USB_DEVICE: Mutex<RefCell<Option<UsbDevice<UsbBus<USB>>>>> =
    Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    static mut EP_MEMORY: [u32; 1024] = [0; 1024];
    static mut USB_BUS: Option<UsbBusAllocator<stm32f4xx_hal::otg_fs::UsbBusType>> = None;

    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp
        .RCC
        .freeze(Config::hsi().sysclk((168).MHz()).pclk1((8).MHz()));

    let gpioa = dp.GPIOA.split();

    let usb = USB::new(
        (dp.OTG_FS_GLOBAL, dp.OTG_FS_DEVICE, dp.OTG_FS_PWRCLK),
        (gpioa.pa11, gpioa.pa12),
        &rcc.clocks,
    );

    *USB_BUS = Some(stm32f4xx_hal::otg_fs::UsbBusType::new(usb, EP_MEMORY));
    let usb_bus = USB_BUS.as_ref().unwrap();

    cortex_m::interrupt::free(|cs| {
        *G_USB_SERIAL.borrow(cs).borrow_mut() = Some(SerialPort::new(usb_bus));

        *G_USB_DEVICE.borrow(cs).borrow_mut() = Some(
            UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x16c0, 0x27dd))
                .device_class(usbd_serial::USB_CLASS_CDC)
                .strings(&[StringDescriptors::default()
                    .manufacturer("Fake Company")
                    .product("Product")
                    .serial_number("TEST")])
                .unwrap()
                .build(),
        );
    });

    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::OTG_FS);
    }

    #[allow(clippy::empty_loop)]
    loop {
        // Do nothing. Everything is done in the IRQ handler
    }
}

#[interrupt]
fn OTG_FS() {
    static mut USB_SERIAL: Option<SerialPort<UsbBus<USB>>> = None;
    static mut USB_DEVICE: Option<UsbDevice<UsbBus<USB>>> = None;

    let usb_dev = USB_DEVICE.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| {
            // Move USB device here, leaving a None in its place
            G_USB_DEVICE.borrow(cs).replace(None).unwrap()
        })
    });

    let serial = USB_SERIAL.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| {
            // Move USB serial device here, leaving a None in its place
            G_USB_SERIAL.borrow(cs).replace(None).unwrap()
        })
    });

    if usb_dev.poll(&mut [serial]) {
        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

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
