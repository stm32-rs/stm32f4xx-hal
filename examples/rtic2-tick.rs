#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;
use rtic_time::Monotonic;
use stm32f4xx_hal::{
    gpio::{Output, PC13},
    pac,
    prelude::*,
};
type Mono = stm32f4xx_hal::timer::MonoTimerUs<pac::TIM3>;

// To use SysTick as monotonic timer, uncomment the lines below
// *and* remove the Mono type alias above
//use rtic_monotonics::systick::prelude::*;
//systick_monotonic!(Mono, 1000);

use rtic::app;

#[app(device = pac, dispatchers = [USART1], peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PC13<Output>,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local) {
        let rcc = ctx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Create TIM3 monotonic and initialize timer queue
        ctx.device.TIM3.monotonic_us(&mut ctx.core.NVIC, &clocks);

        // Uncomment if use SysTick as monotonic timer
        //Mono::start(ctx.core.SYST, 48_000_000);

        let gpioc = ctx.device.GPIOC.split();
        let led = gpioc.pc13.into_push_pull_output();
        defmt::info!("Start");

        tick::spawn().ok();
        (Shared {}, Local { led })
    }

    #[task(local = [led, count: u32 = 0])]
    async fn tick(ctx: tick::Context) {
        loop {
            ctx.local.led.toggle();
            *ctx.local.count += 1;
            defmt::info!("Tick {}", *ctx.local.count);
            Mono::delay(500.millis().into()).await;
        }
    }
}

//Add this to Cargo.toml if use SysTick as monotonic timer
//[dependencies.rtic-monotonics]
//version = "2.0"
//features = ["cortex-m-systick"]
