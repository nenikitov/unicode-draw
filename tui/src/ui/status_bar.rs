use std::{slice::Iter, fmt::Display};

use tui::{style::{Color, Style, Modifier}, widgets::{Paragraph, Widget}, text::Span, layout::{Alignment, Direction}};

use super::{
    drawable::*, layout::Layout, spacer::Spacer,
};

#[derive(
    Clone, Copy
)]
pub enum Mode {
    Normal,
    Replace
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Normal => write!(f, "Normal"),
            Mode::Replace => write!(f, "Replace")
        }
    }
}


impl Mode {
    pub fn all() -> Iter<'static, Mode> {
        static MODES: [Mode; 2] = [Mode::Normal, Mode::Replace];
        MODES.iter()
    }
}


pub struct ModeBar<'a> {
    mode: &'a Mode
}

impl<'a> ModeBar<'a> {
    pub fn new(mode: &'a Mode) -> Self {
        Self {
            mode
        }
    }

    fn color(&self) -> Color {
        match self.mode {
            Mode::Normal => Color::White,
            Mode::Replace => Color::Blue
        }
    }
}

impl<'a> Widget for ModeBar<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
       let text = Paragraph::new(Span::styled(
            format!(" {} ", self.mode),
            Style::default().fg(self.color()).add_modifier(Modifier::REVERSED)
        )).alignment(Alignment::Center);
        text.render(area, buf);
    }
}

impl<'a> DrawableSize for ModeBar<'a> {
    fn size_preferred(&self) -> (Size, Size) {
        (
            Size {
                size: Mode::all().map(|m| m.to_string().len()).max().unwrap() as u16
                    + 2,
                flexible: false
            },
            Size {
                size: 1,
                flexible: false
            }
        )
    }
}

impl<'a> Drawable for ModeBar<'a> {}


pub struct CursorBar<'a> {
    cursor: &'a (u16, u16)
}

impl<'a> CursorBar<'a> {
    pub fn new(cursor: &'a (u16, u16)) -> Self {
        Self {
            cursor
        }
    }
}

impl<'a> Widget for CursorBar<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let text = Paragraph::new(Span::styled(
            format!(" {}:{}", self.cursor.0, self.cursor.1),
            Style::default().fg(Color::Gray).add_modifier(Modifier::REVERSED)
        )).alignment(Alignment::Center);
        text.render(area, buf);
    }
}

impl<'a> DrawableSize for CursorBar<'a> {
    fn size_preferred(&self) -> (Size, Size) {
        (
            Size {
                size: " 000:000 ".len() as u16,
                flexible: false
            },
            Size {
                size: 1,
                flexible: false
            }
        )
    }
}

impl<'a> Drawable for CursorBar<'a> {}


pub struct StatusBar<'a> {
    layout: Layout<'a>
}

impl<'a> StatusBar<'a> {
    pub fn new(mode: &'a Mode, cursor: &'a (u16, u16)) -> Self {
        let mode = ModeBar {
            mode
        };
        let cursor = CursorBar {
            cursor
        };

        Self {
            layout: Layout::new(
                vec![
                    Box::new(mode),
                    Box::new(Spacer::new_simple_flexible(1)),
                    Box::new(cursor),
                ],
                Direction::Horizontal,
                1
            )
        }
    }
}

impl<'a> Widget for StatusBar<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        for (c, r) in self.layout.align_children(area) {
            c.render(r, buf);
        }
    }
}

impl<'a> DrawableSize for StatusBar<'a> {
    fn size_preferred(&self) -> (Size, Size) {
        self.layout.size_preferred()
    }
}

impl<'a> Drawable for StatusBar<'a> {}
