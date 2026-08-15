//! Paper hex+alpha is grouped as `RRGGBB_AA`.
#![allow(clippy::unusual_byte_groupings)]

use std::time::Duration;

use crate::motion::{Motion, StyledSlot};
use crate::theme::{paint, rgb, ActiveTheme, Theme, ThemeKind};
use gpui::{
    div, prelude::*, px, App, BoxShadow, Div, FontWeight, IntoElement, Render, RenderOnce,
    SharedString, Stateful, StyleRefinement, Styled, Window,
};

const DEFAULT_SHOW_DELAY: Duration = Duration::from_millis(300);

/// Inverse-glass tooltip matching Paper `Grafik UI` → Tooltips.
///
/// Render it directly for a visible specimen, or attach it to a [`crate::Button`]
/// with [`crate::Button::tooltip`] for delayed hover behavior.
#[derive(Clone, IntoElement)]
pub struct Tooltip {
    label: SharedString,
    theme: Option<Theme>,
    show_delay: Duration,
    style: StyleRefinement,
}

impl Tooltip {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            theme: None,
            show_delay: DEFAULT_SHOW_DELAY,
            style: StyleRefinement::default(),
        }
    }

    /// Override the active app theme for this tooltip only.
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn show_delay(mut self, delay: Duration) -> Self {
        self.show_delay = delay;
        self
    }

    pub fn delay(&self) -> Duration {
        self.show_delay
    }

    pub(crate) fn attach(self, trigger: Stateful<Div>) -> Stateful<Div> {
        let delay = self.show_delay;
        trigger
            .tooltip(move |_, cx| {
                cx.new(|_| TooltipView {
                    tooltip: self.clone(),
                })
                .into()
            })
            .tooltip_show_delay(delay)
    }
}

impl Styled for Tooltip {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Tooltip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = self.theme.unwrap_or_else(|| cx.theme());
        let chrome = tooltip_chrome(theme);

        div()
            .h(px(24.))
            .flex()
            .items_center()
            .flex_shrink_0()
            .px(px(8.))
            .rounded(px(6.))
            .border_1()
            .border_color(chrome.border)
            .bg(chrome.bg)
            .shadow(vec![
                BoxShadow::new(px(0.), px(1.), chrome.inset).inset(),
                BoxShadow::new(px(0.), px(chrome.shadow_y), chrome.shadow)
                    .blur_radius(px(chrome.shadow_blur)),
            ])
            .font_family(theme.font_family)
            .font_weight(FontWeight::MEDIUM)
            .text_size(px(12.))
            .line_height(px(16.))
            .text_color(chrome.fg)
            .refine_style(&self.style)
            .child(self.label)
    }
}

struct TooltipView {
    tooltip: Tooltip,
}

impl Render for TooltipView {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        // GPUI positions tooltips at the pointer. This transparent lead-in keeps
        // the visible chip away from the cursor while retaining edge avoidance.
        div().pt(px(7.)).pl(px(7.)).child(
            Motion::new()
                .id(format!("tooltip-{}", self.tooltip.label))
                .surface_in()
                .child(self.tooltip.clone()),
        )
    }
}

#[derive(Clone, Copy, Debug)]
struct TooltipChrome {
    bg: gpui::Hsla,
    border: gpui::Hsla,
    fg: gpui::Hsla,
    inset: gpui::Hsla,
    shadow: gpui::Hsla,
    shadow_y: f32,
    shadow_blur: f32,
}

fn tooltip_chrome(theme: Theme) -> TooltipChrome {
    match theme.kind {
        ThemeKind::Light => TooltipChrome {
            bg: paint(0x18181B_B8),
            border: paint(0xFFFFFF_29),
            fg: rgb(0xFAFAFA),
            inset: paint(0xFFFFFF_38),
            shadow: paint(0x0F172A_0A),
            shadow_y: 8.0,
            shadow_blur: 20.0,
        },
        ThemeKind::Dark => TooltipChrome {
            bg: rgb(0xFAFAFA),
            border: paint(0xFFFFFF_B8),
            fg: rgb(0x18181B),
            inset: rgb(0xFFFFFF),
            shadow: paint(0x000000_47),
            shadow_y: 8.0,
            shadow_blur: 24.0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_delay_matches_paper() {
        assert_eq!(Tooltip::new("Export PNG").delay(), DEFAULT_SHOW_DELAY);
    }

    #[test]
    fn light_material_matches_paper() {
        let chrome = tooltip_chrome(Theme::light());
        assert_eq!(chrome.bg, paint(0x18181B_B8));
        assert_eq!(chrome.border, paint(0xFFFFFF_29));
        assert_eq!(chrome.fg, rgb(0xFAFAFA));
        assert_eq!(chrome.inset, paint(0xFFFFFF_38));
        assert_eq!(chrome.shadow, paint(0x0F172A_0A));
        assert_eq!((chrome.shadow_y, chrome.shadow_blur), (8.0, 20.0));
    }

    #[test]
    fn dark_material_matches_paper() {
        let chrome = tooltip_chrome(Theme::dark());
        assert_eq!(chrome.bg, rgb(0xFAFAFA));
        assert_eq!(chrome.border, paint(0xFFFFFF_B8));
        assert_eq!(chrome.fg, rgb(0x18181B));
        assert_eq!(chrome.inset, rgb(0xFFFFFF));
        assert_eq!(chrome.shadow, paint(0x000000_47));
        assert_eq!((chrome.shadow_y, chrome.shadow_blur), (8.0, 24.0));
    }
}
