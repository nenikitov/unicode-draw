use std::{slice::Iter, fmt::Display};

use tui::{backend::Backend, style::{Color, Style, Modifier}, widgets::{Paragraph, Widget}, text::Span, layout::{Alignment, Direction}};

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


pub struct ModeBar {
    mode: Mode
}

impl ModeBar {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    fn color(&self) -> Color {
        match self.mode {
            Mode::Normal => Color::White,
            Mode::Replace => Color::Blue
        }
    }
}

impl<B: Backend> WidgetRender<B> for ModeBar {
    fn render(&self, f: &mut tui::Frame<B>, target: tui::layout::Rect) {
        let text = Paragraph::new(Span::styled(
            format!(" {} ", self.mode),
            Style::default().fg(self.color()).add_modifier(Modifier::REVERSED)
        )).alignment(Alignment::Center);
        f.render_widget(text, target);
    }
}

impl WidgetSized for ModeBar {
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


pub struct CursorBar {
    cursor: (u16, u16)
}

impl CursorBar {
    pub fn new(cursor: (u16, u16)) -> Self {
        Self {
            cursor
        }
    }

    pub fn cursor(&self) -> (u16, u16) {
        self.cursor
    }

    pub fn set_cursor(&mut self, cursor: (u16, u16)) {
        self.cursor = cursor;
    }
}

impl<B: Backend> WidgetRender<B> for CursorBar {
    fn render(&self, f: &mut tui::Frame<B>, target: tui::layout::Rect) {
        let text = Paragraph::new(Span::styled(
            format!(" {}:{}", self.cursor.0, self.cursor.1),
            Style::default().fg(Color::Gray).add_modifier(Modifier::REVERSED)
        )).alignment(Alignment::Center);
        f.render_widget(text, target)
    }
}

impl WidgetSized for CursorBar {
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


pub struct StatusBar<'a> {
    layout: Layout<'a>,
    mode: ModeBar,
    cursor: CursorBar,
}

impl<'a> StatusBar<'a> {
    pub fn new(mode: Mode, cursor: (u16, u16)) -> Self {
        let mode = ModeBar {
            mode
        };
        let cursor = CursorBar {
            cursor
        };

        Self {
            layout: Layout::new(
                vec![
                    &mode,
                    &Spacer::new_simple_flexible(1),
                    &cursor
                ],
                Direction::Horizontal,
                1
            ),
            mode,
            cursor
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode.set_mode(mode);
    }

    pub fn set_cursor_position(&mut self, cursor_position: (u16, u16)) {
        self.cursor_position = cursor_position;
    }
}
