use std::{cell::Cell, rc::Rc};

use gpui::{
    div, point, prelude::*, px, size, Context, FocusHandle, KeyDownEvent, Modifiers, Render,
    TestAppContext, VisualTestContext, Window,
};
use gpui_ui::{init_theme, AlertDialog};

struct AlertDialogHarness {
    open: bool,
    trigger_focus: FocusHandle,
    cancellations: Rc<Cell<usize>>,
    confirmations: Rc<Cell<usize>>,
    background_clicks: Rc<Cell<usize>>,
    trigger_key_received: Rc<Cell<bool>>,
}

impl AlertDialogHarness {
    fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        cancellations: Rc<Cell<usize>>,
        confirmations: Rc<Cell<usize>>,
        background_clicks: Rc<Cell<usize>>,
        trigger_key_received: Rc<Cell<bool>>,
    ) -> Self {
        let trigger_focus = cx.focus_handle().tab_stop(true);
        trigger_focus.focus(window, cx);
        Self {
            open: false,
            trigger_focus,
            cancellations,
            confirmations,
            background_clicks,
            trigger_key_received,
        }
    }
}

impl Render for AlertDialogHarness {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let open_harness = cx.entity();
        let cancel_harness = cx.entity();
        let confirm_harness = cx.entity();
        let cancellations = self.cancellations.clone();
        let confirmations = self.confirmations.clone();
        let background_clicks = self.background_clicks.clone();
        let trigger_key_received = self.trigger_key_received.clone();

        div()
            .size_full()
            .track_focus(&self.trigger_focus)
            .on_key_down(move |event: &KeyDownEvent, _, _| {
                if event.keystroke.key.as_str() == "x" {
                    trigger_key_received.set(true);
                }
            })
            .child(
                div()
                    .id("alert-trigger")
                    .debug_selector(|| "ALERT_TRIGGER".into())
                    .w(px(120.))
                    .h(px(36.))
                    .on_click(move |_, _, cx| {
                        open_harness.update(cx, |harness, cx| {
                            harness.open = true;
                            cx.notify();
                        });
                    }),
            )
            .child(
                div()
                    .id("alert-background")
                    .debug_selector(|| "ALERT_BACKGROUND".into())
                    .absolute()
                    .right(px(0.))
                    .bottom(px(0.))
                    .w(px(120.))
                    .h(px(120.))
                    .on_click(move |_, _, _| {
                        background_clicks.set(background_clicks.get() + 1);
                    }),
            )
            .child(
                AlertDialog::new(
                    "test-alert",
                    "Delete this page?",
                    "Home and everything on it are gone. This cannot be undone.",
                )
                .open(self.open)
                .confirm_label("Delete page")
                .on_cancel(move |_, cx| {
                    cancellations.set(cancellations.get() + 1);
                    cancel_harness.update(cx, |harness, cx| {
                        harness.open = false;
                        cx.notify();
                    });
                })
                .on_confirm(move |_, cx| {
                    confirmations.set(confirmations.get() + 1);
                    confirm_harness.update(cx, |harness, cx| {
                        harness.open = false;
                        cx.notify();
                    });
                }),
            )
    }
}

struct AlertDialogTestContext {
    cx: VisualTestContext,
    cancellations: Rc<Cell<usize>>,
    confirmations: Rc<Cell<usize>>,
    background_clicks: Rc<Cell<usize>>,
    trigger_key_received: Rc<Cell<bool>>,
}

fn setup(cx: &mut TestAppContext) -> AlertDialogTestContext {
    cx.update(init_theme);
    let cancellations = Rc::new(Cell::new(0));
    let confirmations = Rc::new(Cell::new(0));
    let background_clicks = Rc::new(Cell::new(0));
    let trigger_key_received = Rc::new(Cell::new(false));
    let window = cx.open_window(size(px(800.), px(600.)), {
        let cancellations = cancellations.clone();
        let confirmations = confirmations.clone();
        let background_clicks = background_clicks.clone();
        let trigger_key_received = trigger_key_received.clone();
        move |window, cx| {
            AlertDialogHarness::new(
                window,
                cx,
                cancellations,
                confirmations,
                background_clicks,
                trigger_key_received,
            )
        }
    });
    cx.run_until_parked();
    AlertDialogTestContext {
        cx: VisualTestContext::from_window(window.into(), cx),
        cancellations,
        confirmations,
        background_clicks,
        trigger_key_received,
    }
}

fn open_alert(cx: &mut VisualTestContext) {
    let trigger = cx.debug_bounds("ALERT_TRIGGER").expect("alert trigger");
    cx.simulate_click(trigger.center(), Modifiers::default());
    assert!(cx.debug_bounds("test-alert-overlay").is_some());
}

#[gpui::test]
fn alert_dialog_matches_paper_geometry(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    let background_before = test
        .cx
        .debug_bounds("ALERT_BACKGROUND")
        .expect("background bounds");

    open_alert(&mut test.cx);

    let overlay = test
        .cx
        .debug_bounds("test-alert-overlay")
        .expect("alert overlay");
    let panel = test
        .cx
        .debug_bounds("test-alert-panel")
        .expect("alert panel");
    let background_after = test
        .cx
        .debug_bounds("ALERT_BACKGROUND")
        .expect("background bounds");

    assert_eq!(overlay.size, size(px(800.), px(600.)));
    assert_eq!(panel.size, size(px(400.), px(180.)));
    assert_eq!(panel.center(), overlay.center());
    assert_eq!(background_after, background_before);
}

#[gpui::test]
fn safe_cancel_action_receives_initial_focus(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    open_alert(&mut test.cx);

    test.cx.simulate_keystrokes("enter");

    assert_eq!(test.cancellations.get(), 1);
    assert_eq!(test.confirmations.get(), 0);
    assert!(test.cx.debug_bounds("test-alert-overlay").is_none());
}

#[gpui::test]
fn tab_moves_to_destructive_confirmation(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    open_alert(&mut test.cx);

    test.cx.simulate_keystrokes("tab enter");

    assert_eq!(test.cancellations.get(), 0);
    assert_eq!(test.confirmations.get(), 1);
    assert!(test.cx.debug_bounds("test-alert-overlay").is_none());
}

#[gpui::test]
fn scrim_is_locked_and_occludes_background(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    open_alert(&mut test.cx);
    let overlay = test
        .cx
        .debug_bounds("test-alert-overlay")
        .expect("alert overlay");
    let scrim_point = point(overlay.left() + px(20.), overlay.bottom() - px(20.));

    test.cx.simulate_click(scrim_point, Modifiers::default());

    assert_eq!(test.cancellations.get(), 0);
    assert_eq!(test.confirmations.get(), 0);
    assert_eq!(test.background_clicks.get(), 0);
    assert!(test.cx.debug_bounds("test-alert-overlay").is_some());
}

#[gpui::test]
fn escape_cancels_and_restores_focus(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    open_alert(&mut test.cx);

    test.cx.simulate_keystrokes("escape");

    assert_eq!(test.cancellations.get(), 1);
    assert_eq!(test.confirmations.get(), 0);
    assert!(test.cx.debug_bounds("test-alert-overlay").is_none());
    test.cx.simulate_keystrokes("x");
    assert!(
        test.trigger_key_received.get(),
        "focus should return to the trigger"
    );
}
