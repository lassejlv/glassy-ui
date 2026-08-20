//! Paper hex+alpha is grouped as `RRGGBB_AA`.
#![allow(clippy::unusual_byte_groupings)]

use std::time::Duration;

use crate::motion::{Motion, StyledSlot};
use crate::theme::{paint, rgb, ActiveTheme, Theme, ThemeKind};
use gpui::{
    anchored, deferred, div, point, prelude::*, px, relative, App, Corner, Div, FontWeight,
    IntoElement, Pixels, RenderOnce, SharedString, Stateful, StyleRefinement, Styled, Window,
};

use crate::chrome::box_shadow;
use crate::compat::{AccessibilityExt, Role, StyleCompatExt};

const DEFAULT_SHOW_DELAY: Duration = Duration::from_millis(300);
const TOOLTIP_GAP: f32 = 6.0;

/// Inverse-glass tooltip matching Paper `Glassy UI` → Tooltips.
///
/// Render it directly for a visible specimen, or attach it to a [`crate::Button`]
/// with [`crate::Button::tooltip`] for delayed hover behavior.
#[derive(Clone, IntoElement)]
pub struct Tooltip {
    label: SharedString,
    theme: Option<Theme>,
    show_delay: Duration,
    placement: TooltipPlacement,
    style: StyleRefinement,
}

/// Paper placements: above / below / start / end of the trigger.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipPlacement {
    #[default]
    Above,
    Below,
    Start,
    End,
}

impl TooltipPlacement {
    fn anchor(self) -> Corner {
        match self {
            Self::Above => Corner::BottomLeft,
            Self::Below => Corner::TopLeft,
            Self::Start => Corner::TopRight,
            Self::End => Corner::TopLeft,
        }
    }

    fn offset(self, gap: Pixels) -> gpui::Point<Pixels> {
        match self {
            Self::Above => point(px(0.), -gap),
            Self::Below => point(px(0.), gap),
            Self::Start => point(-gap, px(0.)),
            Self::End => point(gap, px(0.)),
        }
    }

    fn marker(self) -> Div {
        let marker = div().absolute().size(px(0.));
        match self {
            Self::Above => marker.left(px(0.)).top(px(0.)),
            Self::Below => marker.left(px(0.)).top(relative(1.)),
            Self::Start => marker.left(px(0.)).top(relative(0.5)),
            Self::End => marker.left(relative(1.)).top(relative(0.5)),
        }
    }
}

struct TooltipAttachState {
    open: bool,
    hover_generation: u64,
}

impl Tooltip {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            theme: None,
            show_delay: DEFAULT_SHOW_DELAY,
            placement: TooltipPlacement::Above,
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

    pub fn placement(mut self, placement: TooltipPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn delay(&self) -> Duration {
        self.show_delay
    }

    pub fn placement_kind(&self) -> TooltipPlacement {
        self.placement
    }

    pub(crate) fn attach(
        self,
        trigger_id: SharedString,
        trigger: Stateful<Div>,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let delay = self.show_delay;
        let placement = self.placement;
        let state_id = SharedString::from(format!("{trigger_id}-tooltip-state"));
        let chip_selector = format!("{trigger_id}-tooltip");
        let state = window.use_keyed_state(state_id, cx, |_, _| TooltipAttachState {
            open: false,
            hover_generation: 0,
        });
        let open = state.read(cx).open;
        let hover_state = state.clone();

        let trigger = trigger.on_hover(move |hovered, window, cx| {
            if *hovered {
                if delay.is_zero() {
                    hover_state.update(cx, |tooltip, cx| {
                        tooltip.open = true;
                        cx.notify();
                    });
                    return;
                }
                hover_state.update(cx, |tooltip, _| {
                    tooltip.hover_generation += 1;
                });
                let generation = hover_state.read(cx).hover_generation;
                let pending = hover_state.downgrade();
                window
                    .spawn(cx, async move |cx| {
                        cx.background_executor().timer(delay).await;
                        pending
                            .update(cx, |tooltip, cx| {
                                if tooltip.hover_generation == generation {
                                    tooltip.open = true;
                                    cx.notify();
                                }
                            })
                            .ok();
                    })
                    .detach();
            } else {
                hover_state.update(cx, |tooltip, cx| {
                    tooltip.hover_generation += 1;
                    tooltip.open = false;
                    cx.notify();
                });
            }
        });

        let gap = px(TOOLTIP_GAP);
        let surface = Motion::new()
            .id(format!("{trigger_id}-tooltip-surface"))
            .surface_in()
            .child(
                div()
                    .id(SharedString::from(chip_selector.clone()))
                    .debug_selector(move || chip_selector.clone())
                    .child(self),
            );
        let popup = match placement {
            TooltipPlacement::Above | TooltipPlacement::Below => placement
                .marker()
                .child(
                    deferred(
                        anchored()
                            .anchor(placement.anchor())
                            .offset(placement.offset(gap))
                            .snap_to_window_with_margin(px(8.))
                            .child(surface),
                    )
                    .with_priority(4),
                )
                .into_any_element(),
            TooltipPlacement::Start => div()
                .absolute()
                .top(px(0.))
                .bottom(px(0.))
                .right(relative(1.))
                .w(px(0.))
                .flex()
                .items_center()
                .justify_end()
                .child(deferred(div().mr(gap).child(surface)).with_priority(4))
                .into_any_element(),
            TooltipPlacement::End => div()
                .absolute()
                .top(px(0.))
                .bottom(px(0.))
                .left(relative(1.))
                .w(px(0.))
                .flex()
                .items_center()
                .justify_start()
                .child(deferred(div().ml(gap).child(surface)).with_priority(4))
                .into_any_element(),
        };

        div()
            .relative()
            .flex()
            .flex_none()
            .self_start()
            .items_center()
            .child(trigger)
            .when(open, |el| el.child(popup))
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
            .id(SharedString::from(format!("tooltip-{}", self.label)))
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
                box_shadow(0., 1., chrome.inset, 0., 0.),
                box_shadow(0., chrome.shadow_y, chrome.shadow, chrome.shadow_blur, 0.),
            ])
            .role(Role::Tooltip)
            .font_family(theme.font_family)
            .font_weight(FontWeight::MEDIUM)
            .text_size(px(12.))
            .line_height(px(16.))
            .text_color(chrome.fg)
            .refine_style(&self.style)
            .child(self.label)
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
    fn default_placement_is_above() {
        assert_eq!(
            Tooltip::new("Export PNG").placement_kind(),
            TooltipPlacement::Above
        );
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
