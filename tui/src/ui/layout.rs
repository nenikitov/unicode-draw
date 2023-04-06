use tui::{backend::Backend, layout::{Direction, Rect}};

use super::drawable::*;

pub struct Layout<B: Backend> {
    children: Vec<Box<dyn Widget<B>>>,
    direction: Direction,
    margin: u16
}

impl<B: Backend> Layout<B> {
    pub fn new(children: Vec<Box<dyn Widget<B>>>, direction: Direction, margin: u16) -> Self {
        Self {
            children,
            direction,
            margin
        }
    }


    fn count_flexible_children(&self, direction: &Direction) -> u16 {
        self.children.iter()
            .filter(|c| c.size_preferred_in_direction(direction).flexible)
            .count() as u16
    }

    fn count_margins(&self) -> u16 {
        (self.children.len() - 1).max(0) as u16
    }
}

impl<B: Backend> WidgetRender<B> for Layout<B> {
    fn render(&self, f: &mut tui::Frame<B>, target: tui::layout::Rect) {
        let size_fixed: u16 = self.children.iter()
            .map(|c| c.size_preferred_in_direction(&self.direction))
            .filter(|s| !s.flexible).map(|s| s.size).sum();

        let mut count_flexible = self.count_flexible_children(&self.direction);
        let mut size_flexible = target.width - size_fixed - self.count_margins() * self.margin;
        let mut position = match &self.direction {
            Direction::Horizontal => target.x,
            Direction::Vertical => target.y,
        };

        for c in &self.children {
            let size =
                if c.size_preferred_in_direction(&self.direction).flexible {
                    let size = size_flexible / count_flexible;
                    count_flexible -= 1;
                    size_flexible -= size;
                    size
                }
                else {
                    c.size_preferred_in_direction(&self.direction).size
                };
            c.render(
                f,
                Rect {
                    x: match &self.direction {
                        Direction::Horizontal => position,
                        Direction::Vertical => target.x,
                    },
                    y: match &self.direction {
                        Direction::Horizontal => target.y,
                        Direction::Vertical => position,
                    },
                    width: match &self.direction {
                        Direction::Horizontal => size,
                        Direction::Vertical => target.width,
                    },
                    height: match &self.direction {
                        Direction::Horizontal => target.height,
                        Direction::Vertical => size,
                    },
                }
            );
            position += size + self.margin;
        }
    }
}

impl<B: Backend> WidgetSized for Layout<B> {
    fn size_preferred(&self) -> (Size, Size) {
        match self.direction {
            Direction::Horizontal => (
                Size {
                    size: self.children.iter().map(|c| c.size_preferred().0.size).sum::<u16>()
                        + self.count_margins() * self.margin,
                    flexible: self.count_flexible_children(&Direction::Horizontal) > 0
                },
                Size {
                    size: self.children.iter().map(|c| c.size_preferred().1.size).max().unwrap(),
                    flexible: self.count_flexible_children(&Direction::Vertical) > 0
                }
            ),
            Direction::Vertical => (
                Size {
                    size: self.children.iter().map(|c| c.size_preferred().0.size).max().unwrap(),
                    flexible: self.count_flexible_children(&Direction::Horizontal) > 0
                },
                Size {
                    size: self.children.iter().map(|c| c.size_preferred().1.size).sum::<u16>()
                        + self.count_margins() * self.margin,
                    flexible: self.count_flexible_children(&Direction::Vertical) > 0
                }
            )
        }
    }
}


impl<B: Backend> Widget<B> for Layout<B> {}
