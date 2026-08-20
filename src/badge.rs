//! Design-spec hex+alpha is grouped as `RRGGBB_AA`.
#![allow(clippy::unusual_byte_groupings)]

use crate::motion::StyledSlot;
use crate::theme::{ActiveTheme, Theme};
use gpui::{
    div, prelude::*, px, App, FontWeight, IntoElement, RenderOnce, SharedString, StyleRefinement,
    Styled, Window,
};

use crate::button::ButtonVariant;
use crate::chrome::{box_shadow, button_chrome, ButtonChrome};

const BADGE_HEIGHT: f32 = 22.0;
const BADGE_RADIUS: f32 = 6.0;
const BADGE_PAD_X: f32 = 8.0;

/// Visual treatment from the design source `Glassy UI` → Badges.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Muted,
    Destructive,
}

impl BadgeVariant {
    fn button_variant(self) -> ButtonVariant {
        match self {
            Self::Default => ButtonVariant::Primary,
            Self::Muted => ButtonVariant::Ghost,
            Self::Destructive => ButtonVariant::Destructive,
        }
    }
}

fn badge_chrome(theme: Theme, variant: BadgeVariant) -> ButtonChrome {
    let mut chrome = button_chrome(theme, variant.button_variant());
    if variant == BadgeVariant::Muted {
        chrome.fg = theme.muted_fg();
    }
    chrome
}

/// A compact, non-interactive status or count chip.
#[derive(IntoElement)]
pub struct Badge {
    label: SharedString,
    variant: BadgeVariant,
    style: StyleRefinement,
}

impl Badge {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            variant: BadgeVariant::Default,
            style: StyleRefinement::default(),
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl Styled for Badge {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let chrome = badge_chrome(theme, self.variant);
        let mut shadows = vec![box_shadow(0., 1., chrome.inset, 0., 0.)];
        if chrome.shadow_blur > 0.0 {
            shadows.push(box_shadow(
                0.,
                chrome.shadow_y,
                chrome.shadow,
                chrome.shadow_blur,
                0.,
            ));
        }

        div()
            .flex()
            .items_center()
            .justify_center()
            .h(px(BADGE_HEIGHT))
            .flex_shrink_0()
            .px(px(BADGE_PAD_X))
            .rounded(px(BADGE_RADIUS))
            .border_1()
            .border_color(chrome.border)
            .bg(chrome.bg)
            .shadow(shadows)
            .cursor_default()
            .refine_style(&self.style)
            .child(
                div()
                    .font_family(theme.font_family)
                    .font_weight(FontWeight::MEDIUM)
                    .text_size(px(12.))
                    .line_height(px(16.))
                    .text_color(chrome.fg)
                    .child(self.label),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{paint, rgb};

    #[test]
    fn metrics_match_spec() {
        assert_eq!(BADGE_HEIGHT, 22.0);
        assert_eq!(BADGE_RADIUS, 6.0);
        assert_eq!(BADGE_PAD_X, 8.0);
    }

    #[test]
    fn light_variants_match_spec() {
        let theme = Theme::light();
        let default = badge_chrome(theme, BadgeVariant::Default);
        assert_eq!(default.bg, paint(0x18181B_B8));
        assert_eq!(default.border, paint(0xFFFFFF_29));
        assert_eq!(default.fg, rgb(0xFAFAFA));

        let muted = badge_chrome(theme, BadgeVariant::Muted);
        assert_eq!(muted.bg, paint(0xFFFFFF_29));
        assert_eq!(muted.border, paint(0xFFFFFF_47));
        assert_eq!(muted.fg, rgb(0xA1A1AA));

        let destructive = badge_chrome(theme, BadgeVariant::Destructive);
        assert_eq!(destructive.bg, paint(0xDC2626_C7));
        assert_eq!(destructive.border, paint(0xFFFFFF_47));
        assert_eq!(destructive.fg, rgb(0xFFFFFF));
    }

    #[test]
    fn dark_variants_match_spec() {
        let theme = Theme::dark();
        let default = badge_chrome(theme, BadgeVariant::Default);
        assert_eq!(default.bg, paint(0xFFFFFF_29));
        assert_eq!(default.border, paint(0xFFFFFF_38));
        assert_eq!(default.fg, rgb(0xFAFAFA));

        let muted = badge_chrome(theme, BadgeVariant::Muted);
        assert_eq!(muted.bg, paint(0xFFFFFF_08));
        assert_eq!(muted.border, paint(0xFFFFFF_0F));
        assert_eq!(muted.fg, rgb(0x71717A));

        let destructive = badge_chrome(theme, BadgeVariant::Destructive);
        assert_eq!(destructive.bg, paint(0xDC2626_9E));
        assert_eq!(destructive.border, paint(0xFFFFFF_29));
        assert_eq!(destructive.fg, rgb(0xFAFAFA));
    }
}
