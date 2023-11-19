#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [SDIO])]
mod usart_shell {
    use core::fmt::Write;
    use dwt_systick_monotonic::DwtSystick;
    use embedded_graphics::{
        mono_font::{ascii::FONT_5X8, MonoTextStyleBuilder},
        pixelcolor::BinaryColor,
        prelude::*,
        text::Baseline,
        text::Text,
    };
    use heapless::String;

    use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

    use stm32f4xx_hal::{
        gpio::{gpioa::PA0, gpioc::PC13, Edge, Input, Output, PushPull},
        i2c::I2c,
        pac::I2C1,
        pac::USART1,
        prelude::*,
        serial::{self, config::Config, Serial},
        timer::Event,
    };

    use ushell::{
        autocomplete::StaticAutocomplete, history::LRUHistory, Input as ushell_input,
        ShellError as ushell_error, UShell,
    };

    type LedType = PC13<Output<PushPull>>;
    type ButtonType = PA0<Input>;
    type ShellType = UShell<Serial<USART1>, StaticAutocomplete<5>, LRUHistory<32, 4>, 32>;
    type DisplayType = Ssd1306<
        I2CInterface<I2c<I2C1>>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >;

    const SHELL_PROMPT: &str = "#> ";
    const CR: &str = "\r\n";
    const HELP: &str = "\r\n\
        help: !
        ";
    const SYSFREQ: u32 = 100_000_000;
    const FPS: u32 = 25;

    #[derive(Clone, Copy)]
    pub struct DisplayInfo {
        led_status: bool,
        update_hz: u32,
    }

    #[monotonic(binds = SysTick, default = true)]
    type Mono = DwtSystick<SYSFREQ>;
    // Shared resources go here
    #[shared]
    struct Shared {
        led_enabled: bool,
        display_state: DisplayInfo,
    }

    // Local resources go here
    #[local]
    struct Local {
        button: ButtonType,
        led: LedType,
        shell: ShellType,
        ldisp: DisplayType,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        // syscfg
        let mut syscfg = ctx.device.SYSCFG.constrain();
        // clocks
        let rcc = ctx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(SYSFREQ.Hz()).use_hse(25.MHz()).freeze();
        // monotonic timer
        let mono = DwtSystick::new(&mut ctx.core.DCB, ctx.core.DWT, ctx.core.SYST, SYSFREQ);
        // gpio ports A and C
        let gpioa = ctx.device.GPIOA.split();
        let gpiob = ctx.device.GPIOB.split();
        let gpioc = ctx.device.GPIOC.split();
        // button
        let mut button = gpioa.pa0.into_pull_up_input();
        button.make_interrupt_source(&mut syscfg);
        button.enable_interrupt(&mut ctx.device.EXTI);
        button.trigger_on_edge(&mut ctx.device.EXTI, Edge::Falling);
        // led
        let led = gpioc.pc13.into_push_pull_output();
        // i2c
        let scl = gpiob.pb8;
        let sda = gpiob.pb9;
        let i2c = I2c::new(ctx.device.I2C1, (scl, sda), 400.kHz(), &clocks);
        // serial
        let pins = (gpioa.pa9, gpioa.pa10);
        let mut serial = Serial::new(
            ctx.device.USART1,
            pins,
            Config::default().baudrate(115_200.bps()).wordlength_8(),
            &clocks,
        )
        .unwrap()
        .with_u8_data();
        serial.listen(serial::Event::RxNotEmpty);
        // ushell
        let autocomplete = StaticAutocomplete(["clear", "help", "off", "on", "status"]);
        let history = LRUHistory::default();
        let shell = UShell::new(serial, autocomplete, history);
        // display
        let interface = I2CDisplayInterface::new(i2c);
        let mut ldisp = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        ldisp.init().unwrap();

        let mut timer = ctx.device.TIM2.counter_hz(&clocks);
        //let mut timer = FTimer::new(ctx.device.TIM1, &clocks).counter_hz();
        timer.start(FPS.Hz()).unwrap();
        timer.listen(Event::Update);

