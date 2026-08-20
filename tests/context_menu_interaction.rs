use std::{cell::Cell, rc::Rc};

use glassy_ui::{init_theme, ContextMenu, DropdownMenuEntry, DropdownMenuItem};
use gpui::{
    div, point, prelude::*, px, size, Context, Modifiers, MouseButton, Render, TestAppContext,
    VisualTestContext, Window,
};

struct ContextHarness {
    selected: Rc<Cell<usize>>,
    open: Rc<Cell<bool>>,
    background_clicks: Rc<Cell<usize>>,
    target_key_received: Rc<Cell<bool>>,
    target_focus: gpui::FocusHandle,
}

impl Render for ContextHarness {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let selected_new = self.selected.clone();
        let selected_png = self.selected.clone();
        let selected_delete = self.selected.clone();
        let open = self.open.clone();
        let background_clicks = self.background_clicks.clone();
        let target_key_received = self.target_key_received.clone();
        let target_focus = self.target_focus.clone();

        let entries = vec![
            DropdownMenuItem::new("New file")
                .shortcut("⌘N")
                .on_select(move |_, _| selected_new.set(1))
                .into(),
            DropdownMenuEntry::separator(),
            DropdownMenuItem::new("Export PDF").disabled(true).into(),
            DropdownMenuItem::new("Export")
                .submenu([
                    DropdownMenuItem::new("PNG")
                        .on_select(move |_, _| selected_png.set(2))
                        .into(),
                    DropdownMenuEntry::item("SVG"),
                ])
                .into(),
            DropdownMenuItem::new("Delete page")
                .destructive(true)
                .on_select(move |_, _| selected_delete.set(3))
                .into(),
        ];

        div()
            .size_full()
            .p(px(80.))
            .track_focus(&target_focus)
            .on_key_down(move |event, _, _| {
                if event.keystroke.key.as_str() == "x" {
                    target_key_received.set(true);
                }
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(20.))
                    .child(
                        ContextMenu::new("test-context")
                            .entries(entries)
                            .on_open_change(move |is_open, _, _| open.set(is_open))
                            .child(
                                div()
                                    .id("context-target-visual")
                                    .debug_selector(|| "CONTEXT_TARGET".into())
                                    .w(px(240.))
                                    .h(px(120.)),
                            ),
                    )
                    .child(
                        div()
                            .debug_selector(|| "CONTEXT_SENTINEL".into())
                            .w(px(80.))
                            .h(px(12.)),
                    ),
            )
            .child(
                div()
                    .id("context-background")
                    .debug_selector(|| "CONTEXT_BACKGROUND".into())
                    .absolute()
                    .right(px(20.))
                    .bottom(px(20.))
                    .size(px(100.))
                    .on_click(move |_, _, _| {
                        background_clicks.set(background_clicks.get() + 1);
                    }),
            )
    }
}

struct ContextTest {
    cx: VisualTestContext,
    selected: Rc<Cell<usize>>,
    open: Rc<Cell<bool>>,
    background_clicks: Rc<Cell<usize>>,
    target_key_received: Rc<Cell<bool>>,
}

fn setup(cx: &mut TestAppContext) -> ContextTest {
    cx.update(init_theme);
    let selected = Rc::new(Cell::new(0));
    let open = Rc::new(Cell::new(false));
    let background_clicks = Rc::new(Cell::new(0));
    let target_key_received = Rc::new(Cell::new(false));
    let window = cx.add_window({
        let selected = selected.clone();
        let open = open.clone();
        let background_clicks = background_clicks.clone();
        let target_key_received = target_key_received.clone();
        move |window, cx| {
            let target_focus = cx.focus_handle().tab_stop(true);
            target_focus.focus(window);
            ContextHarness {
                selected,
                open,
                background_clicks,
                target_key_received,
                target_focus,
            }
        }
    });
    cx.simulate_window_resize(window.into(), size(px(900.), px(700.)));
    cx.run_until_parked();

    ContextTest {
        cx: VisualTestContext::from_window(window.into(), cx),
        selected,
        open,
        background_clicks,
        target_key_received,
    }
}

fn open_at_target(cx: &mut VisualTestContext) -> gpui::Point<gpui::Pixels> {
    let target = cx.debug_bounds("test-context-target").expect("target");
    let click = target.origin + point(px(24.), px(32.));
    cx.simulate_mouse_down(click, MouseButton::Right, Modifiers::default());
    cx.simulate_mouse_up(click, MouseButton::Right, Modifiers::default());
    click
}

#[gpui::test]
fn menu_opens_at_the_pointer_without_reflow(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    let sentinel_before = test
        .cx
        .debug_bounds("CONTEXT_SENTINEL")
        .expect("sentinel bounds");

    let click = open_at_target(&mut test.cx);
    let panel = test
        .cx
        .debug_bounds("test-context-panel")
        .expect("menu panel");
    let sentinel_after = test
        .cx
        .debug_bounds("CONTEXT_SENTINEL")
        .expect("sentinel bounds");

    assert_eq!(panel.size.width, px(240.));
    assert_eq!(panel.origin, click);
    assert_eq!(sentinel_after, sentinel_before);
}

#[gpui::test]
fn pointer_action_closes_menu(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    open_at_target(&mut test.cx);
    let delete = test
        .cx
        .debug_bounds("test-context-item-4")
        .expect("delete item");

    test.cx
        .simulate_click(delete.center(), Modifiers::default());

    assert_eq!(test.selected.get(), 3);
    assert!(!test.open.get());
}

#[gpui::test]
fn arrows_skip_non_actions_and_enter_submenu(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    open_at_target(&mut test.cx);

    test.cx.simulate_keystrokes("down");
    test.cx.simulate_keystrokes("right");
    test.cx.simulate_keystrokes("enter");

    assert_eq!(test.selected.get(), 2);
    assert!(!test.open.get());
}

#[gpui::test]
fn outside_click_closes_and_reaches_background(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    open_at_target(&mut test.cx);
    let background = test
        .cx
        .debug_bounds("CONTEXT_BACKGROUND")
        .expect("background bounds");

    test.cx
        .simulate_click(background.center(), Modifiers::default());

    assert!(!test.open.get());
    assert_eq!(test.background_clicks.get(), 1);
}

#[gpui::test]
fn escape_closes_and_restores_surface_focus(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    open_at_target(&mut test.cx);

    test.cx.simulate_keystrokes("escape");
    test.cx.simulate_keystrokes("x");

    assert!(!test.open.get());
    assert!(
        test.target_key_received.get(),
        "focus should return to the surface that opened the menu"
    );
}
