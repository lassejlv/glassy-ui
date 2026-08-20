//! Design-spec hex+alpha is grouped as `RRGGBB_AA`.
#![allow(clippy::unusual_byte_groupings)]

use gpui::{rgba, Hsla};

/// Typeface used on the design spec kit pages.
pub const FONT_FAMILY: &str = "Inter Variable";

/// Light or dark kit surface.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemeKind {
    #[default]
    Light,
    Dark,
}

impl ThemeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    pub fn parse(name: &str) -> Option<Self> {
        match name.trim().to_ascii_lowercase().as_str() {
            "light" => Some(Self::Light),
            "dark" => Some(Self::Dark),
            _ => None,
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

/// Semantic colors for the active scheme.
///
/// Build one with [`Theme::light`] / [`Theme::dark`], tweak public fields,
/// then [`crate::ActiveTheme::set_theme`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theme {
    pub kind: ThemeKind,
    pub font_family: &'static str,
    pub canvas: Hsla,
    pub heading: Hsla,
    pub body: Hsla,
    pub label: Hsla,
    pub on_solid: Hsla,
    pub ink: Hsla,
    pub placeholder: Hsla,
    pub destructive: Hsla,
    pub destructive_soft: Hsla,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            kind: ThemeKind::Light,
            font_family: FONT_FAMILY,
            canvas: rgb(0xE8EAEE),
            heading: rgb(0x18181B),
            body: rgb(0x71717A),
            label: rgb(0xA1A1AA),
            on_solid: rgb(0xFAFAFA),
            ink: rgb(0x18181B),
            placeholder: rgb(0xA1A1AA),
            destructive: rgb(0xDC2626),
            destructive_soft: rgb(0xDC2626),
        }
    }

    pub fn dark() -> Self {
        Self {
            kind: ThemeKind::Dark,
            font_family: FONT_FAMILY,
            canvas: rgb(0x0B0C0F),
            heading: rgb(0xFAFAFA),
            body: rgb(0xA1A1AA),
            label: rgb(0x71717A),
            on_solid: rgb(0xFAFAFA),
            ink: rgb(0xFAFAFA),
            placeholder: rgb(0x71717A),
            destructive: rgb(0xF87171),
            destructive_soft: rgb(0xFCA5A5),
        }
    }

    pub fn for_kind(kind: ThemeKind) -> Self {
        match kind {
            ThemeKind::Light => Self::light(),
            ThemeKind::Dark => Self::dark(),
        }
    }

    pub fn named(name: &str) -> Option<Self> {
        ThemeKind::parse(name).map(Self::for_kind)
    }

    pub fn toggle(self) -> Self {
        Self::for_kind(self.kind.toggle())
    }

    pub fn is_dark(self) -> bool {
        self.kind == ThemeKind::Dark
    }

    pub fn muted_fg(self) -> Hsla {
        match self.kind {
            ThemeKind::Light => rgb(0xA1A1AA),
            ThemeKind::Dark => rgb(0x71717A),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

/// Opaque `0xRRGGBB`.
pub fn rgb(value: u32) -> Hsla {
    gpui::rgb(value).into()
}

/// `0xRRGGBBAA`.
pub fn paint(value: u32) -> Hsla {
    rgba(value).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_and_dark_match_spec() {
        let light = Theme::light();
        assert_eq!(light.canvas, rgb(0xE8EAEE));
        assert_eq!(light.heading, rgb(0x18181B));
        assert_eq!(light.placeholder, rgb(0xA1A1AA));
        assert!(!light.is_dark());

        let dark = Theme::dark();
        assert_eq!(dark.canvas, rgb(0x0B0C0F));
        assert_eq!(dark.heading, rgb(0xFAFAFA));
        assert_eq!(dark.placeholder, rgb(0x71717A));
        assert!(dark.is_dark());
    }

    #[test]
    fn named_and_toggle() {
        assert_eq!(Theme::named("dark").unwrap().kind, ThemeKind::Dark);
        assert_eq!(Theme::named("LIGHT").unwrap().kind, ThemeKind::Light);
        assert!(Theme::named("midnight").is_none());
        assert_eq!(ThemeKind::Light.toggle(), ThemeKind::Dark);
        assert_eq!(Theme::light().toggle().kind, ThemeKind::Dark);
    }
}
