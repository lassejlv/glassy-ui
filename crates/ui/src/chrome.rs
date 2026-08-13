//! Paper hex+alpha is grouped as `RRGGBB_AA`.
#![allow(clippy::unusual_byte_groupings)]

use gpui::Hsla;
use gpui_kit_theme::{paint, rgb, Theme, ThemeKind};

use crate::button::ButtonVariant;

/// Rest / focus / disabled / invalid paint for Input and Textarea.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FieldState {
    #[default]
    Rest,
    Focus,
    Disabled,
    Invalid,
}

/// Per-state paint used by [`crate::Input`].
#[derive(Clone, Copy, Debug)]
pub struct FieldChrome {
    pub bg: Hsla,
    pub border: Hsla,
    pub fg: Hsla,
    pub placeholder: Hsla,
    pub caret: Hsla,
    pub inset: Hsla,
    pub shadow: Hsla,
    pub shadow_y: f32,
    pub shadow_blur: f32,
    pub ring: Option<Hsla>,
}

/// Per-variant paint used by [`crate::Button`].
#[derive(Clone, Copy, Debug)]
pub struct ButtonChrome {
    pub bg: Hsla,
    pub hover_bg: Hsla,
    pub border: Hsla,
    pub fg: Hsla,
    pub inset: Hsla,
    pub shadow: Hsla,
    pub shadow_y: f32,
    pub shadow_blur: f32,
}

