use std::time::Duration;

use glassy_ui::{init_theme, Button, ButtonVariant, IconName, Tooltip, TooltipPlacement};
use gpui::{
    div, prelude::*, px, size, Context, Modifiers, Render, TestAppContext, VisualTestContext,
    Window,
};

struct TooltipHarness {
    placement: TooltipPlacement,
}

impl Render for TooltipHarness {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().flex().flex_col().p(px(200.)).child(
            Button::icon_only("tip-trigger", IconName::Download)
                .variant(ButtonVariant::Ghost)
                .tooltip(
                    Tooltip::new("Export PNG")
                        .placement(self.placement)
                        .show_delay(Duration::ZERO),
                ),
        )
    }
}

fn setup(cx: &mut TestAppContext, placement: TooltipPlacement) -> VisualTestContext {
    cx.update(init_theme);
    let window = cx.add_window(move |_, _| TooltipHarness { placement });
    cx.simulate_window_resize(window.into(), size(px(800.), px(600.)));
    cx.run_until_parked();
    VisualTestContext::from_window(window.into(), cx)
}

fn hover_trigger(cx: &mut VisualTestContext) {
    let trigger = cx.debug_bounds("tip-trigger").expect("trigger");
    cx.simulate_mouse_move(trigger.center(), None, Modifiers::default());
    cx.run_until_parked();
}

#[gpui::test]
fn above_keeps_a_six_pixel_gap(cx: &mut TestAppContext) {
    let mut cx = setup(cx, TooltipPlacement::Above);
    hover_trigger(&mut cx);
    let trigger = cx.debug_bounds("tip-trigger").expect("trigger");
    let tip = cx.debug_bounds("tip-trigger-tooltip").expect("tooltip");
    assert_eq!(trigger.top() - tip.bottom(), px(6.));
}

#[gpui::test]
fn below_keeps_a_six_pixel_gap(cx: &mut TestAppContext) {
    let mut cx = setup(cx, TooltipPlacement::Below);
    hover_trigger(&mut cx);
    let trigger = cx.debug_bounds("tip-trigger").expect("trigger");
    let tip = cx.debug_bounds("tip-trigger-tooltip").expect("tooltip");
    assert_eq!(tip.top() - trigger.bottom(), px(6.));
}

#[gpui::test]
fn start_keeps_a_six_pixel_gap(cx: &mut TestAppContext) {
    let mut cx = setup(cx, TooltipPlacement::Start);
    hover_trigger(&mut cx);
    let trigger = cx.debug_bounds("tip-trigger").expect("trigger");
    let tip = cx.debug_bounds("tip-trigger-tooltip").expect("tooltip");
    assert_eq!(trigger.left() - tip.right(), px(6.));
}

#[gpui::test]
fn end_keeps_a_six_pixel_gap(cx: &mut TestAppContext) {
    let mut cx = setup(cx, TooltipPlacement::End);
    hover_trigger(&mut cx);
    let trigger = cx.debug_bounds("tip-trigger").expect("trigger");
    let tip = cx.debug_bounds("tip-trigger-tooltip").expect("tooltip");
    assert_eq!(tip.left() - trigger.right(), px(6.));
}
