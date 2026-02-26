//! Display panel drivers and initialization support.
//!
//! This module provides drivers for LCD panels used with the STM32F4 DSI/LTDC
//! display subsystem.
//!
//! Currently supported:
//!
//! - [`nt35510::Nt35510`] — NT35510 panel driver (STM32F469I-DISCO B08 revision)
//! - [`f469disco`] — high-level init helpers for the STM32F469I-DISCO board
//!
//! For OTM8009A panels (B07 and earlier revisions), use the external
//! [`otm8009a`](https://crates.io/crates/otm8009a) crate; the [`f469disco`]
//! helpers integrate with it to provide runtime NT35510 / OTM8009A detection and
//! initialization.
//!
//! # Panel autodetection
//!
//! The [`nt35510::Nt35510::probe`] method can be used directly to detect
//! whether an NT35510 panel is connected via DSI. For a complete F469I-DISCO
//! autodetection flow (including LTDC and panel init), use
//! [`f469disco::init_display_full`].

pub mod nt35510;
pub mod f469disco;
