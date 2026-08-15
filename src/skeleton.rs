//! Paper hex+alpha is grouped as `RRGGBB_AA`.
#![allow(clippy::unusual_byte_groupings)]

use std::f32::consts::TAU;
use std::time::Duration;

use crate::motion::StyledSlot;
use crate::theme::{ActiveTheme, Theme};
use gpui::{
    div, px, Animation, AnimationExt as _, App, BoxShadow, IntoElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window,
};

use crate::button::ButtonVariant;
use crate::chrome::button_chrome;

/// Paper Skeleton shapes and their default geometry.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SkeletonShape {
    #[default]
    Text,
    Avatar,
    Control,
}

impl SkeletonShape {
    fn metrics(self) -> (f32, f32, f32) {
        match self {
            Self::Text => (180.0, 12.0, 6.0),
            Self::Avatar => (32.0, 32.0, 16.0),
            Self::Control => (280.0, 36.0, 6.0),
        }
    }
}

/// Pulsing secondary-glass placeholder matching Paper `Grafik UI` → Skeletons.
///
/// GPUI automatically holds repeating animations at their first frame when
/// reduced motion is enabled, so the static state remains the full-strength
/// Paper material.
#[derive(IntoElement)]
pub struct Skeleton {
    id: SharedString,
    shape: SkeletonShape,
    theme: Option<Theme>,
    style: StyleRefinement,
}

impl Skeleton {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            shape: SkeletonShape::Text,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn text(id: impl Into<SharedString>) -> Self {
        Self::new(id)
    }

    pub fn avatar(id: impl Into<SharedString>) -> Self {
        Self::new(id).shape(SkeletonShape::Avatar)
    }

    pub fn control(id: impl Into<SharedString>) -> Self {
        Self::new(id).shape(SkeletonShape::Control)
    }

    pub fn shape(mut self, shape: SkeletonShape) -> Self {
        self.shape = shape;
        self
    }

    /// Override the active app theme for this skeleton only.
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl Styled for Skeleton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Skeleton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = self.theme.unwrap_or_else(|| cx.theme());
        let chrome = button_chrome(theme, ButtonVariant::Secondary);
        let (width, height, radius) = self.shape.metrics();

        let shadows = vec![
            BoxShadow::new(px(0.), px(1.), chrome.inset).inset(),
            BoxShadow::new(px(0.), px(chrome.shadow_y), chrome.shadow)
                .blur_radius(px(chrome.shadow_blur)),
        ];

        div()
            .w(px(width))
            .h(px(height))
            .flex_shrink_0()
            .rounded(px(radius))
            .border_1()
            .border_color(chrome.border)
            .bg(chrome.bg)
            .shadow(shadows)
            .refine_style(&self.style)
            .with_animation(
                self.id,
                Animation::new(Duration::from_millis(1600)).repeat(),
                |el, delta| {
                    // Full-strength at both ends makes the repeating seam invisible.
                    let opacity = 0.8 + 0.2 * (delta * TAU).cos();
                    el.opacity(opacity)
                },
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{paint, Theme};

    #[test]
    fn presets_match_paper_geometry() {
        assert_eq!(SkeletonShape::Text.metrics(), (180.0, 12.0, 6.0));
        assert_eq!(SkeletonShape::Avatar.metrics(), (32.0, 32.0, 16.0));
        assert_eq!(SkeletonShape::Control.metrics(), (280.0, 36.0, 6.0));
    }

    #[test]
    fn secondary_glass_matches_paper() {
        let light = button_chrome(Theme::light(), ButtonVariant::Secondary);
        assert_eq!(light.bg, paint(0xFFFFFF_85));
        assert_eq!(light.border, paint(0xFFFFFF_B8));
        assert_eq!(light.inset, paint(0xFFFFFF_E6));
        assert_eq!(light.shadow, paint(0x0F172A_0F));

        let dark = button_chrome(Theme::dark(), ButtonVariant::Secondary);
        assert_eq!(dark.bg, paint(0xFFFFFF_12));
        assert_eq!(dark.border, paint(0xFFFFFF_1A));
        assert_eq!(dark.inset, paint(0xFFFFFF_1F));
        assert_eq!(dark.shadow, paint(0x000000_2E));
    }
}
