//! Paper hex+alpha is grouped as `RRGGBB_AA`.
#![allow(clippy::unusual_byte_groupings)]

use std::f32::consts::{PI, TAU};

use crate::motion::StyledSlot;
use crate::theme::{ActiveTheme, Theme};
use gpui::{
    canvas, div, point, prelude::FluentBuilder, px, relative, App, Bounds, BoxShadow, Hsla,
    IntoElement, ParentElement, PathBuilder, Pixels, RenderOnce, StyleRefinement, Styled, Window,
};

use crate::button::ButtonVariant;
use crate::chrome::button_chrome;
use crate::spinner::{paint_dot, paint_ring, point_on_circle, spinner_paints, SpinnerTone};

/// Determinate linear progress matching Paper `Glassy UI` → Progress.
///
/// Values are clamped to `0.0..=1.0`. The default geometry is 280×8; width can
/// be changed through [`Styled`] and the fill remains proportional.
#[derive(IntoElement)]
pub struct Progress {
    value: f32,
    theme: Option<Theme>,
    style: StyleRefinement,
}

impl Progress {
    pub fn new(value: f32) -> Self {
        Self {
            value: normalize(value),
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    /// Override the active app theme for this progress indicator only.
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl Styled for Progress {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Progress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = self.theme.unwrap_or_else(|| cx.theme());
        let track = button_chrome(theme, ButtonVariant::Outline);
        let fill = button_chrome(theme, ButtonVariant::Primary);

        div()
            .w(px(280.))
            .h(px(8.))
            .flex_shrink_0()
            .overflow_hidden()
            .rounded(px(4.))
            .border_1()
            .border_color(track.border)
            .bg(track.bg)
            .shadow(vec![
                BoxShadow::new(px(0.), px(1.), track.inset).inset(),
                BoxShadow::new(px(0.), px(track.shadow_y), track.shadow)
                    .blur_radius(px(track.shadow_blur)),
            ])
            .refine_style(&self.style)
            .when(self.value > 0.0, |el| {
                el.child(
                    div()
                        .h_full()
                        .w(relative(self.value))
                        .flex_shrink_0()
                        .rounded(px(4.))
                        .bg(fill.bg)
                        .shadow(vec![BoxShadow::new(px(0.), px(1.), fill.inset).inset()]),
                )
            })
    }
}

/// Determinate 24px circular progress matching Paper `Glassy UI` → Progress.
#[derive(IntoElement)]
pub struct CircularProgress {
    value: f32,
    theme: Option<Theme>,
    style: StyleRefinement,
}

impl CircularProgress {
    pub fn new(value: f32) -> Self {
        Self {
            value: normalize(value),
            theme: None,
            style: StyleRefinement::default().flex_shrink_0(),
        }
    }

    /// Override the active app theme for this progress indicator only.
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl Styled for CircularProgress {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for CircularProgress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = self.theme.unwrap_or_else(|| cx.theme());
        let (track, fill) = spinner_paints(theme, SpinnerTone::Default);
        let value = self.value;

        div()
            .size(px(24.))
            .flex_shrink_0()
            .overflow_hidden()
            .refine_style(&self.style)
            .child(
                canvas(
                    |_, _, _| {},
                    move |bounds, _, window, _| {
                        paint_circular_progress(window, bounds, value, track, fill);
                    },
                )
                .size_full(),
            )
    }
}

fn normalize(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

fn paint_circular_progress(
    window: &mut Window,
    bounds: Bounds<Pixels>,
    value: f32,
    track: Hsla,
    fill: Hsla,
) {
    let stroke = px(2.);
    let center = bounds.center();
    let radius = (bounds.size.width.min(bounds.size.height) / 2.0) * 0.75;
    paint_ring(window, center, radius, stroke, track);

    if value <= 0.0 {
        return;
    }
    if value >= 1.0 {
        paint_ring(window, center, radius, stroke, fill);
        return;
    }

    let start = point_on_circle(center, radius, -PI / 2.0);
    let end = point_on_circle(center, radius, -PI / 2.0 + TAU * value);
    let mut path = PathBuilder::stroke(stroke);
    path.move_to(start);
    path.arc_to(point(radius, radius), px(0.), value > 0.5, true, end);
    if let Ok(path) = path.build() {
        window.paint_path(path, fill);
    }

    let cap = stroke / 2.0;
    paint_dot(window, start, cap, fill);
    paint_dot(window, end, cap, fill);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{paint, rgb};

    #[test]
    fn values_are_clamped() {
        assert_eq!(normalize(-1.0), 0.0);
        assert_eq!(normalize(0.4), 0.4);
        assert_eq!(normalize(2.0), 1.0);
        assert_eq!(normalize(f32::NAN), 0.0);
    }

    #[test]
    fn linear_material_matches_paper() {
        let light_track = button_chrome(Theme::light(), ButtonVariant::Outline);
        let light_fill = button_chrome(Theme::light(), ButtonVariant::Primary);
        assert_eq!(light_track.bg, paint(0xFFFFFF_47));
        assert_eq!(light_track.border, paint(0xFFFFFF_9E));
        assert_eq!(light_fill.bg, paint(0x18181B_B8));
        assert_eq!(light_fill.inset, paint(0xFFFFFF_38));

        let dark_track = button_chrome(Theme::dark(), ButtonVariant::Outline);
        let dark_fill = button_chrome(Theme::dark(), ButtonVariant::Primary);
        assert_eq!(dark_track.bg, paint(0xFFFFFF_0A));
        assert_eq!(dark_track.border, paint(0xFFFFFF_24));
        assert_eq!(dark_fill.bg, paint(0xFFFFFF_29));
        assert_eq!(dark_fill.inset, paint(0xFFFFFF_47));
    }

    #[test]
    fn circular_paints_match_paper() {
        assert_eq!(
            spinner_paints(Theme::light(), SpinnerTone::Default),
            (paint(0x18181B_33), rgb(0x18181B))
        );
        assert_eq!(
            spinner_paints(Theme::dark(), SpinnerTone::Default),
            (paint(0xFAFAFA_2E), rgb(0xFAFAFA))
        );
    }
}
