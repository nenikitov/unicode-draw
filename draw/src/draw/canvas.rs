use crate::draw::character::Character;

use super::character::BlendMode;

#[derive(Debug)]
pub struct Canvas {
    buffer: Vec<Vec<Character>>
}

impl Canvas {
    pub fn new_with_buffer(buffer: Vec<Vec<Character>>) -> Self {
        let width = buffer[0].len();
        for l in &buffer {
            assert_eq!(l.len(), width, "Canvas buffer should be rectangular");
        }

        Self { buffer }
    }

    pub fn new_filled(width: usize, height: usize, character: Character) -> Self {
        Self::new_with_buffer(
            vec![vec![character; width]; height]
        )
    }

    pub fn new(width: usize, height: usize) -> Self {
        Self::new_filled(width, height, Character::default())
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

    pub fn draw_character(&mut self, point: (usize, usize), character: &Character, mode: BlendMode) {
        if let Some(l) = self.buffer.get_mut(point.1) {
            if let Some(c) = l.get_mut(point.0) {
                c.blend(character, mode)
            }
        }
    }
}


#[cfg(test)]
mod test {
    use crate::draw::style::{Style, Color, Modifiers};

    use super::*;

    #[test]
    fn test_new_with_buffer() {
        let buffer = vec![
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
        assert_eq!(
            Canvas::new_with_buffer(buffer.clone()).buffer,
            buffer
        );
    }

    #[test]
    #[should_panic(expected = "assertion failed: `(left == right)`\n  left: `8`,\n right: `5`: Canvas buffer should be rectangular")]
    fn test_new_with_buffer_panics_on_non_rectangular() {
        let buffer = vec![
            vec![
                Character::new('H', Style::new(Color::DarkBlue,    Color::DarkRed,     Modifiers::new(true,  true,  false))),
                Character::new('e', Style::new(Color::Red,         Color::DarkBlue,    Modifiers::new(false, true,  false))),
                Character::new('l', Style::new(Color::DarkBlue,    Color::Green,       Modifiers::new(false, false, true ))),
                Character::new('l', Style::new(Color::Red,         Color::White,       Modifiers::new(false, false, false))),
                Character::new('o', Style::new(Color::Magenta,     Color::Magenta,     Modifiers::new(true,  false, false))),
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
        Canvas::new_with_buffer(buffer);
    }

    #[test]
    fn test_canvas_new_filled() {
        let c = Character::new('#', Style::new(Color::DarkCyan, Color::Red, Modifiers::new(true, true, false)));
        assert_eq!(
            Canvas::new_filled(3, 2, c).buffer,
            vec![
                vec![
                    c,
                    c,
                    c
                ],
                vec![
                    c,
                    c,
                    c
                ]
            ]
        )
    }

    #[test]
    fn test_canvas_new() {
        assert_eq!(
            Canvas::new(3, 2).buffer,
            vec![
                vec![
                    Character::default(),
                    Character::default(),
                    Character::default()
                ],
                vec![
                    Character::default(),
                    Character::default(),
                    Character::default()
                ]
            ]
        )
    }

    #[test]
    fn test_canvas_width() {
        assert_eq!(
            Canvas::new(3, 2).width(),
            3
        )
    }

    #[test]
    fn test_canvas_height() {
        assert_eq!(
            Canvas::new(3, 2).height(),
            2
        )
    }

    #[test]
    fn test_canvas_draw_overflow() {
       let fill = Character::new('_', Style::new(Color::Red, Color::Black, Modifiers::new(false, false, false)));
        let brush = Character::new('#', Style::new(Color::DarkCyan, Color::Red, Modifiers::new(true, true, false)));
        let mut c = Canvas::new_filled(3, 2, fill);

        c.draw_character(
            (10, 10),
            &brush,
            BlendMode::Overwrite
        );

        assert_eq!(
            c.buffer,
            vec![
                vec![
                    fill,
                    fill,
                    fill
                ],
                vec![
                    fill,
                    fill,
                    fill
                ]
            ]
        )
    }

    #[test]
    fn test_canvas_draw_overwrite() {
       let fill = Character::new('_', Style::new(Color::Red, Color::Black, Modifiers::new(false, false, false)));
        let brush = Character::new('#', Style::new(Color::DarkCyan, Color::Red, Modifiers::new(true, true, false)));
        let mut c = Canvas::new_filled(3, 2, fill);

        c.draw_character(
            (2, 1),
            &brush,
            BlendMode::Overwrite
        );

        assert_eq!(
            c.buffer,
            vec![
                vec![
                    fill,
                    fill,
                    fill
                ],
                vec![
                    fill,
                    fill,
                    brush
                ]
            ]
        )
    }

    #[test]
    fn test_canvas_draw_only_character() {
        let fill = Character::new('_', Style::new(Color::Red, Color::Black, Modifiers::new(false, false, false)));
        let brush = Character::new('#', Style::new(Color::DarkCyan, Color::Red, Modifiers::new(true, true, false)));
        let mut c = Canvas::new_filled(3, 2, fill);

        c.draw_character(
            (0, 0),
            &brush,
            BlendMode::OnlyCharacter
        );

        assert_eq!(
            c.buffer,
            vec![
                vec![
                    Character::new(brush.character(), *fill.style()),
                    fill,
                    fill
                ],
                vec![
                    fill,
                    fill,
                    fill
                ]
            ]
        )
    }

    #[test]
    fn test_canvas_draw_only_style() {
        let fill = Character::new('_', Style::new(Color::Red, Color::Black, Modifiers::new(false, false, false)));
        let brush = Character::new('#', Style::new(Color::DarkCyan, Color::Red, Modifiers::new(true, true, false)));
        let mut c = Canvas::new_filled(3, 2, fill);

        c.draw_character(
            (1, 1),
            &brush,
            BlendMode::OnlyStyle
        );

        assert_eq!(
            c.buffer,
            vec![
                vec![
                    fill,
                    fill,
                    fill
                ],
                vec![
                    fill,
                    Character::new(fill.character(), *brush.style()),
                    fill
                ]
            ]
        )
    }

    #[test]
    fn test_canvas_resize_bigger() {
        let fill = Character::new('_', Style::new(Color::Red, Color::Black, Modifiers::new(false, false, false)));
        let default = Character::default();
        let mut c = Canvas::new_filled(3, 2, fill);

        c.resize(5, 3);

        assert_eq!(
            c.buffer,
            vec![
                vec![
                    fill,
                    fill,
                    fill,
                    default,
                    default
                ],
                vec![
                    fill,
                    fill,
                    fill,
                    default,
                    default
                ],
                vec![
                    default,
                    default,
                    default,
                    default,
                    default
                ]
            ]
        )
    }

    #[test]
    fn test_canvas_resize_smaller() {
        let fill = Character::new('_', Style::new(Color::Red, Color::Black, Modifiers::new(false, false, false)));
        let mut c = Canvas::new_filled(3, 2, fill);

        c.resize(2, 1);

        assert_eq!(
            c.buffer,
            vec![
                vec![
                    fill,
                    fill
                ]
            ]
        )
    }
}
