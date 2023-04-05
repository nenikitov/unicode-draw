use bincode;

use super::loader::*;
use crate::draw::character::Character;

pub struct LoaderBin {}

impl Importer<&[u8]> for LoaderBin {
    fn import(data: &[u8]) -> Vec<Vec<Character>> {
        bincode::deserialize(data).unwrap()
    }
}

impl Exporter<Vec<u8>> for LoaderBin {
    fn export(data: &Vec<Vec<Character>>) -> Vec<u8> {
        bincode::serialize(data).unwrap()
    }
}


#[cfg(test)]
mod test {
    use crate::{draw::{character::Character, style::{Style, Color, Modifiers}}, loaders::loader::{Exporter, Importer}};

    use super::LoaderBin;

    #[test]
    fn test_loader_bin_import_export_reversible() {
        let data = vec![
            vec![
                Character::new('H', Style::new(Color::DarkBlue,    Color::DarkRed,     Modifiers::new(true,  true,  false))),
                Character::new('e', Style::new(Color::Red,         Color::DarkBlue,    Modifiers::new(false, true,  false))),
                Character::new('l', Style::new(Color::DarkBlue,    Color::Green,       Modifiers::new(false, false, true ))),
                Character::new('l', Style::new(Color::Red,         Color::White,       Modifiers::new(false, false, false))),
                Character::new('o', Style::new(Color::Magenta,     Color::Magenta,     Modifiers::new(true,  false, false))),
                Character::new(' ', Style::new(Color::Green,       Color::DarkMagenta, Modifiers::new(false, true,  true ))),
                Character::new(' ', Style::new(Color::Green,       Color::Green,       Modifiers::new(false, false, true ))),
                Character::new(' ', Style::new(Color::DarkMagenta, Color::Green,       Modifiers::new(false, true,  true ))),
            ],
            vec![
                Character::new('w', Style::new(Color::DarkMagenta, Color::DarkGreen,   Modifiers::new(false, false, true ))),
                Character::new('o', Style::new(Color::DarkGray,    Color::DarkGray,    Modifiers::new(false, true,  false))),
                Character::new('r', Style::new(Color::Black,       Color::DarkGray,    Modifiers::new(true,  true,  false))),
                Character::new('l', Style::new(Color::Red,         Color::DarkGray,    Modifiers::new(true,  true,  true ))),
                Character::new('d', Style::new(Color::DarkRed,     Color::DarkGreen,   Modifiers::new(true,  true,  false))),
                Character::new('!', Style::new(Color::DarkCyan,    Color::DarkBlue,    Modifiers::new(true,  true,  false))),
                Character::new('!', Style::new(Color::DarkGreen,   Color::Cyan,        Modifiers::new(true,  true,  false))),
                Character::new('!', Style::new(Color::Cyan,        Color::Red,         Modifiers::new(true,  true,  true ))),
            ]
        ];

        let exported = LoaderBin::export(&data);
        let imported = LoaderBin::import(&exported);

        assert_eq!(data, imported)
    }
}
