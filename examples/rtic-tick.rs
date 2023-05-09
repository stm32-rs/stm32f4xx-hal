#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [USART1])]
mod app {
    use fugit::ExtU64;
    use stm32f4xx_hal::{
        gpio::{Output, PC13},
        pac,
        prelude::*,
        //timer::MonoTimerUs, // Easy monotonic timer for 32-bit TIMs only
        timer::MonoTimer64Us, // Extended 64-bit timer for 16/32-bit TIMs
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PC13<Output>,
    }

    //#[monotonic(binds = TIM2, default = true)]
    //type MicrosecMono = MonoTimerUs<pac::TIM2>;
    #[monotonic(binds = TIM3, default = true)]
    type MicrosecMono = MonoTimer64Us<pac::TIM3>;

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let rcc = ctx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        let gpioc = ctx.device.GPIOC.split();
        let led = gpioc.pc13.into_push_pull_output();
        defmt::info!("Start");

        //let mono = ctx.device.TIM2.monotonic_us(&clocks);
        let mono = ctx.device.TIM3.monotonic64_us(&clocks);
        tick::spawn().ok();
        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[task(local = [led])]
    fn tick(ctx: tick::Context) {
        tick::spawn_after(1.secs()).ok();
        ctx.local.led.toggle();
        defmt::info!("Tick");
    }
}
