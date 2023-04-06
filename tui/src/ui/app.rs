use std::{io::{self, Write}, time::Duration};

use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, event::{self, Event, poll, KeyEventKind, KeyCode, KeyModifiers},
};
use tui::{backend::Backend, Terminal};

use super::{drawable::*, canvas::Canvas};


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
        if poll(Duration::from_secs(0))? {
            let key = event::read()?;
            if let Event::Key(key) = key {
                match (key.kind, key.code, key.modifiers) {
                    (KeyEventKind::Press, KeyCode::Esc, KeyModifiers::NONE) =>
                        self.state = AppState::End,
                    (KeyEventKind::Press, KeyCode::Char('c'), KeyModifiers::CONTROL) =>
                        self.state = AppState::End,
                    _ => ()
                }
            }
        }
        Ok(())
    }
}

impl<B: Backend> WidgetRender<B> for App {
    fn render(&self, f: &mut tui::Frame<B>, target: tui::layout::Rect) {
        self.canvas.render(f, target);
    }
}

impl WidgetSized for App {
    fn size_preferred(&self) -> (Size, Size) {
        self.canvas.size_preferred()
    }
}

impl<B: Backend> Widget<B> for App {}
