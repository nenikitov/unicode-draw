use tui::backend::Backend;

use super::{
    drawable::*,
};

pub struct Spacer {
    sizes: (Size, Size)
}

impl Spacer {
    pub fn new(size: (u16, u16), flexible: (bool, bool)) -> Self {
        Self {
            sizes: (
                Size {
                    size: size.0,
                    flexible: flexible.0
                },
                Size {
                    size: size.1,
                    flexible: flexible.1
                }
            )
        }
    }

    pub fn new_simple(size: u16, flexible: bool) -> Self {
        Self::new((size, size), (flexible, flexible))
    }

    pub fn new_simple_flexible(size: u16) -> Self {
        Self::new_simple(size, true)
    }
}

impl <B: Backend> WidgetRender<B> for Spacer {
    fn render(&self, f: &mut tui::Frame<B>, target: tui::layout::Rect) {}
}

impl WidgetSized for Spacer {
    fn size_preferred(&self) -> (Size, Size) {
        self.sizes
    }
}
