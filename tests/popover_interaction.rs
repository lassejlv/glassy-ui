use std::{cell::Cell, rc::Rc};

use glassy_ui::{
    init_theme, Popover, PopoverContent, PopoverDescription, PopoverPlacement, PopoverTitle,
};
use gpui::{
    div, prelude::*, px, size, Context, KeyDownEvent, Modifiers, Render, TestAppContext,
    VisualTestContext, Window,
};

struct PopoverHarness {
    placement: PopoverPlacement,
    default_open: bool,
    changes: Rc<Cell<usize>>,
    last_open: Rc<Cell<bool>>,
    background_clicks: Rc<Cell<usize>>,
    trigger_key_received: Rc<Cell<bool>>,
}

impl Render for PopoverHarness {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let changes = self.changes.clone();
        let last_open = self.last_open.clone();
        let background_clicks = self.background_clicks.clone();
        let trigger_key_received = self.trigger_key_received.clone();

        div()
            .size_full()
            .p(px(260.))
            .on_key_down(move |event: &KeyDownEvent, _, _| {
                if event.keystroke.key.as_str() == "x" {
                    trigger_key_received.set(true);
                }
            })
            .child(
                div()
                    .id("popover-background")
                    .debug_selector(|| "POPOVER_BACKGROUND".into())
                    .absolute()
                    .right(px(20.))
                    .bottom(px(20.))
                    .size(px(100.))
                    .on_click(move |_, _, _| {
                        background_clicks.set(background_clicks.get() + 1);
                    }),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(20.))
                    .child(
                        Popover::new("test-popover")
                            .default_open(self.default_open)
                            .placement(self.placement)
                            .trigger_label("Show page metadata")
                            .trigger(
                                div()
                                    .debug_selector(|| "POPOVER_TRIGGER_VISUAL".into())
                                    .size(px(36.)),
                            )
                            .on_open_change(move |open, _, _| {
                                changes.set(changes.get() + 1);
                                last_open.set(open);
                            })
                            .child(
                                PopoverContent::new()
                                    .child(PopoverTitle::new("Home"))
                                    .child(PopoverDescription::new("1440 × 900 · 3 layers")),
                            ),
                    )
                    .child(
                        div()
                            .debug_selector(|| "POPOVER_SENTINEL".into())
                            .w(px(80.))
                            .h(px(12.)),
                    ),
            )
    }
}

struct PopoverTestContext {
    cx: VisualTestContext,
    changes: Rc<Cell<usize>>,
    last_open: Rc<Cell<bool>>,
    background_clicks: Rc<Cell<usize>>,
    trigger_key_received: Rc<Cell<bool>>,
}

fn setup(
    cx: &mut TestAppContext,
    placement: PopoverPlacement,
    default_open: bool,
) -> PopoverTestContext {
    cx.update(init_theme);
    let changes = Rc::new(Cell::new(0));
    let last_open = Rc::new(Cell::new(default_open));
    let background_clicks = Rc::new(Cell::new(0));
    let trigger_key_received = Rc::new(Cell::new(false));
    let window = cx.add_window({
        let changes = changes.clone();
        let last_open = last_open.clone();
        let background_clicks = background_clicks.clone();
        let trigger_key_received = trigger_key_received.clone();
        move |_, _| PopoverHarness {
            placement,
            default_open,
            changes,
            last_open,
            background_clicks,
            trigger_key_received,
        }
    });
    cx.simulate_window_resize(window.into(), size(px(900.), px(700.)));
    cx.run_until_parked();

    PopoverTestContext {
        cx: VisualTestContext::from_window(window.into(), cx),
        changes,
        last_open,
        background_clicks,
        trigger_key_received,
    }
}

fn click_trigger(cx: &mut VisualTestContext) {
    let trigger = cx
        .debug_bounds("test-popover-trigger")
        .expect("popover trigger");
    cx.simulate_click(trigger.center(), Modifiers::default());
}

#[gpui::test]
fn panel_matches_spec_and_does_not_reflow(cx: &mut TestAppContext) {
    let mut test = setup(cx, PopoverPlacement::Bottom, false);
    let sentinel_before = test
        .cx
        .debug_bounds("POPOVER_SENTINEL")
        .expect("sentinel bounds");

    click_trigger(&mut test.cx);

    let trigger = test
        .cx
        .debug_bounds("test-popover-trigger")
        .expect("popover trigger");
    let content = test
        .cx
        .debug_bounds("test-popover-content")
        .expect("popover content");
    let sentinel_after = test
        .cx
        .debug_bounds("POPOVER_SENTINEL")
        .expect("sentinel bounds");

    assert_eq!(content.size.width, px(220.));
    assert_eq!(content.top() - trigger.bottom(), px(6.));
    assert_eq!(content.left(), trigger.left());
    assert_eq!(sentinel_after, sentinel_before);
}

