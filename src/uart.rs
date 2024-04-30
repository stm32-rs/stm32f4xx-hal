//!
//! Asynchronous serial communication using UART peripherals
//!
//! # Word length
//!
//! By default, the UART/UART uses 8 data bits. The `Serial`, `Rx`, and `Tx` structs implement
//! the embedded-hal read and write traits with `u8` as the word type.
//!
//! You can also configure the hardware to use 9 data bits with the `Config` `wordlength_9()`
//! function. After creating a `Serial` with this option, use the `with_u16_data()` function to
//! convert the `Serial<_, u8>` object into a `Serial<_, u16>` that can send and receive `u16`s.
//!
//! In this mode, the `Serial<_, u16>`, `Rx<_, u16>`, and `Tx<_, u16>` structs instead implement
//! the embedded-hal read and write traits with `u16` as the word type. You can use these
//! implementations for 9-bit words.

use crate::pac;

use crate::serial::uart_impls::{RegisterBlockImpl, RegisterBlockUart};

pub use crate::serial::{config, Error, Event, Instance, NoRx, NoTx, Rx, RxISR, Serial, Tx, TxISR};
pub use config::Config;

#[cfg(not(any(feature = "stm32f413", feature = "stm32f423",)))]
macro_rules! halUart {
    ($UART:ty, $Serial:ident, $Rx:ident, $Tx:ident) => {
        pub type $Serial<WORD = u8> = Serial<$UART, WORD>;
        pub type $Tx<WORD = u8> = Tx<$UART, WORD>;
        pub type $Rx<WORD = u8> = Rx<$UART, WORD>;

        impl Instance for $UART {
            type RegisterBlock = RegisterBlockUart;

            fn ptr() -> *const RegisterBlockUart {
                <$UART>::ptr() as *const _
            }

            fn set_stopbits(&self, bits: config::StopBits) {
                use crate::pac::uart4::cr2::STOP_A;
                use config::StopBits;

                /*
                    StopBits::STOP0P5 and StopBits::STOP1P5 aren't supported when using UART
                    STOP_A::STOP1 and STOP_A::STOP2 will be used, respectively
                */
                self.cr2.write(|w| {
                    w.stop().variant(match bits {
                        StopBits::STOP0P5 => STOP_A::Stop1,
                        StopBits::STOP1 => STOP_A::Stop1,
                        StopBits::STOP1P5 => STOP_A::Stop2,
                        StopBits::STOP2 => STOP_A::Stop2,
                    })
                });
            }

            fn peri_address() -> u32 {
                unsafe { (*Self::ptr()).peri_address() }
            }
        }
    };
}

#[cfg(feature = "uart4")]
#[cfg(not(any(feature = "stm32f413", feature = "stm32f423")))]
halUart! { pac::UART4, Serial4, Rx4, Tx4 }
#[cfg(feature = "uart5")]
#[cfg(not(any(feature = "stm32f413", feature = "stm32f423")))]
halUart! { pac::UART5, Serial5, Rx5, Tx5 }

#[cfg(feature = "uart4")]
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl Instance for pac::UART4 {
    type RegisterBlock = RegisterBlockUart;

    fn ptr() -> *const RegisterBlockUart {
        pac::UART4::ptr() as *const _
    }

    fn set_stopbits(&self, _bits: config::StopBits) {
        todo!()
    }

    fn peri_address() -> u32 {
        unsafe { (*Self::ptr()).peri_address() }
    }
}

#[cfg(feature = "uart5")]
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
crate::serial::halUsart! { pac::UART5, Serial5, Rx5, Tx5 }

#[cfg(feature = "uart7")]
crate::serial::halUsart! { pac::UART7, Serial7, Rx7, Tx7 }
#[cfg(feature = "uart8")]
crate::serial::halUsart! { pac::UART8, Serial8, Rx8, Tx8 }
#[cfg(feature = "uart9")]
crate::serial::halUsart! { pac::UART9, Serial9, Rx9, Tx9 }
#[cfg(feature = "uart10")]
crate::serial::halUsart! { pac::UART10, Serial10, Rx10, Tx10 }
