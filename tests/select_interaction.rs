use std::{cell::Cell, rc::Rc};

use gpui::{
    div, point, prelude::*, px, size, Context, Modifiers, Render, TestAppContext,
    VisualTestContext, Window,
};
use gpui_ui::{init_theme, Select, SelectItem};

struct SelectHarness {
    first_picked: Rc<Cell<bool>>,
}

impl Render for SelectHarness {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let items = [
            SelectItem::new("PNG"),
            SelectItem::new("SVG"),
            SelectItem::new("PDF").disabled(true),
        ];
        let first_picked = self.first_picked.clone();

        div()
            .flex()
            .flex_col()
            .gap(px(16.))
            .child(
                div()
                    .flex()
                    .gap(px(16.))
                    .child(
                        div().debug_selector(|| "FIRST_SELECT".into()).child(
                            Select::new("test-first")
                                .items(items.clone())
                                .on_select(move |_, _, _| first_picked.set(true)),
                        ),
                    )
                    .child(
                        div()
                            .debug_selector(|| "SECOND_SELECT".into())
                            .child(Select::new("test-second").items(items)),
                    ),
            )
            .child(div().debug_selector(|| "SENTINEL".into()).h(px(10.)))
    }
}

fn setup(cx: &mut TestAppContext) -> (VisualTestContext, Rc<Cell<bool>>) {
    cx.update(init_theme);
    let first_picked = Rc::new(Cell::new(false));
    let window = cx.open_window(size(px(800.), px(600.)), {
        let first_picked = first_picked.clone();
        move |_, _| SelectHarness { first_picked }
    });
    cx.run_until_parked();
    (
        VisualTestContext::from_window(window.into(), cx),
        first_picked,
    )
}

fn trigger_position(
    cx: &mut VisualTestContext,
    selector: &'static str,
) -> gpui::Point<gpui::Pixels> {
    let bounds = cx.debug_bounds(selector).expect("select bounds");
    bounds.origin + point(px(20.), px(18.))
}

fn first_row_position(cx: &mut VisualTestContext) -> gpui::Point<gpui::Pixels> {
    let bounds = cx
        .debug_bounds("FIRST_SELECT")
        .expect("first select bounds");
    bounds.origin + point(px(20.), px(60.))
}

#[gpui::test]
fn opening_menu_does_not_reflow_surrounding_content(cx: &mut TestAppContext) {
    let (mut cx, _) = setup(cx);
    let sentinel_before = cx.debug_bounds("SENTINEL").expect("sentinel");

    let trigger = trigger_position(&mut cx, "FIRST_SELECT");
    cx.simulate_click(trigger, Modifiers::default());

    let sentinel_after = cx.debug_bounds("SENTINEL").expect("sentinel");
    assert_eq!(sentinel_after.origin, sentinel_before.origin);
}

#[gpui::test]
fn popup_keeps_an_eight_pixel_trigger_gap(cx: &mut TestAppContext) {
    let (mut cx, _) = setup(cx);
    let trigger = trigger_position(&mut cx, "FIRST_SELECT");
    cx.simulate_click(trigger, Modifiers::default());

    let trigger_bounds = cx
        .debug_bounds("test-first-trigger")
        .expect("trigger bounds");
    let popup_bounds = cx.debug_bounds("test-first-popup").expect("popup bounds");

    assert_eq!(popup_bounds.top() - trigger_bounds.bottom(), px(8.));
}

#[gpui::test]
fn opening_another_select_closes_the_first(cx: &mut TestAppContext) {
    let (mut cx, first_picked) = setup(cx);
    let first_trigger = trigger_position(&mut cx, "FIRST_SELECT");
    cx.simulate_click(first_trigger, Modifiers::default());
    let second_trigger = trigger_position(&mut cx, "SECOND_SELECT");
    cx.simulate_click(second_trigger, Modifiers::default());

    let former_first_row = first_row_position(&mut cx);
    cx.simulate_click(former_first_row, Modifiers::default());

    assert!(!first_picked.get(), "the first menu should have closed");
}

#[gpui::test]
fn outside_click_closes_select(cx: &mut TestAppContext) {
    let (mut cx, first_picked) = setup(cx);
    let trigger = trigger_position(&mut cx, "FIRST_SELECT");
    cx.simulate_click(trigger, Modifiers::default());
    cx.simulate_click(point(px(700.), px(500.)), Modifiers::default());

    let former_first_row = first_row_position(&mut cx);
    cx.simulate_click(former_first_row, Modifiers::default());

    assert!(!first_picked.get(), "outside click should close the menu");
}

#[gpui::test]
fn escape_closes_select(cx: &mut TestAppContext) {
    let (mut cx, first_picked) = setup(cx);
    let trigger = trigger_position(&mut cx, "FIRST_SELECT");
    cx.simulate_click(trigger, Modifiers::default());
    cx.simulate_keystrokes("escape");

    let former_first_row = first_row_position(&mut cx);
    cx.simulate_click(former_first_row, Modifiers::default());

    assert!(!first_picked.get(), "escape should close the menu");
}

#[gpui::test]
fn arrows_and_enter_select_an_item(cx: &mut TestAppContext) {
    let (mut cx, first_picked) = setup(cx);
    let trigger = trigger_position(&mut cx, "FIRST_SELECT");
    cx.simulate_click(trigger, Modifiers::default());
    cx.simulate_keystrokes("down enter");

    assert!(
        first_picked.get(),
        "arrow navigation followed by enter should select"
    );
}
