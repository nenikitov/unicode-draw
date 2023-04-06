use tui::{
    backend::Backend,
    text::Spans,
    widgets::{Block, Borders, Paragraph}, layout::Rect,
};

use super::{
    character_mappings::CharacterMapping,
    drawable::{Drawable, Size},
};


pub struct Canvas<'a> {
    canvas: &'a draw::Canvas,
}

impl<'a, B: Backend> Drawable<B> for Canvas<'a> {
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
