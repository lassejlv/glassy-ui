use gpui::{Refineable, StyleRefinement, Styled};

/// Merge a [`StyleRefinement`] into any [`Styled`] element.
pub trait StyledSlot: Styled + Sized {
    fn refine_style(mut self, style: &StyleRefinement) -> Self {
        self.style().refine(style);
        self
    }
}

impl<T: Styled> StyledSlot for T {}
