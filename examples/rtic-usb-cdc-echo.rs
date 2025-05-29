#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [USART1])]
mod app {
    use stm32f4xx_hal::{
        gpio::{Output, PC13},
        otg_fs::{UsbBus, UsbBusType, USB},
        pac,
        prelude::*,
        rcc::Config,
        timer::MonoTimerUs,
    };

    use usb_device::prelude::*;
    use usbd_serial::SerialPort;

    #[shared]
    struct Shared {
        usb_dev: UsbDevice<'static, UsbBusType>,
        usb_serial: SerialPort<'static, UsbBusType>,
    }

    #[local]
    struct Local {
        led: PC13<Output>,
    }

    #[monotonic(binds = TIM2, default = true)]
    type MicrosecMono = MonoTimerUs<pac::TIM2>;

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        static mut EP_MEMORY: [u32; 1024] = [0; 1024];
        static mut USB_BUS: Option<usb_device::bus::UsbBusAllocator<UsbBusType>> = None;

        let dp = ctx.device;

        // Setup system clocks
        let rcc = dp
            .RCC
            .freeze(Config::hse(25.MHz()).sysclk(84.MHz()).require_pll48clk());

        let gpioa = dp.GPIOA.split();
        let gpioc = dp.GPIOC.split();
        let led = gpioc.pc13.into_push_pull_output();

        let mono = dp.TIM2.monotonic_us(&rcc.clocks);
        tick::spawn().ok();

        // *** Begin USB setup ***
        let usb = USB {
            usb_global: dp.OTG_FS_GLOBAL,
            usb_device: dp.OTG_FS_DEVICE,
            usb_pwrclk: dp.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into(),
            pin_dp: gpioa.pa12.into(),
            hclk: rcc.clocks.hclk(),
        };
        unsafe {
            USB_BUS.replace(UsbBus::new(usb, &mut EP_MEMORY));
        }

        let usb_serial = usbd_serial::SerialPort::new(unsafe { USB_BUS.as_ref().unwrap() });
        let usb_dev = UsbDeviceBuilder::new(
            unsafe { USB_BUS.as_ref().unwrap() },
            UsbVidPid(0x16c0, 0x27dd),
        )
        .device_class(usbd_serial::USB_CLASS_CDC)
        .strings(&[StringDescriptors::default()
            .manufacturer("Fake Company")
            .product("Product")
            .serial_number("TEST")])
        .unwrap()
        .build();

        (
            Shared {
                usb_dev,
                usb_serial,
            },
            Local { led },
            init::Monotonics(mono),
        )
    }

    #[task(local = [led])]
    fn tick(ctx: tick::Context) {
        tick::spawn_after(1.secs()).ok();
        ctx.local.led.toggle();
    }

    #[task(binds=OTG_FS, shared=[usb_dev, usb_serial])]
    fn usb_fs(cx: usb_fs::Context) {
        let usb_fs::SharedResources {
            mut usb_dev,
            mut usb_serial,
        } = cx.shared;

        (&mut usb_dev, &mut usb_serial).lock(|usb_dev, usb_serial| {
            if usb_dev.poll(&mut [usb_serial]) {
                let mut buf = [0u8; 64];

                match usb_serial.read(&mut buf) {
                    Ok(count) if count > 0 => {
                        let mut write_offset = 0;
                        while write_offset < count {
                            match usb_serial.write(&mut buf[write_offset..count]) {
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
        });
    }
}