pub fn button_chrome(theme: Theme, variant: ButtonVariant) -> ButtonChrome {
    match (theme.kind, variant) {
        (ThemeKind::Light, ButtonVariant::Primary) => ButtonChrome {
            bg: paint(0x18181B_B8),
            hover_bg: paint(0x18181B_CC),
            border: paint(0xFFFFFF_29),
            fg: theme.on_solid,
            inset: paint(0xFFFFFF_38),
            shadow: paint(0x0F172A_1A),
            shadow_y: 8.0,
            shadow_blur: 20.0,
        },
        (ThemeKind::Light, ButtonVariant::Secondary) => ButtonChrome {
            bg: paint(0xFFFFFF_85),
            hover_bg: paint(0xFFFFFF_99),
            border: paint(0xFFFFFF_B8),
            fg: theme.ink,
            inset: paint(0xFFFFFF_E6),
            shadow: paint(0x0F172A_0F),
            shadow_y: 6.0,
            shadow_blur: 16.0,
        },
        (ThemeKind::Light, ButtonVariant::Destructive) => ButtonChrome {
            bg: paint(0xDC2626_C7),
            hover_bg: paint(0xDC2626_DB),
            border: paint(0xFFFFFF_47),
            fg: rgb(0xFFFFFF),
            inset: paint(0xFFFFFF_47),
            shadow: paint(0xDC2626_29),
            shadow_y: 8.0,
            shadow_blur: 20.0,
        },
        (ThemeKind::Light, ButtonVariant::Outline) => ButtonChrome {
            bg: paint(0xFFFFFF_47),
            hover_bg: paint(0xFFFFFF_5C),
            border: paint(0xFFFFFF_9E),
            fg: theme.ink,
            inset: paint(0xFFFFFF_BF),
            shadow: paint(0x0F172A_0A),
            shadow_y: 4.0,
            shadow_blur: 12.0,
        },
        (ThemeKind::Light, ButtonVariant::OutlineDestructive) => ButtonChrome {
            bg: paint(0xFEE2E2_6B),
            hover_bg: paint(0xFEE2E2_85),
            border: paint(0xFECACA_B3),
            fg: theme.destructive,
            inset: paint(0xFFFFFF_B3),
            shadow: paint(0xDC2626_0F),
            shadow_y: 4.0,
            shadow_blur: 12.0,
        },
        (ThemeKind::Light, ButtonVariant::Ghost) => ButtonChrome {
            bg: paint(0xFFFFFF_29),
            hover_bg: paint(0xFFFFFF_3D),
            border: paint(0xFFFFFF_47),
            fg: theme.ink,
            inset: paint(0xFFFFFF_66),
            shadow: gpui::transparent_black(),
            shadow_y: 0.0,
            shadow_blur: 0.0,
        },
        (ThemeKind::Dark, ButtonVariant::Primary) => ButtonChrome {
            bg: paint(0xFFFFFF_29),
            hover_bg: paint(0xFFFFFF_38),
            border: paint(0xFFFFFF_38),
            fg: theme.on_solid,
            inset: paint(0xFFFFFF_47),
            shadow: paint(0x000000_47),
            shadow_y: 8.0,
            shadow_blur: 24.0,
        },
        (ThemeKind::Dark, ButtonVariant::Secondary) => ButtonChrome {
            bg: paint(0xFFFFFF_12),
            hover_bg: paint(0xFFFFFF_1A),
            border: paint(0xFFFFFF_1A),
            fg: theme.on_solid,
            inset: paint(0xFFFFFF_1F),
            shadow: paint(0x000000_2E),
            shadow_y: 6.0,
            shadow_blur: 16.0,
        },
        (ThemeKind::Dark, ButtonVariant::Destructive) => ButtonChrome {
            bg: paint(0xDC2626_9E),
            hover_bg: paint(0xDC2626_B3),
            border: paint(0xFFFFFF_29),
            fg: theme.on_solid,
            inset: paint(0xFFFFFF_2E),
            shadow: paint(0xDC2626_2E),
            shadow_y: 8.0,
            shadow_blur: 20.0,
        },
        (ThemeKind::Dark, ButtonVariant::Outline) => ButtonChrome {
            bg: paint(0xFFFFFF_0A),
            hover_bg: paint(0xFFFFFF_14),
            border: paint(0xFFFFFF_24),
            fg: theme.on_solid,
            inset: paint(0xFFFFFF_1A),
            shadow: paint(0x000000_29),
            shadow_y: 4.0,
            shadow_blur: 12.0,
        },
        (ThemeKind::Dark, ButtonVariant::OutlineDestructive) => ButtonChrome {
            bg: paint(0x7F1D1D_47),
            hover_bg: paint(0x7F1D1D_5C),
            border: paint(0xF87171_47),
            fg: theme.destructive_soft,
            inset: paint(0xFFFFFF_14),
            shadow: gpui::transparent_black(),
            shadow_y: 0.0,
            shadow_blur: 0.0,
        },
        (ThemeKind::Dark, ButtonVariant::Ghost) => ButtonChrome {
            bg: paint(0xFFFFFF_08),
            hover_bg: paint(0xFFFFFF_12),
            border: paint(0xFFFFFF_0F),
            fg: theme.on_solid,
            inset: paint(0xFFFFFF_0F),
            shadow: gpui::transparent_black(),
            shadow_y: 0.0,
            shadow_blur: 0.0,
        },
    }
}

