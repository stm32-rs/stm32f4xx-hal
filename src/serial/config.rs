use crate::time::Bps;
use crate::time::U32Ext;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordLength {
    #[cfg(feature = "uart_v3")]
    DataBits7,
    DataBits8,
    DataBits9,
}

/// Parity generation and checking. If odd or even parity is selected, the
/// underlying USART will be configured to send/receive the parity bit in
/// addition to the data bits.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    /// No parity bit will be added/checked.
    ParityNone,
    /// The MSB transmitted/received will be generated/checked to have a
    /// even number of bits set.
    ParityEven,
    /// The MSB transmitted/received will be generated/checked to have a
    /// odd number of bits set.
    ParityOdd,
}

/// Stop Bit configuration parameter for serial.
///
/// Wrapper around `STOP_A`
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopBits {
    #[doc = "1 stop bit"]
    STOP1,
    #[doc = "0.5 stop bits"]
    STOP0P5,
    #[doc = "2 stop bits"]
    STOP2,
    #[doc = "1.5 stop bits"]
    STOP1P5,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaConfig {
    None,
    Tx,
    Rx,
    TxRx,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub baudrate: Bps,
    pub wordlength: WordLength,
    pub parity: Parity,
    pub stopbits: StopBits,
    pub dma: DmaConfig,
}

impl Config {
    pub fn baudrate(mut self, baudrate: Bps) -> Self {
        self.baudrate = baudrate;
        self
    }

    pub fn parity_none(mut self) -> Self {
        self.parity = Parity::ParityNone;
        self
    }

    pub fn parity_even(mut self) -> Self {
        self.parity = Parity::ParityEven;
        self
    }

    pub fn parity_odd(mut self) -> Self {
        self.parity = Parity::ParityOdd;
        self
    }

    pub fn wordlength_8(mut self) -> Self {
        self.wordlength = WordLength::DataBits8;
        self
    }

    pub fn wordlength_9(mut self) -> Self {
        self.wordlength = WordLength::DataBits9;
        self
    }

    pub fn stopbits(mut self, stopbits: StopBits) -> Self {
        self.stopbits = stopbits;
        self
    }

    pub fn dma(mut self, dma: DmaConfig) -> Self {
        self.dma = dma;
        self
    }
}

#[derive(Debug)]
pub struct InvalidConfig;

impl Default for Config {
    fn default() -> Config {
        let baudrate = 115_200_u32.bps();
        Config {
            baudrate,
            wordlength: WordLength::DataBits8,
            parity: Parity::ParityNone,
            stopbits: StopBits::STOP1,
            dma: DmaConfig::None,
        }
    }
}

impl<T: Into<Bps>> From<T> for Config {
    fn from(b: T) -> Config {
        Config {
            baudrate: b.into(),
            ..Default::default()
        }
    }
}
