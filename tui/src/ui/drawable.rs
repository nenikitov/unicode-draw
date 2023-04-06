use tui::{Frame, backend::Backend, layout::{Rect, Direction}};

pub struct Size {
    pub size: u16,
    pub flexible: bool
}

pub trait Drawable<B: Backend> {
    fn render(&self, f: &mut Frame<B>, target: Rect);
    fn size_preferred(&self) -> (Size, Size);

    fn size_preferred_in_direction(&self, direction: &Direction) -> Size {
        let size = self.size_preferred();
        match direction {
            Direction::Horizontal => size.0,
            Direction::Vertical => size.1,
        }
    }
}
