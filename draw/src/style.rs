use std::ops::Add;

use crate::copy_over::CopyFrom;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    None,
    Black,    Red,     Green,     Yellow,     Blue,     Magenta,     Cyan,     LightGray,
    DarkGray, DarkRed, DarkGreen, DarkYellow, DarkBlue, DarkMagenta, DarkCyan, White,
    Indexed { i: u8 }, Rgb { r: u8, g: u8, b: u8 }
}


#[derive(Debug)]
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

impl Add for Modifiers {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Modifiers::new(
            self.bold || rhs.bold,
            self.italic || rhs.italic,
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


#[derive(Debug)]
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

impl Add for Style {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Style::new(
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
