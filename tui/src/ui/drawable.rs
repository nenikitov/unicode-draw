use tui::{Frame, backend::Backend, layout::{Rect, Direction}};

#[derive(Clone, Copy)]
pub struct Size {
    pub size: u16,
    pub flexible: bool
}

pub trait Widget<B: Backend>: WidgetRender<B> + WidgetSized {}

pub trait WidgetRender<B: Backend> {
    fn render(&self, f: &mut Frame<B>, target: Rect);
}

pub trait WidgetSized {
    fn size_preferred(&self) -> (Size, Size);

    fn size_preferred_in_direction(&self, direction: &Direction) -> Size {
        let size = self.size_preferred();
        match direction {
            Direction::Horizontal => size.0,
            Direction::Vertical => size.1,
        }
    }
}
