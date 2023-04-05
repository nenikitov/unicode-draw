use std::ops::Add;

use serde::{Serialize, Deserialize};

use crate::traits::copy_over::CopyFrom;

#[derive(
    Debug,
    Clone, Copy,
    PartialEq, Eq,
    Serialize, Deserialize
)]
pub enum Color {
    None,
    Black,    Red,     Green,     Yellow,     Blue,     Magenta,     Cyan,     LightGray,
    DarkGray, DarkRed, DarkGreen, DarkYellow, DarkBlue, DarkMagenta, DarkCyan, White,
    Indexed { i: u8 }, Rgb { r: u8, g: u8, b: u8 }
}


#[derive(
    Debug,
    Clone, Copy,
    PartialEq, Eq,
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
    Debug,
    Clone, Copy,
    PartialEq, Eq,
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifiers_new() {
        assert_eq!(
            Modifiers::new(false, false, false),
            Modifiers { bold: false, italic: false, reverse: false }
        );
        assert_eq!(
            Modifiers::new(true, false, false),
            Modifiers { bold: true, italic: false, reverse: false }
        );
        assert_eq!(
            Modifiers::new(false, true, false),
            Modifiers { bold: false, italic: true, reverse: false }
        );
        assert_eq!(
            Modifiers::new(false, false, true),
            Modifiers { bold: false, italic: false, reverse: true }
        );
        assert_eq!(
            Modifiers::new(true, true, true),
            Modifiers { bold: true, italic: true, reverse: true }
        );
    }

    #[test]
    fn test_modifiers_default() {
        assert_eq!(
            Modifiers::default(),
            Modifiers { bold: false, italic: false, reverse: false }
        );
    }

    #[test]
    fn test_modifiers_add() {
        assert_eq!(
            Modifiers::new(true, false, true) + Modifiers::new(false, true, true),
            Modifiers { bold: true, italic: true, reverse: true }
        );
    }

    #[test]
    fn test_modifiers_copy_from() {
        let mut m1 = Modifiers::new(true, false, true);
        let m2 = Modifiers::new(false, true, true);
        let before = &m1 as *const Modifiers;

        m1.copy_from(&m2);

        assert_eq!(m1, m2);
        assert_eq!(&m1 as *const Modifiers, before);
    }


    #[test]
    fn test_style_new() {
        assert_eq!(
            Style::new(Color::Red, Color::Blue, Modifiers::new(true, false, false)),
            Style { fg: Color::Red, bg: Color::Blue, modifiers: Modifiers::new(true, false, false) }
        );
    }

    #[test]
    fn test_style_default() {
        assert_eq!(
            Style::default(),
            Style { fg: Color::None, bg: Color::None, modifiers: Modifiers::default() }
        );
    }

    #[test]
    fn test_style_add() {
        assert_eq!(
            Style::new(Color::Red, Color::Blue, Modifiers::new(true, false, true))
                + Style::new(Color::Green, Color::Yellow, Modifiers::new(false, false, true)),
            Style { fg: Color::Green, bg: Color::Yellow, modifiers: Modifiers::new(true, false, true) }
        );
    }

    #[test]
    fn test_style_copy_from() {
        let mut s1 = Style::new(Color::Cyan, Color::Yellow, Modifiers::new(true, false, true));
        let s2 = Style::new(Color::Blue, Color::Green, Modifiers::new(false, true, false));
        let before = &s1 as *const Style;

        s1.copy_from(&s2);

        assert_eq!(s1, s2);
        assert_eq!(&s1 as *const Style, before);
    }
}
