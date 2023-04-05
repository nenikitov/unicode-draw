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
            + "\n"
        }).collect()
    }
}

