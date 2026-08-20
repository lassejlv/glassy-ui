use std::{cell::Cell, rc::Rc};

use gpui::{
    div, point, prelude::*, px, size, Context, FocusHandle, KeyDownEvent, Modifiers, Render,
    TestAppContext, VisualTestContext, Window,
};
use gpui_ui::{
    init_theme, Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
};

struct DialogHarness {
    open: bool,
    trigger_focus: FocusHandle,
    dismissals: Rc<Cell<usize>>,
    background_clicks: Rc<Cell<usize>>,
    trigger_key_received: Rc<Cell<bool>>,
}

impl DialogHarness {
    fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        dismissals: Rc<Cell<usize>>,
        background_clicks: Rc<Cell<usize>>,
        trigger_key_received: Rc<Cell<bool>>,
    ) -> Self {
        let trigger_focus = cx.focus_handle().tab_stop(true);
        trigger_focus.focus(window, cx);
        Self {
            open: false,
            trigger_focus,
            dismissals,
            background_clicks,
            trigger_key_received,
        }
    }
}

impl Render for DialogHarness {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let open_harness = cx.entity();
        let dismiss_harness = cx.entity();
        let dismissals = self.dismissals.clone();
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
                    .id("dialog-trigger")
                    .debug_selector(|| "DIALOG_TRIGGER".into())
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
                    .id("dialog-background")
                    .debug_selector(|| "DIALOG_BACKGROUND".into())
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
                Dialog::new("test-dialog")
                    .open(self.open)
                    .on_dismiss(move |_, cx| {
                        dismissals.set(dismissals.get() + 1);
                        dismiss_harness.update(cx, |harness, cx| {
                            harness.open = false;
                            cx.notify();
                        });
                    })
                    .child(
                        DialogContent::new()
                            .child(
                                DialogHeader::new()
                                    .child(DialogTitle::new("Rename artboard"))
                                    .child(DialogDescription::new(
                                        "The page keeps the same ID. Only the label changes.",
                                    )),
                            )
                            .child(div().h(px(36.)))
                            .child(DialogFooter::new().child(div().w(px(80.)).h(px(36.)))),
                    ),
            )
    }
}

struct DialogTestContext {
    cx: VisualTestContext,
    dismissals: Rc<Cell<usize>>,
    background_clicks: Rc<Cell<usize>>,
    trigger_key_received: Rc<Cell<bool>>,
}

fn setup(cx: &mut TestAppContext) -> DialogTestContext {
    cx.update(init_theme);
    let dismissals = Rc::new(Cell::new(0));
    let background_clicks = Rc::new(Cell::new(0));
    let trigger_key_received = Rc::new(Cell::new(false));
    let window = cx.open_window(size(px(800.), px(600.)), {
        let dismissals = dismissals.clone();
        let background_clicks = background_clicks.clone();
        let trigger_key_received = trigger_key_received.clone();
        move |window, cx| {
            DialogHarness::new(
                window,
                cx,
                dismissals,
                background_clicks,
                trigger_key_received,
            )
        }
    });
    cx.run_until_parked();
    DialogTestContext {
        cx: VisualTestContext::from_window(window.into(), cx),
        dismissals,
        background_clicks,
        trigger_key_received,
    }
}

fn open_dialog(cx: &mut VisualTestContext) {
    let trigger = cx.debug_bounds("DIALOG_TRIGGER").expect("dialog trigger");
    cx.simulate_click(trigger.center(), Modifiers::default());
    assert!(cx.debug_bounds("test-dialog-overlay").is_some());
}

#[gpui::test]
fn dialog_is_centered_without_reflowing_content(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    let background_before = test
        .cx
        .debug_bounds("DIALOG_BACKGROUND")
        .expect("background bounds");

    open_dialog(&mut test.cx);

    let overlay = test
        .cx
        .debug_bounds("test-dialog-overlay")
        .expect("dialog overlay");
    let panel = test
        .cx
        .debug_bounds("test-dialog-panel")
        .expect("dialog panel");
    let background_after = test
        .cx
        .debug_bounds("DIALOG_BACKGROUND")
        .expect("background bounds");

    assert_eq!(overlay.size, size(px(800.), px(600.)));
    assert_eq!(panel.size.width, px(400.));
    assert_eq!(panel.center(), overlay.center());
    assert_eq!(background_after, background_before);
}

#[gpui::test]
fn escape_dismisses_and_restores_focus(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    open_dialog(&mut test.cx);

    test.cx.simulate_keystrokes("escape");

    assert_eq!(test.dismissals.get(), 1);
    assert!(test.cx.debug_bounds("test-dialog-overlay").is_none());
    test.cx.simulate_keystrokes("x");
    assert!(
        test.trigger_key_received.get(),
        "focus should return to the trigger"
    );
}

#[gpui::test]
fn panel_click_does_not_dismiss(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    open_dialog(&mut test.cx);
    let panel = test
        .cx
        .debug_bounds("test-dialog-panel")
        .expect("dialog panel");

    test.cx.simulate_click(panel.center(), Modifiers::default());

    assert_eq!(test.dismissals.get(), 0);
    assert!(test.cx.debug_bounds("test-dialog-overlay").is_some());
}

#[gpui::test]
fn scrim_click_dismisses_without_reaching_background(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    open_dialog(&mut test.cx);
    let overlay = test
        .cx
        .debug_bounds("test-dialog-overlay")
        .expect("dialog overlay");
    let scrim_point = point(overlay.left() + px(20.), overlay.bottom() - px(20.));

    test.cx.simulate_click(scrim_point, Modifiers::default());

    assert_eq!(test.dismissals.get(), 1);
    assert_eq!(test.background_clicks.get(), 0);
    assert!(test.cx.debug_bounds("test-dialog-overlay").is_none());
}
