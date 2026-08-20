//! Design-spec hex+alpha is grouped as `RRGGBB_AA`.
#![allow(clippy::unusual_byte_groupings)]

use std::f32::consts::PI;
use std::time::Duration;

use crate::motion::StyledSlot;
use crate::theme::{paint, rgb, ActiveTheme, Theme, ThemeKind};
use gpui::{
    canvas, div, point, px, Animation, AnimationExt as _, App, Bounds, Hsla, IntoElement,
    ParentElement, PathBuilder, Pixels, Point, RenderOnce, StyleRefinement, Styled, Window,
};

/// Design-spec sizes: 16 / 20 / 24 / 32.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpinnerSize {
    Small,
    #[default]
    Default,
    Large,
    Display,
}

impl SpinnerSize {
    pub fn px(self) -> f32 {
        match self {
            Self::Small => 16.0,
            Self::Default => 20.0,
            Self::Large => 24.0,
            Self::Display => 32.0,
        }
    }

    pub fn stroke(self) -> f32 {
        match self {
            Self::Small => 1.5,
            Self::Default => 1.75,
            Self::Large => 2.0,
            Self::Display => 2.25,
        }
    }
}

/// Design-spec color row: Default / Muted / Inverse / Destructive.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpinnerTone {
    #[default]
    Default,
    Muted,
    Inverse,
    Destructive,
}

/// Indeterminate spinner. Track + 120° arc, one turn every 700ms.
#[derive(IntoElement)]
pub struct Spinner {
    size_px: f32,
    tone: SpinnerTone,
    arc: Option<Hsla>,
    track: Option<Hsla>,
    style: StyleRefinement,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            size_px: SpinnerSize::Default.px(),
            tone: SpinnerTone::Default,
            arc: None,
            track: None,
            style: StyleRefinement::default().flex_shrink_0(),
        }
    }

    pub fn size(mut self, size: SpinnerSize) -> Self {
        self.size_px = size.px();
        self
    }

    pub fn px(mut self, size: impl Into<Pixels>) -> Self {
        self.size_px = f32::from(size.into());
        self
    }

    pub fn tone(mut self, tone: SpinnerTone) -> Self {
        self.tone = tone;
        self
    }

    /// Paint the arc with `color`. Track is the same hue at 22% alpha.
    pub fn color(mut self, color: Hsla) -> Self {
        self.arc = Some(color);
        self
    }

    pub fn track_color(mut self, color: Hsla) -> Self {
        self.track = Some(color);
        self
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Spinner {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Spinner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let size_px = self.size_px;
        let stroke = stroke_for_size(size_px);
        let (track, arc) = resolve_paints(cx.theme(), self.tone, self.arc, self.track);

        div()
            .size(px(size_px))
            .flex_shrink_0()
            .overflow_hidden()
            .refine_style(&self.style)
            .with_animation(
                "spinner-rotate",
                Animation::new(Duration::from_millis(700)).repeat(),
                move |el, delta| {
                    el.child(
                        canvas(
                            |_, _, _| {},
                            move |bounds, _, window, _| {
                                paint_spinner(window, bounds, stroke, track, arc, delta);
                            },
                        )
                        .size_full(),
                    )
                },
            )
    }
}

fn stroke_for_size(size: f32) -> f32 {
    if (size - 14.0).abs() < f32::EPSILON || (size - 16.0).abs() < f32::EPSILON {
        1.5
    } else if (size - 20.0).abs() < f32::EPSILON {
        1.75
    } else if (size - 24.0).abs() < f32::EPSILON {
        2.0
    } else if (size - 32.0).abs() < f32::EPSILON {
        2.25
    } else {
        size * 1.75 / 20.0
    }
}

fn resolve_paints(
    theme: Theme,
    tone: SpinnerTone,
    arc: Option<Hsla>,
    track: Option<Hsla>,
) -> (Hsla, Hsla) {
    let (spec_track, spec_arc) = spinner_paints(theme, tone);
    let arc = arc.unwrap_or(spec_arc);
    let track = track.unwrap_or_else(|| {
        if arc == spec_arc {
            spec_track
        } else {
            arc.alpha(0.22)
        }
    });
    (track, arc)
}

