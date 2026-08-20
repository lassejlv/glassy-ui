//! Paper hex+alpha is grouped as `RRGGBB_AA`.
#![allow(clippy::unusual_byte_groupings)]

use crate::motion::StyledSlot;
use crate::theme::{paint, ActiveTheme};
use gpui::{div, prelude::*, px, App, IntoElement, RenderOnce, StyleRefinement, Styled, Window};

/// Horizontal 280×1 or vertical 1×36. Zinc at 12%, not a black rule.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// 1px rule matching Paper `Glassy UI` → Separators.
#[derive(IntoElement)]
pub struct Separator {
    orientation: SeparatorOrientation,
    style: StyleRefinement,
}

impl Separator {
    pub fn new() -> Self {
        Self::horizontal()
    }

    pub fn horizontal() -> Self {
        Self {
            orientation: SeparatorOrientation::Horizontal,
            style: StyleRefinement::default(),
        }
    }

    pub fn vertical() -> Self {
        Self {
            orientation: SeparatorOrientation::Vertical,
            style: StyleRefinement::default(),
        }
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Separator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Separator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = if cx.theme().is_dark() {
            paint(0xFAFAFA_1F)
        } else {
            paint(0x18181B_1F)
        };

        div()
            .flex_shrink_0()
            .bg(color)
            .when(self.orientation == SeparatorOrientation::Horizontal, |el| {
                el.w(px(280.)).h(px(1.))
            })
            .when(self.orientation == SeparatorOrientation::Vertical, |el| {
                el.w(px(1.)).h(px(36.))
            })
            .refine_style(&self.style)
    }
}