pub fn field_chrome(theme: Theme, state: FieldState) -> FieldChrome {
    match (theme.kind, state) {
        (ThemeKind::Light, FieldState::Rest) => FieldChrome {
            bg: paint(0xFFFFFF_47),
            border: paint(0xFFFFFF_9E),
            fg: theme.ink,
            placeholder: theme.placeholder,
            caret: theme.ink,
            inset: paint(0xFFFFFF_BF),
            shadow: paint(0x0F172A_0A),
            shadow_y: 4.0,
            shadow_blur: 12.0,
            ring: None,
        },
        (ThemeKind::Light, FieldState::Focus) => FieldChrome {
            bg: paint(0xFFFFFF_5C),
            border: paint(0x18181B_47),
            fg: theme.ink,
            placeholder: theme.placeholder,
            caret: rgb(0x18181B),
            inset: paint(0xFFFFFF_BF),
            shadow: paint(0x0F172A_0A),
            shadow_y: 4.0,
            shadow_blur: 12.0,
            ring: Some(paint(0x18181B_24)),
        },
        (ThemeKind::Light, FieldState::Disabled) => FieldChrome {
            bg: paint(0xFFFFFF_29),
            border: paint(0xFFFFFF_47),
            fg: theme.muted_fg(),
            placeholder: theme.muted_fg(),
            caret: theme.muted_fg(),
            inset: paint(0xFFFFFF_66),
            shadow: gpui::transparent_black(),
            shadow_y: 0.0,
            shadow_blur: 0.0,
            ring: None,
        },
        (ThemeKind::Light, FieldState::Invalid) => FieldChrome {
            bg: paint(0xFEE2E2_6B),
            border: paint(0xFECACA_B3),
            fg: theme.ink,
            placeholder: theme.placeholder,
            caret: theme.ink,
            inset: paint(0xFFFFFF_B3),
            shadow: paint(0xDC2626_0F),
            shadow_y: 4.0,
            shadow_blur: 12.0,
            ring: None,
        },
        (ThemeKind::Dark, FieldState::Rest) => FieldChrome {
            bg: paint(0xFFFFFF_0A),
            border: paint(0xFFFFFF_24),
            fg: theme.ink,
            placeholder: theme.placeholder,
            caret: theme.ink,
            inset: paint(0xFFFFFF_1A),
            shadow: paint(0x000000_29),
            shadow_y: 4.0,
            shadow_blur: 12.0,
            ring: None,
        },
        (ThemeKind::Dark, FieldState::Focus) => FieldChrome {
            bg: paint(0xFFFFFF_14),
            border: paint(0xFFFFFF_38),
            fg: theme.ink,
            placeholder: theme.placeholder,
            caret: rgb(0xFAFAFA),
            inset: paint(0xFFFFFF_1A),
            shadow: paint(0x000000_29),
            shadow_y: 4.0,
            shadow_blur: 12.0,
            ring: Some(paint(0xFFFFFF_24)),
        },
        (ThemeKind::Dark, FieldState::Disabled) => FieldChrome {
            bg: paint(0xFFFFFF_08),
            border: paint(0xFFFFFF_0F),
            fg: theme.muted_fg(),
            placeholder: theme.muted_fg(),
            caret: theme.muted_fg(),
            inset: paint(0xFFFFFF_0F),
            shadow: gpui::transparent_black(),
            shadow_y: 0.0,
            shadow_blur: 0.0,
            ring: None,
        },
        (ThemeKind::Dark, FieldState::Invalid) => FieldChrome {
            bg: paint(0x7F1D1D_47),
            border: paint(0xF87171_47),
            fg: theme.ink,
            placeholder: theme.placeholder,
            caret: theme.ink,
            inset: paint(0xFFFFFF_14),
            shadow: gpui::transparent_black(),
            shadow_y: 0.0,
            shadow_blur: 0.0,
            ring: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::button::ButtonVariant;

    #[test]
    fn light_primary_matches_paper() {
        let chrome = button_chrome(Theme::light(), ButtonVariant::Primary);
        assert_eq!(chrome.bg, paint(0x18181B_B8));
        assert_eq!(chrome.border, paint(0xFFFFFF_29));
        assert_eq!(chrome.shadow_y, 8.0);
        assert_eq!(chrome.shadow_blur, 20.0);
    }

    #[test]
    fn dark_primary_is_white_glass() {
        let chrome = button_chrome(Theme::dark(), ButtonVariant::Primary);
        assert_eq!(chrome.bg, paint(0xFFFFFF_29));
        assert_eq!(chrome.border, paint(0xFFFFFF_38));
        assert_eq!(chrome.shadow_blur, 24.0);
    }

    #[test]
    fn light_focus_matches_paper() {
        let chrome = field_chrome(Theme::light(), FieldState::Focus);
        assert_eq!(chrome.bg, paint(0xFFFFFF_5C));
        assert_eq!(chrome.border, paint(0x18181B_47));
        assert_eq!(chrome.ring, Some(paint(0x18181B_24)));
    }

    #[test]
    fn dark_invalid_matches_paper() {
        let chrome = field_chrome(Theme::dark(), FieldState::Invalid);
        assert_eq!(chrome.bg, paint(0x7F1D1D_47));
        assert_eq!(chrome.border, paint(0xF87171_47));
    }
}
