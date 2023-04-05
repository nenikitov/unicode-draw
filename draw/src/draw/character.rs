use serde::{Serialize, Deserialize};
use crate::{draw::style::Style, traits::copy_over::CopyFrom};

pub enum BlendMode {
    Overwrite,
    OnlyCharacter,
    OnlyStyle
}

#[derive(
    Debug, Clone, Copy,
    Serialize, Deserialize
)]
pub struct Character {
    character: char,
    style: Style
}

impl Character {
    pub fn new(character: char, style: Style) -> Self {
        Self {
            character,
            style
        }
    }

    pub fn blend(&mut self, rhs: &Self, mode: BlendMode) {
        match mode {
            BlendMode::Overwrite => {
                self.copy_from(rhs)
            },
            BlendMode::OnlyCharacter => {
                self.character = rhs.character
            },
            BlendMode::OnlyStyle => {
                self.style.copy_from(&rhs.style)
            }
        }
    }


    pub fn character(&self) -> char {
        self.character
    }

    pub fn style(&self) -> &Style {
        &self.style
    }
}

impl Default for Character {
    fn default() -> Self {
        Self::new(
            ' ',
            Style::default()
        )
    }
}

impl CopyFrom for Character {
    fn copy_from(&mut self, rhs: &Self) {
        self.character= rhs.character;
        self.style.copy_from(&rhs.style);
    }
}
