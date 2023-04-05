use crate::draw::character::Character;

pub trait Importer<T> {
    fn import(data: T) -> Vec<Vec<Character>>;
}

pub trait Exporter<T> {
    fn export(data: &Vec<Vec<Character>>) -> T;
}
