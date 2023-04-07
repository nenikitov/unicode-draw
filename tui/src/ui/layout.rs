use std::collections::HashMap;

use tui::{layout::{Direction, Rect}, widgets::Widget};

use super::drawable::*;

pub struct Layout<'a> {
    children: Vec<Box<dyn Drawable + 'a>>,
    direction: Direction,
    margin: u16
}

impl<'a> Layout<'a> {
    pub fn new(children: Vec<Box<dyn Drawable + 'a>>, direction: Direction, margin: u16) -> Self {
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

    pub fn align_children(&self, area: Rect) -> Vec<(&dyn Drawable, Rect)> {
        let size_fixed: u16 = self.children.iter()
            .map(|c| c.size_preferred_in_direction(&self.direction))
            .filter(|s| !s.flexible).map(|s| s.size).sum();

        let mut count_flexible = self.count_flexible_children(&self.direction);
        let mut size_flexible = area.width - size_fixed - self.count_margins() * self.margin;
        let mut position = match &self.direction {
            Direction::Horizontal => area.x,
            Direction::Vertical => area.y,
        };

        self.children.iter().map(|c| {
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

            let r = Rect {
                x: match &self.direction {
                    Direction::Horizontal => position,
                    Direction::Vertical => area.x,
                },
                y: match &self.direction {
                    Direction::Horizontal => area.y,
                    Direction::Vertical => position,
                },
                width: match &self.direction {
                    Direction::Horizontal => size,
                    Direction::Vertical => area.width,
                },
                height: match &self.direction {
                    Direction::Horizontal => area.height,
                    Direction::Vertical => size,
                }
            };

            position += size;
            (&**c, r)
        }).collect()
    }
}

impl<'a> DrawableSize for Layout<'a> {
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
