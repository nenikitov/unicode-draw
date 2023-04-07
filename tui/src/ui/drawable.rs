use tui::{layout::Direction, widgets::Widget};

#[derive(Clone, Copy)]
pub struct Size {
    pub size: u16,
    pub flexible: bool
}

pub trait Drawable: Widget + DrawableSize {}

pub trait DrawableSize {
    fn size_preferred(&self) -> (Size, Size);

    fn size_preferred_in_direction(&self, direction: &Direction) -> Size {
        let size = self.size_preferred();
        match direction {
            Direction::Horizontal => size.0,
            Direction::Vertical => size.1,
        }
    }
}
