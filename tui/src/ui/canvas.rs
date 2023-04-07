use tui::{
    backend::Backend,
    text::Spans,
    widgets::{Block, Borders, Paragraph}, layout::Rect,
};

use super::{
    character_mappings::CharacterMapping,
    drawable::*,
};


pub struct Canvas {
    canvas: draw::Canvas
}

impl Canvas {
    pub fn new(canvas: draw::Canvas) -> Self {
        Self { canvas }
    }

    pub fn canvas(&self) -> &draw::Canvas {
        &self.canvas
    }
}

impl<B: Backend> WidgetRender<B> for Canvas {
   fn render(&self, f: &mut tui::Frame<B>, target: tui::layout::Rect) {
        let lines = self.canvas.buffer().iter()
            .map(|l| {
                let text = Spans::from(l.iter()
                    .map(|c| CharacterMapping::from(c).into())
                    .collect::<Vec<_>>(),
                );
                Paragraph::new(text)
                    .block(Block::default().borders(Borders::NONE))
            }).collect::<Vec<_>>();

        for (i, l) in lines.into_iter().enumerate() {
            f.render_widget(
                l,
                Rect {
                    x: target.x,
                    y: target.y + i as u16,
                    width: target.width,
                    height: 1,
                }
            );
        }
    }
}

impl WidgetSized for Canvas {
    fn size_preferred(&self) -> (super::drawable::Size, super::drawable::Size) {
        (
            Size {
                size: self.canvas.width() as u16,
                flexible: false,
            },
            Size {
                size: self.canvas.height() as u16,
                flexible: false,
            },
        )
    }
}
