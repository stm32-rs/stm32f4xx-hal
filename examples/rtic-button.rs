#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true)]
mod app {
    use stm32f4xx_hal::{
        gpio::{gpioa::PA0, gpioc::PC13, Edge, Input, Output, PinState, Pull},
        prelude::*,
        rcc::CFGR,
    };
    const SYSFREQ: u32 = 100_000_000;
    // Shared resources go here
    #[shared]
    struct Shared {}

    // Local resources go here
    #[local]
    struct Local {
        button: PA0<Input>,
        led: PC13<Output>,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        // syscfg
        let mut syscfg = ctx.device.SYSCFG.constrain();
        // clocks
        let _rcc = ctx
            .device
            .RCC
            .freeze(CFGR::hse(25.MHz()).sysclk(SYSFREQ.Hz()));
        // gpio ports A and C
        let gpioa = ctx.device.GPIOA.split();
        let gpioc = ctx.device.GPIOC.split();
        // button
        let mut button = Input::new(gpioa.pa0, Pull::Up);
        // or
        //let mut button = gpioa.pa0.into_pull_up_input();
        button.make_interrupt_source(&mut syscfg);
        button.enable_interrupt(&mut ctx.device.EXTI);
        button.trigger_on_edge(&mut ctx.device.EXTI, Edge::Falling);
        // led
        let led = Output::new(gpioc.pc13, PinState::Low);

        (
            Shared {
               // Initialization of shared resources go here
            },
            Local {
                // Initialization of local resources go here
                button,
                led,
            },
            init::Monotonics(),
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            continue;
        }
    }

    #[task(binds = EXTI0, local = [button, led])]
    fn button_click(ctx: button_click::Context) {
        ctx.local.button.clear_interrupt_pending_bit();
        ctx.local.led.toggle();
    }
}
