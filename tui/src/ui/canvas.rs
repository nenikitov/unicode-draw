use tui::{
    text::Spans,
    widgets::{Block, Borders, Paragraph, Widget}, layout::Rect,
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

impl Widget for Canvas {
    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer) {
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
            l.render(
                Rect {
                    x: area.x,
                    y: area.y + i as u16,
                    width: area.width,
                    height: 1,
                } ,
                buf
            )
        }
    }
}

impl DrawableSize for Canvas {
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

impl Drawable for Canvas {}
