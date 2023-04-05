use super::loader::*;
use crate::draw::{character::Character, style::Style};

pub struct LoaderTxt {}

impl Importer<&str> for LoaderTxt {
    fn import(data: &str) -> Vec<Vec<Character>> {
        let width = data.lines().map(|l| l.len()).max().unwrap();

        data.lines().map(|l| {
            let mut l: Vec<_> = l.chars().map(|c|
                Character::new(c, Style::default())
            ).collect();

            l.resize(width, Character::default());
            l
        }).collect()
    }
}

impl Exporter<String> for LoaderTxt {
    fn export(data: &Vec<Vec<Character>>) -> String {
        data.iter().map(|l| {
            l.iter().map(|c| {
                c.character()
            }).collect::<String>()
        }).collect::<Vec<String>>().join("\n")
    }
}


#[cfg(test)]
mod test {
    use crate::draw::style::{Color, Modifiers};

    use super::*;

    #[test]
    fn test_loader_txt_import() {
        assert_eq!(
            LoaderTxt::import("Hello\nworld!!!"),
            vec![
                vec![
                    Character::new('H', Style::default()),
                    Character::new('e', Style::default()),
                    Character::new('l', Style::default()),
                    Character::new('l', Style::default()),
                    Character::new('o', Style::default()),
                    Character::new(' ', Style::default()),
                    Character::new(' ', Style::default()),
                    Character::new(' ', Style::default()),
                ],
                vec![
                    Character::new('w', Style::default()),
                    Character::new('o', Style::default()),
                    Character::new('r', Style::default()),
                    Character::new('l', Style::default()),
                    Character::new('d', Style::default()),
                    Character::new('!', Style::default()),
                    Character::new('!', Style::default()),
                    Character::new('!', Style::default()),
                ]
            ]
        )
    }

    #[test]
    fn test_loader_txt_export() {
        assert_eq!(
            LoaderTxt::export(&vec![
                vec![
                    Character::new('H', Style::new(Color::Red, Color::Black, Modifiers::new(true, false, false))),
                    Character::new('e', Style::default()),
                    Character::new('l', Style::default()),
                    Character::new('l', Style::default()),
                    Character::new('o', Style::default()),
                    Character::new(' ', Style::default()),
                    Character::new(' ', Style::default()),
                    Character::new(' ', Style::default()),
                ],
                vec![
                    Character::new('w', Style::new(Color::DarkCyan, Color::Red, Modifiers::new(false, true, false))),
                    Character::new('o', Style::default()),
                    Character::new('r', Style::default()),
                    Character::new('l', Style::default()),
                    Character::new('d', Style::default()),
                    Character::new('!', Style::default()),
                    Character::new('!', Style::default()),
                    Character::new('!', Style::default()),
                ]
            ]),
            "Hello   \nworld!!!"
        );
    }
}