#[gpui::test]
fn top_placement_keeps_a_six_pixel_gap(cx: &mut TestAppContext) {
    let mut test = setup(cx, PopoverPlacement::Top, true);
    let trigger = test
        .cx
        .debug_bounds("test-popover-trigger")
        .expect("popover trigger");
    let content = test
        .cx
        .debug_bounds("test-popover-content")
        .expect("popover content");

    assert_eq!(trigger.top() - content.bottom(), px(6.));
    assert_eq!(content.left(), trigger.left());
}

#[gpui::test]
fn start_placement_keeps_a_six_pixel_gap(cx: &mut TestAppContext) {
    let mut test = setup(cx, PopoverPlacement::Start, true);
    let trigger = test
        .cx
        .debug_bounds("test-popover-trigger")
        .expect("popover trigger");
    let content = test
        .cx
        .debug_bounds("test-popover-content")
        .expect("popover content");

    assert_eq!(trigger.left() - content.right(), px(6.));
    assert_eq!(content.center().y, trigger.center().y);
}

#[gpui::test]
fn end_placement_keeps_a_six_pixel_gap(cx: &mut TestAppContext) {
    let mut test = setup(cx, PopoverPlacement::End, true);
    let trigger = test
        .cx
        .debug_bounds("test-popover-trigger")
        .expect("popover trigger");
    let content = test
        .cx
        .debug_bounds("test-popover-content")
        .expect("popover content");

    assert_eq!(content.left() - trigger.right(), px(6.));
    assert_eq!(content.center().y, trigger.center().y);
}

#[gpui::test]
fn trigger_toggles_without_duplicate_change_events(cx: &mut TestAppContext) {
    let mut test = setup(cx, PopoverPlacement::Bottom, false);

    click_trigger(&mut test.cx);
    assert!(test.cx.debug_bounds("test-popover-content").is_some());
    assert_eq!(test.changes.get(), 1);
    assert!(test.last_open.get());

    click_trigger(&mut test.cx);
    assert!(!test.last_open.get());
    assert_eq!(test.changes.get(), 2);
    assert!(!test.last_open.get());
}

#[gpui::test]
fn outside_click_closes_and_reaches_background(cx: &mut TestAppContext) {
    let mut test = setup(cx, PopoverPlacement::Bottom, false);
    click_trigger(&mut test.cx);
    let background = test
        .cx
        .debug_bounds("POPOVER_BACKGROUND")
        .expect("background bounds");

    test.cx
        .simulate_click(background.center(), Modifiers::default());

    assert!(!test.last_open.get());
    assert_eq!(test.changes.get(), 2);
    assert_eq!(test.background_clicks.get(), 1);
}

#[gpui::test]
fn panel_click_does_not_dismiss(cx: &mut TestAppContext) {
    let mut test = setup(cx, PopoverPlacement::Bottom, false);
    click_trigger(&mut test.cx);
    let content = test
        .cx
        .debug_bounds("test-popover-content")
        .expect("popover content");

    test.cx
        .simulate_click(content.center(), Modifiers::default());

    assert!(test.cx.debug_bounds("test-popover-content").is_some());
    assert_eq!(test.changes.get(), 1);
}

#[gpui::test]
fn escape_closes_and_keeps_focus_on_trigger(cx: &mut TestAppContext) {
    let mut test = setup(cx, PopoverPlacement::Bottom, false);
    click_trigger(&mut test.cx);

    test.cx.simulate_keystrokes("escape");

    assert!(!test.last_open.get());
    assert_eq!(test.changes.get(), 2);
    test.cx.simulate_keystrokes("x");
    assert!(
        test.trigger_key_received.get(),
        "focus should remain on the popover trigger"
    );
}

#[gpui::test]
fn keyboard_reopens_after_escape(cx: &mut TestAppContext) {
    let mut test = setup(cx, PopoverPlacement::Bottom, false);
    click_trigger(&mut test.cx);
    test.cx.simulate_keystrokes("escape");

    test.cx.simulate_keystrokes("enter");

    assert!(test.cx.debug_bounds("test-popover-content").is_some());
    assert_eq!(test.changes.get(), 3);
    assert!(test.last_open.get());
}