/// `(track, arc)` from the Spinners design page's computed styles.
pub fn spinner_paints(theme: Theme, tone: SpinnerTone) -> (Hsla, Hsla) {
    match (theme.kind, tone) {
        (ThemeKind::Light, SpinnerTone::Default) => (paint(0x18181B_33), rgb(0x18181B)),
        (ThemeKind::Light, SpinnerTone::Muted) => (paint(0x52525B_47), rgb(0x52525B)),
        (ThemeKind::Light, SpinnerTone::Inverse) => (paint(0xFAFAFA_38), rgb(0xFAFAFA)),
        (ThemeKind::Light, SpinnerTone::Destructive) => (paint(0xDC2626_33), rgb(0xDC2626)),
        (ThemeKind::Dark, SpinnerTone::Default) => (paint(0xFAFAFA_2E), rgb(0xFAFAFA)),
        (ThemeKind::Dark, SpinnerTone::Muted) => (paint(0xA1A1AA_47), rgb(0xA1A1AA)),
        (ThemeKind::Dark, SpinnerTone::Inverse) => (paint(0x18181B_2E), rgb(0x18181B)),
        (ThemeKind::Dark, SpinnerTone::Destructive) => (paint(0xF87171_47), rgb(0xF87171)),
    }
}

fn paint_spinner(
    window: &mut Window,
    bounds: Bounds<Pixels>,
    stroke: f32,
    track: Hsla,
    arc: Hsla,
    delta: f32,
) {
    let stroke = px(stroke);
    let center = bounds.center();
    let radius = (bounds.size.width.min(bounds.size.height) / 2.0) * 0.75;

    paint_ring(window, center, radius, stroke, track);

    let turn = delta * 2.0 * PI;
    let start_angle = -PI / 2.0 + turn;
    let end_angle = start_angle + (2.0 * PI / 3.0);
    let start = point_on_circle(center, radius, start_angle);
    let end = point_on_circle(center, radius, end_angle);

    let mut arc_path = PathBuilder::stroke(stroke);
    arc_path.move_to(start);
    arc_path.arc_to(point(radius, radius), px(0.), false, true, end);
    if let Ok(path) = arc_path.build() {
        window.paint_path(path, arc);
    }

    let cap = stroke / 2.0;
    paint_dot(window, start, cap, arc);
    paint_dot(window, end, cap, arc);
}

pub(crate) fn paint_ring(
    window: &mut Window,
    center: Point<Pixels>,
    radius: Pixels,
    stroke: Pixels,
    color: Hsla,
) {
    let mut builder = PathBuilder::stroke(stroke);
    builder.move_to(point(center.x + radius, center.y));
    builder.arc_to(
        point(radius, radius),
        px(0.),
        false,
        true,
        point(center.x - radius, center.y),
    );
    builder.arc_to(
        point(radius, radius),
        px(0.),
        false,
        true,
        point(center.x + radius, center.y),
    );
    builder.close();
    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}

pub(crate) fn paint_dot(window: &mut Window, origin: Point<Pixels>, radius: Pixels, color: Hsla) {
    if radius <= px(0.) {
        return;
    }
    let mut builder = PathBuilder::fill();
    builder.move_to(point(origin.x + radius, origin.y));
    builder.arc_to(
        point(radius, radius),
        px(0.),
        false,
        true,
        point(origin.x - radius, origin.y),
    );
    builder.arc_to(
        point(radius, radius),
        px(0.),
        false,
        true,
        point(origin.x + radius, origin.y),
    );
    builder.close();
    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}

pub(crate) fn point_on_circle(center: Point<Pixels>, radius: Pixels, angle: f32) -> Point<Pixels> {
    point(
        center.x + radius * angle.cos(),
        center.y + radius * angle.sin(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_match_spec() {
        assert_eq!(SpinnerSize::Small.px(), 16.0);
        assert_eq!(SpinnerSize::Small.stroke(), 1.5);
        assert_eq!(SpinnerSize::Default.px(), 20.0);
        assert_eq!(SpinnerSize::Default.stroke(), 1.75);
        assert_eq!(SpinnerSize::Large.px(), 24.0);
        assert_eq!(SpinnerSize::Large.stroke(), 2.0);
        assert_eq!(SpinnerSize::Display.px(), 32.0);
        assert_eq!(SpinnerSize::Display.stroke(), 2.25);
        assert_eq!(stroke_for_size(14.0), 1.5);
    }

    #[test]
    fn light_default_matches_spec() {
        let (track, arc) = spinner_paints(Theme::light(), SpinnerTone::Default);
        assert_eq!(arc, rgb(0x18181B));
        assert_eq!(track, paint(0x18181B_33));
    }

    #[test]
    fn dark_muted_matches_spec() {
        let (track, arc) = spinner_paints(Theme::dark(), SpinnerTone::Muted);
        assert_eq!(arc, rgb(0xA1A1AA));
        assert_eq!(track, paint(0xA1A1AA_47));
    }
}
