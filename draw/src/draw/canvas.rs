use crate::draw::character::Character;

use super::character::BlendMode;

#[derive(Debug)]
pub struct Canvas {
    buffer: Vec<Vec<Character>>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![vec![Character::default(); width]; height],
        }
    }

    pub fn buffer(&self) -> &[Vec<Character>] {
        self.buffer.as_slice()
    }

    pub fn width(&self) -> usize {
        self.buffer[0].len()
    }

    pub fn height(&self) -> usize {
        self.buffer.len()
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.buffer.resize(height, vec![Character::default(); self.width()]);
        for r in self.buffer.iter_mut() {
            r.resize(width, Character::default())
        }
    }

    pub fn draw_character(&mut self, x: usize, y: usize, character: &Character, mode: BlendMode) {
        if let Some(l) = self.buffer.get_mut(y) {
            if let Some(c) = l.get_mut(x) {
                c.blend(character, mode)
            }
        }
    }

    pub fn load(&mut self, buffer: Vec<Vec<Character>>) {
        self.buffer = buffer;
    }
}
