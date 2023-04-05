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

