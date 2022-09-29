use core::marker::PhantomData;

use super::Instance;
use crate::dma::traits::PeriAddress;

pub(crate) struct Tx<I2C> {
    i2c: PhantomData<I2C>,
}

pub(crate) struct Rx<I2C> {
    i2c: PhantomData<I2C>,
}

unsafe impl<I2C: Instance> PeriAddress for Rx<I2C> {
    #[inline(always)]
    fn address(&self) -> u32 {
        unsafe { &(*I2C::ptr()).dr as *const _ as u32 }
    }

    type MemSize = u8;
}

unsafe impl<I2C: Instance> PeriAddress for Tx<I2C> {
    #[inline(always)]
    fn address(&self) -> u32 {
        unsafe { &(*I2C::ptr()).dr as *const _ as u32 }
    }

    type MemSize = u8;
}
