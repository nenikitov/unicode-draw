use tui::{style::{Color, Style}, layout::Rect, text::{Spans, Span}, widgets::{Paragraph, Widget}};

use super::drawable::*;

const SPAN_HEADER: &str = "Terminal window is too small";

pub struct SizeError {
    min_size: (u16, u16)
}

impl SizeError {
    pub fn new(min_size: (u16, u16)) -> Self {
        Self {
            min_size
        }
    }
}

impl Widget for SizeError {
    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer) {
        let color_x = if area.width < self.min_size.0 { Color::LightRed } else { Color::LightGreen };
        let color_y = if area.height < self.min_size.1 { Color::LightRed } else { Color::LightGreen };

        let text = vec![
            Spans::from(Span::raw(SPAN_HEADER)),
            Spans::from(vec![
                Span::raw("Width: "),
                Span::styled(format!("{}", area.width), Style::default().fg(color_x)),
                Span::raw(format!(" needed {}", self.min_size.0))
            ]),
            Spans::from(vec![
                Span::raw("Height: "),
                Span::styled(format!("{}", area.height), Style::default().fg(color_y)),
                Span::raw(format!(" needed {}", self.min_size.1))
            ])
        ];

        let paragraph = Paragraph::new(text)
            .style(Style::default().bg(Color::Black))
            .alignment(tui::layout::Alignment::Center);
        paragraph.render(area, buf);
    }
}

impl DrawableSize for SizeError {
    fn size_preferred(&self) -> (Size, Size) {
       (
            Size {
                size: SPAN_HEADER.len() as u16 + 4,
                flexible: true
            },
            Size {
                size: 3 + 4,
                flexible: true
            }
        )
    }
}

impl Drawable for SizeError {}
