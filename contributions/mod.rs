//! GUI Screens for Specter-DIY
//!
//! Screen navigation and rendering for the Bitcoin hardware wallet.

pub mod home;
pub mod wallet_gen;
pub mod settings;
pub mod about;

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Rectangle, PrimitiveStyle},
    text::Text,
};

use crate::theme::Theme;

/// Screen action result
#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub enum Action {
    /// No action
    None,
    /// Navigate to another screen
    Navigate(ScreenId),
    /// Go back
    Back,
    /// Render needed
    Render,
}

/// Screen identifiers
#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub enum ScreenId {
    /// Home/menu screen
    Home,
    /// Wallet generation
    WalletGen,
    /// Settings
    Settings,
    /// About
    About,
    /// Load wallet
    LoadWallet,
    /// Sign transaction
    SignTx,
}

/// Touch event
#[derive(Debug, Clone, Copy)]
pub struct TouchEvent {
    pub x: u16,
    pub y: u16,
    pub pressed: bool,
}

/// Button definition for UI
#[derive(Debug, Clone)]
pub struct Button {
    pub rect: Rectangle,
    pub label: &'static str,
    pub action: Action,
}

impl Button {
    pub fn new(x: i32, y: i32, width: u32, height: u32, label: &'static str, action: Action) -> Self {
        Self {
            rect: Rectangle::new(Point::new(x, y), Size::new(width, height)),
            label,
            action,
        }
    }

    pub fn contains(&self, x: u16, y: u16) -> bool {
        let p = Point::new(x as i32, y as i32);
        self.rect.contains(p)
    }
}

/// Clear the framebuffer with background color
pub fn clear_framebuffer(fb: &mut impl DrawTarget<Color = Rgb565>, theme: &Theme) {
    fb.clear(theme.background).ok();
}

/// Draw a title bar
pub fn draw_title(fb: &mut impl DrawTarget<Color = Rgb565>, title: &str, theme: &Theme) {
    let style = MonoTextStyle::new(&FONT_10X20, theme.text_primary);
    Text::new(title, Point::new(240 - (title.len() * 5) as i32, 30), style)
        .draw(fb)
        .ok();
}

/// Draw a button
pub fn draw_button(fb: &mut impl DrawTarget<Color = Rgb565>, btn: &Button, theme: &Theme, selected: bool) {
    let fill_color = if selected { theme.accent } else { Rgb565::new(0x30, 0x30, 0x30) };
    
    btn.rect
        .into_styled(PrimitiveStyle::with_fill(fill_color))
        .draw(fb)
        .ok();
    
    let style = MonoTextStyle::new(&FONT_10X20, theme.text_primary);
    let x = btn.rect.top_left.x + 20;
    let y = btn.rect.top_left.y + 25;
    Text::new(btn.label, Point::new(x, y), style)
        .draw(fb)
        .ok();
}
