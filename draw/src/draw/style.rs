use std::ops::Add;

use serde::{Serialize, Deserialize};

use crate::traits::copy_over::CopyFrom;

#[derive(
    Debug, Clone, Copy,
    Serialize, Deserialize
)]
pub enum Color {
    None,
    Black,    Red,     Green,     Yellow,     Blue,     Magenta,     Cyan,     LightGray,
    DarkGray, DarkRed, DarkGreen, DarkYellow, DarkBlue, DarkMagenta, DarkCyan, White,
    Indexed { i: u8 }, Rgb { r: u8, g: u8, b: u8 }
}


#[derive(
    Debug, Clone, Copy,
    Serialize, Deserialize
)]
pub struct Modifiers {
    pub bold: bool,
    pub italic: bool,
    pub reverse: bool
}

impl Modifiers {
    pub fn new(bold: bool, italic: bool, reverse: bool) -> Self {
        Self {
            bold,
            italic,
            reverse
        }
    }
}

impl Default for Modifiers {
    fn default() -> Self {
        Self::new(
            false,
            false,
            false
        )
    }
}

impl Add for Modifiers {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new( self.bold || rhs.bold, self.italic || rhs.italic,
            self.reverse || rhs.reverse
        )
    }
}

impl CopyFrom for Modifiers {
    fn copy_from(&mut self, rhs: &Self) {
        self.bold = rhs.bold;
        self.italic = rhs.italic;
        self.reverse = rhs.reverse;
    }
}


#[derive(
    Debug, Clone, Copy,
    Serialize, Deserialize
)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub modifiers: Modifiers
}

impl Style {
    pub fn new(fg: Color, bg: Color, modifiers: Modifiers) -> Self {
        Self {
            fg,
            bg,
            modifiers
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new(
            Color::None,
            Color::None,
            Modifiers::default()
        )
    }
}

impl Add for Style {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            rhs.fg,
            rhs.bg,
            self.modifiers + rhs.modifiers
        )
    }
}

impl CopyFrom for Style {
    fn copy_from(&mut self, rhs: &Self) {
        self.fg = rhs.fg;
        self.bg = rhs.bg;
        self.modifiers.copy_from(&rhs.modifiers);
    }
}
