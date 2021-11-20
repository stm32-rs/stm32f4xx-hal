#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [USART1])]
mod app {
    use fugit::ExtU32;
    use stm32f4xx_hal::{
        gpio::{gpioc::PC13, Output, PushPull},
        pac,
        prelude::*,
        timer::{monotonic::MonoTimer, Timer},
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PC13<Output<PushPull>>,
    }

    #[monotonic(binds = TIM2, default = true)]
    type MicrosecMono = MonoTimer<pac::TIM2, 1_000_000>;

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let rcc = ctx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        let gpioc = ctx.device.GPIOC.split();
        let led = gpioc.pc13.into_push_pull_output();

        let mono = Timer::new(ctx.device.TIM2, &clocks).monotonic();
        tick::spawn().ok();
        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }

    #[task(local = [led])]
    fn tick(ctx: tick::Context) {
        tick::spawn_after(1.secs()).ok();
        ctx.local.led.toggle();
    }
}
