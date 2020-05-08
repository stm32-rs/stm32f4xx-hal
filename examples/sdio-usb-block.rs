#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use hal::interrupt;
use hal::sdio::Sdio;
use hal::{prelude::*, stm32};

use cortex_m::peripheral::NVIC;
use stm32f4xx_hal::otg_fs::{UsbBus, UsbBusType, USB};
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use usbd_mass_storage;
use usbd_scsi::{BlockDevice, BlockDeviceError, Scsi};

static mut EP_MEMORY: [u32; 1024] = [0; 1024];
static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;

static USB_DEV: Mutex<RefCell<Option<UsbDevice<UsbBusType>>>> = Mutex::new(RefCell::new(None));
static USB_STORAGE: Mutex<RefCell<Option<usbd_scsi::Scsi<UsbBusType, Storage>>>> =
    Mutex::new(RefCell::new(None));

struct Storage {
    sdio: RefCell<Sdio>,
}

impl BlockDevice for Storage {
    const BLOCK_BYTES: usize = 512;

    fn read_block(&self, lba: u32, block: &mut [u8]) -> Result<(), BlockDeviceError> {
        let mut sdio = self.sdio.borrow_mut();
        sdio.read_block(lba, block).map_err(|e| {
            rprintln!("read error: {:?}", e);
            BlockDeviceError::HardwareError
        })
    }

    fn write_block(&mut self, lba: u32, block: &[u8]) -> Result<(), BlockDeviceError> {
        let mut sdio = self.sdio.borrow_mut();
        sdio.write_block(lba, block).map_err(|e| {
            rprintln!("write error: {:?}", e);
            BlockDeviceError::WriteError
        })
    }

    fn max_lba(&self) -> u32 {
        let sdio = self.sdio.borrow();
        sdio.card().map(|c| c.block_count() - 1).unwrap_or(0)
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!(BlockIfFull);

    let device = stm32::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(12.mhz())
        .require_pll48clk()
        .sysclk(168.mhz())
        .hclk(168.mhz())
        .pclk1(42.mhz())
        .pclk2(84.mhz())
        .freeze();

    assert!(clocks.is_pll48clk_valid());

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(core.SYST, clocks);

    let gpioa = device.GPIOA.split();
    let gpioc = device.GPIOC.split();
    let gpiod = device.GPIOD.split();

    let mut red_led = {
        let mut led = gpioc.pc1.into_push_pull_output();
        let _ = led.set_low().ok();
        led
    };

    let mut sdio = {
        let d0 = gpioc
            .pc8
            .into_push_pull_output()
            .into_alternate_af12()
            .internal_pull_up(true);

        let d1 = gpioc
            .pc9
            .into_push_pull_output()
            .into_alternate_af12()
            .internal_pull_up(true);

        let d2 = gpioc
            .pc10
            .into_push_pull_output()
            .into_alternate_af12()
            .internal_pull_up(true);

        let d3 = gpioc
            .pc11
            .into_push_pull_output()
            .into_alternate_af12()
            .internal_pull_up(true);

        let clk = gpioc
            .pc12
            .into_push_pull_output()
            .into_alternate_af12()
            .internal_pull_up(false);

        let cmd = gpiod
            .pd2
            .into_push_pull_output()
            .into_alternate_af12()
            .internal_pull_up(true);

        Sdio::new(device.SDIO, (clk, cmd, d0, d1, d2, d3))
    };

    rprintln!("Waiting for card...");

    // Loop until we have a card
    loop {
        match sdio.init_card() {
            Ok(_) => break,
            Err(err) => {
                rprintln!("Init err: {:?}", err);
            }
        }

        delay.delay_ms(1000u32);
        red_led.toggle().ok();
    }

    rprintln!("blocks: {:?}", sdio.card().map(|c| c.block_count()));

    let sdhc = Storage {
        sdio: RefCell::new(sdio),
    };

    unsafe {
        let usb = USB {
            usb_global: device.OTG_FS_GLOBAL,
            usb_device: device.OTG_FS_DEVICE,
            usb_pwrclk: device.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate_af10(),
            pin_dp: gpioa.pa12.into_alternate_af10(),
        };

        let usb_bus = UsbBus::new(usb, &mut EP_MEMORY);
        USB_BUS = Some(usb_bus);

        let scsi = Scsi::new(
            USB_BUS.as_ref().unwrap(),
            64,
            sdhc,
            "Fake Co.",
            "Fake product",
            "FK01",
        );

        let usb_dev = UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(), UsbVidPid(0x16c0, 0x27dd))
            .manufacturer("Fake company")
            .product("SdUsb")
            .serial_number("TEST")
            .self_powered(true)
            .device_class(usbd_mass_storage::USB_CLASS_MSC)
            .build();

        cortex_m::interrupt::free(|cs| {
            USB_DEV.borrow(cs).replace(Some(usb_dev));
            USB_STORAGE.borrow(cs).replace(Some(scsi));
        });
    };

    unsafe {
        NVIC::unmask(stm32::Interrupt::OTG_FS);
    }

    rprintln!("Init done");

    loop {
        continue;
    }
}

#[interrupt]
fn OTG_FS() {
    usb_interrupt();
}

fn usb_interrupt() {
    cortex_m::interrupt::free(|cs| {
        let mut dev = USB_DEV.borrow(cs).borrow_mut();
        let usb_dev = dev.as_mut().unwrap();

        let mut scsi = USB_STORAGE.borrow(cs).borrow_mut();
        let scsi = scsi.as_mut().unwrap();

        if !usb_dev.poll(&mut [scsi]) {
            return;
        }
    });
}
