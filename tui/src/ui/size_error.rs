use tui::{backend::Backend, style::{Color, Style}, layout::Rect, text::{Spans, Span}, widgets::Paragraph};

use super::drawable::*;

const SPAN_HEADER: &str = "Terminal window is too small";

struct SizeError<'a> {
    min_size: &'a Rect
}

impl<'a> SizeError<'a> {
    pub fn new(min_size: &'a Rect) -> Self {
        Self {
            min_size
        }
    }
}

impl<'a, B: Backend> WidgetRender<B> for SizeError<'a> {
    fn render(&self, f: &mut tui::Frame<B>, target: tui::layout::Rect) {
        let size = f.size();
        let color_x = if size.width < self.min_size.width { Color::LightRed } else { Color::LightGreen };
        let color_y = if size.height < self.min_size.height { Color::LightRed } else { Color::LightGreen };

        let text = vec![
            Spans::from(Span::raw(SPAN_HEADER)),
            Spans::from(vec![
                Span::raw("Width: "),
                Span::styled(format!("{}", size.width), Style::default().fg(color_x)),
                Span::raw(format!(" needed {}", self.min_size.width))
            ]),
            Spans::from(vec![
                Span::raw("Height: "),
                Span::styled(format!("{}", size.height), Style::default().fg(color_y)),
                Span::raw(format!(" needed {}", self.min_size.height))
            ])
        ];

        let paragraph = Paragraph::new(text)
            .style(Style::default().bg(Color::Black))
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(paragraph, target)
    }
}

impl<'a> WidgetSized for SizeError<'a> {
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

impl<'a, B: Backend> Widget<B> for SizeError<'a> {}
