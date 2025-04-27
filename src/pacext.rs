use stm32f4::{Readable, Reg, RegisterSpec, Resettable, Writable, R, W};

pub mod uart;

macro_rules! wrap_r {
    (pub trait $TrR:ident {
        $(fn $f:ident(&self $(, $n:ident: u8)?) -> $fr:path;)*
    }) => {
        pub trait $TrR {
            $(fn $f(&self $(, $n: u8)?) -> $fr;)*
        }
        impl<REG: reg::$TrR> $TrR for R<REG> {
            $(
                #[inline(always)]
                fn $f(&self $(, $n: u8)?) -> $fr {
                    REG::$f(self $(, $n)?)
                }
            )*
        }
    };
}
pub(crate) use wrap_r;

macro_rules! wrap_w {
    (pub trait $TrR:ident {
        $(fn $f:ident(&mut self $(, $n:ident: u8)?) -> $fr:path;)*
    }) => {
        pub trait $TrR<REG: reg::$TrR> {
            $(fn $f(&mut self $(, $n: u8)?) -> $fr;)*
        }

        impl<REG: reg::$TrR> $TrR<REG> for W<REG> {
            $(
                #[inline(always)]
                fn $f(&mut self $(, $n: u8)?) -> $fr {
                    REG::$f(self $(, $n)?)
                }
            )*
        }
    };
}
pub(crate) use wrap_w;

macro_rules! impl_reg {
    ($($r:ident $(: $n:ident)?  -> &$rty:path;)*) => {
        $(
            #[inline(always)]
            fn $r(&self $(, $n: usize)?) -> &$rty {
                self.$r($($n)?)
            }
        )*
    };
}
pub(crate) use impl_reg;

macro_rules! impl_read {
    ($($f:ident $(: $n:ident)? -> $fty:path;)*) => {
        $(
            #[inline(always)]
            fn $f(r: &R<Self> $(, $n: u8)?) -> $fty {
                r.$f($($n)?)
            }
        )*
    };
}
pub(crate) use impl_read;

macro_rules! impl_write {
    ($($f:ident $(: $n:ident)? -> $fty:path;)*) => {
        $(
            #[inline(always)]
            fn $f(w: &mut W<Self> $(, $n: u8)?) -> $fty {
                w.$f($($n)?)
            }
        )*
    };
}
pub(crate) use impl_write;
