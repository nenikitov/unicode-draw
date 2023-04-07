use tui::{backend::Backend, style::{Color, Style}, layout::Rect, text::{Spans, Span}, widgets::Paragraph};

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

impl<B: Backend> WidgetRender<B> for SizeError {
    fn render(&self, f: &mut tui::Frame<B>, target: tui::layout::Rect) {
        let size = f.size();
        let color_x = if size.width < self.min_size.0 { Color::LightRed } else { Color::LightGreen };
        let color_y = if size.height < self.min_size.1 { Color::LightRed } else { Color::LightGreen };

        let text = vec![
            Spans::from(Span::raw(SPAN_HEADER)),
            Spans::from(vec![
                Span::raw("Width: "),
                Span::styled(format!("{}", size.width), Style::default().fg(color_x)),
                Span::raw(format!(" needed {}", self.min_size.0))
            ]),
            Spans::from(vec![
                Span::raw("Height: "),
                Span::styled(format!("{}", size.height), Style::default().fg(color_y)),
                Span::raw(format!(" needed {}", self.min_size.1))
            ])
        ];

        let paragraph = Paragraph::new(text)
            .style(Style::default().bg(Color::Black))
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(paragraph, target)
    }
}

impl WidgetSized for SizeError {
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
