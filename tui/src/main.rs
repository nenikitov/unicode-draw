mod ui;

use std::io;

use draw::{Canvas, character::Character, style::*};
use ui::{app, drawable::WidgetRender};
use tui::backend::CrosstermBackend;

fn main() -> Result<(), i32> {
    let mut terminal = if let Ok(terminal) = app::start_ui(CrosstermBackend::new(io::stdout())) {
        terminal
    } else {
        eprintln!("Can't initialize TUI session");
        return Err(1);
    };

    let mut app = app::App::new(
        Canvas::new_filled(
            100, 12,
            Character::new('a', Style {
                fg: Color::Magenta,
                bg: Color::DarkGreen,
                modifiers: Modifiers {
                    bold: true,
                    italic: true,
                    reverse: false
                }
            })
        )
    );

    loop {
        terminal.draw(|f| {
            app.render(f, f.size())
        }).unwrap();
        app.update();

        match app.state() {
            app::AppState::InProgress =>
                continue,
            app::AppState::End => {
                app::end_ui(terminal).unwrap();
                return Ok(())
            },
        }
    }
}
