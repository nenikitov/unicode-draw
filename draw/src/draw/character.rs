use serde::{Serialize, Deserialize};
use crate::{draw::style::Style, traits::copy_over::CopyFrom};

pub enum BlendMode {
    Overwrite,
    OnlyCharacter,
    OnlyStyle
}

#[derive(
    Debug, Clone, Copy,
    PartialEq, Eq,
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


#[cfg(test)]
mod tests {
    use crate::draw::style::{Color, Modifiers};

    use super::*;

    #[test]
    fn test_character_new() {
        assert_eq!(
            Character::new('a', Style::new(Color::Green, Color::Yellow, Modifiers::new(true, false, false))),
            Character { character: 'a', style: Style::new(Color::Green, Color::Yellow, Modifiers::new(true, false, false)) }
        );
    }

    #[test]
    fn test_character_default() {
        assert_eq!(
            Character::default(),
            Character { character: ' ', style: Style::default() }
        )
    }

    #[test]
    fn test_character_blend_overwrite() {
        let mut c1 = Character::new('a', Style::new(Color::Yellow, Color::Black, Modifiers::new(false, true, false)));
        let c2 = Character::new('b', Style::new(Color::Green, Color::White, Modifiers::new(true, false, false)));

        c1.blend(&c2, BlendMode::Overwrite);

        assert_eq!(c1, c2);
    }

    #[test]
    fn test_character_blend_only_character() {
        let mut c1 = Character::new('a', Style::new(Color::Yellow, Color::Black, Modifiers::new(false, true, false)));
        let c2 = Character::new('b', Style::new(Color::Green, Color::White, Modifiers::new(true, false, false)));

        c1.blend(&c2, BlendMode::OnlyCharacter);

        assert_eq!(
            c1,
            Character::new('b', Style::new(Color::Yellow, Color::Black, Modifiers::new(false, true, false)))
        );
    }

    #[test]
    fn test_character_blend_only_style() {
        let mut c1 = Character::new('a', Style::new(Color::Yellow, Color::Black, Modifiers::new(false, true, false)));
        let c2 = Character::new('b', Style::new(Color::Green, Color::White, Modifiers::new(true, false, false)));

        c1.blend(&c2, BlendMode::OnlyStyle);

        assert_eq!(
            c1,
            Character::new('a', Style::new(Color::Green, Color::White, Modifiers::new(true, false, false)))
        );
    }
}
