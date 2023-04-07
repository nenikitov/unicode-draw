use std::io::{self, Write};

use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, event::{self, Event, KeyEventKind, KeyCode, KeyModifiers},
};
use tui::{backend::Backend, Terminal, layout::Rect, widgets::Widget};

use super::{drawable::*, canvas::Canvas, size_error::SizeError};


pub fn start_ui<B>(backend: B) -> Result<Terminal<B>, io::Error>
where
    B: Backend,
{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

pub fn end_ui<B>(mut terminal: Terminal<B>) -> Result<(), io::Error>
where
    B: Backend + Write,
{
    disable_raw_mode()?;
    let backend = terminal.backend_mut();
    execute!(backend, LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

#[derive(
    Debug,
    Clone, Copy,
    PartialEq, Eq
)]
pub enum AppState {
    InProgress,
    End
}


pub struct App {
    canvas: Canvas,
    state: AppState
}

impl App {
    pub fn new(canvas: draw::Canvas) -> Self {
        Self {
            canvas: Canvas::new(canvas),
            state: AppState::InProgress
        }
    }

    pub fn state(&self) -> AppState {
        self.state
    }

    pub fn update(&mut self) -> () {
        if let Err(e) = self.handle_input() {
            self.state = AppState::End;
            eprintln!("Error reading from input {e}");
        }
    }

    fn handle_input(&mut self) -> crossterm::Result<()> {
        let event = event::read()?;
        if let Event::Key(key) = event {
            match (key.kind, key.code, key.modifiers) {
                (KeyEventKind::Press, KeyCode::Esc, KeyModifiers::NONE) =>
                    self.state = AppState::End,
                (KeyEventKind::Press, KeyCode::Char('c'), KeyModifiers::CONTROL) =>
                    self.state = AppState::End,
                _ => ()
            }
        }
        Ok(())
    }

    fn check_size(&self, target: Rect) -> bool {
        let size = self.size_preferred();
        target.width >= size.0.size && target.height >= size.1.size
    }
}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer) {
        if self.check_size(area) {
            self.canvas.render(area, buf);
        } else {
            let size = self.size_preferred();
            SizeError::new((size.0.size, size.1.size)).render(area, buf);
        }
    }
}

impl DrawableSize for App {
    fn size_preferred(&self) -> (Size, Size) {
        self.canvas.size_preferred()
    }
}