        (
            Shared {
                // Initialization of shared resources go here
                led_enabled: true,
                display_state: {
                    DisplayInfo {
                        led_status: true,
                        update_hz: FPS,
                    }
                },
            },
            Local {
                // Initialization of local resources go here
                button,
                led,
                shell,
                ldisp,
            },
            init::Monotonics(mono),
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            continue;
        }
    }

    #[task(local = [led], shared = [led_enabled, display_state])]
    fn setled(ctx: setled::Context) {
        let setled::LocalResources { led } = ctx.local;
        let setled::SharedResources {
            mut led_enabled,
            mut display_state,
        } = ctx.shared;
        let led_on = led_enabled.lock(|e| *e);
        if led_on {
            led.set_low();
            display_state.lock(|ds| ds.led_status = true);
        } else {
            led.set_high();
            display_state.lock(|ds| ds.led_status = false);
        }
    }

    #[task(binds = TIM2, local = [ldisp], shared = [display_state])]
    fn updatedisplay(ctx: updatedisplay::Context) {
        let updatedisplay::LocalResources { ldisp } = ctx.local;
        let updatedisplay::SharedResources { mut display_state } = ctx.shared;
        let ds = display_state.lock(|e| *e);
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_5X8)
            .text_color(BinaryColor::On)
            .build();
        let mut fpsstr: String<10> = String::new();
        write!(fpsstr, "FPS: {}", ds.update_hz).unwrap();
        let mut ledstr: String<10> = String::new();
        if ds.led_status {
            write!(ledstr, "LED is ON").unwrap();
        } else {
            write!(ledstr, "LED is OFF").unwrap();
        }
        ldisp.clear();
        Text::with_baseline(fpsstr.as_str(), Point::zero(), text_style, Baseline::Top)
            .draw(ldisp)
            .unwrap();

        Text::with_baseline(
            ledstr.as_str(),
            Point::new(0, 16),
            text_style,
            Baseline::Top,
        )
        .draw(ldisp)
        .unwrap();
        ldisp.flush().unwrap();
    }

    #[task(binds = EXTI0, local = [button], shared = [led_enabled])]
    fn button_click(mut ctx: button_click::Context) {
        ctx.local.button.clear_interrupt_pending_bit();
        let led_on = ctx.shared.led_enabled.lock(|e| *e);
        if led_on {
            ctx.shared.led_enabled.lock(|e| *e = false);
        } else {
            ctx.shared.led_enabled.lock(|e| *e = true);
        }
        setled::spawn().unwrap();
    }

    #[task(binds = USART1, priority = 1, shared = [led_enabled], local = [shell])]
    fn serialshell(ctx: serialshell::Context) {
        let serialshell::LocalResources { shell } = ctx.local;
        let serialshell::SharedResources { mut led_enabled } = ctx.shared;
        loop {
            match shell.poll() {
                Ok(Some(ushell_input::Command((cmd, _args)))) => {
                    match cmd {
                        "help" => {
                            shell.write_str(HELP).ok();
                        }
                        "clear" => {
                            shell.clear().ok();
                        }
                        "on" => {
                            led_enabled.lock(|e| *e = true);
                            setled::spawn().unwrap();
                            shell.write_str(CR).ok();
                        }
                        "off" => {
                            led_enabled.lock(|e| *e = false);
                            setled::spawn().unwrap();
                            shell.write_str(CR).ok();
                        }
                        "status" => {
                            let on = led_enabled.lock(|e| *e);
                            let status = if on { "On" } else { "Off" };
                            write!(shell, "{0:}LED: {1:}{0:}", CR, status).ok();
                        }
                        "" => {
                            shell.write_str(CR).ok();
                        }
                        _ => {
                            write!(shell, "{0:}unsupported command{0:}", CR).ok();
                        }
                    }
                    shell.write_str(SHELL_PROMPT).ok();
                }
                Err(ushell_error::WouldBlock) => break,
                _ => {}
            }
        }
    }
}
