use std::{cell::Cell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Context, KeyDownEvent, Modifiers, Render, TestAppContext,
    VisualTestContext, Window,
};
use glassy_ui::{init_theme, DropdownMenu, DropdownMenuEntry, DropdownMenuItem};

struct DropdownHarness {
    selected: Rc<Cell<usize>>,
    background_clicks: Rc<Cell<usize>>,
    trigger_key_received: Rc<Cell<bool>>,
}

impl Render for DropdownHarness {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let selected_new = self.selected.clone();
        let selected_png = self.selected.clone();
        let selected_delete = self.selected.clone();
        let background_clicks = self.background_clicks.clone();
        let trigger_key_received = self.trigger_key_received.clone();

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
            .on_key_down(move |event: &KeyDownEvent, _, _| {
                if event.keystroke.key.as_str() == "x" {
                    trigger_key_received.set(true);
                }
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(20.))
                    .child(
                        DropdownMenu::new("test-menu")
                            .trigger_label("Open File menu")
                            .trigger(
                                div()
                                    .debug_selector(|| "DROPDOWN_TRIGGER_VISUAL".into())
                                    .w(px(80.))
                                    .h(px(36.)),
                            )
                            .entries(entries),
                    )
                    .child(
                        div()
                            .debug_selector(|| "DROPDOWN_SENTINEL".into())
                            .w(px(80.))
                            .h(px(12.)),
                    ),
            )
            .child(
                div()
                    .id("dropdown-background")
                    .debug_selector(|| "DROPDOWN_BACKGROUND".into())
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

struct DropdownTestContext {
    cx: VisualTestContext,
    selected: Rc<Cell<usize>>,
    background_clicks: Rc<Cell<usize>>,
    trigger_key_received: Rc<Cell<bool>>,
}

fn setup(cx: &mut TestAppContext) -> DropdownTestContext {
    cx.update(init_theme);
    let selected = Rc::new(Cell::new(0));
    let background_clicks = Rc::new(Cell::new(0));
    let trigger_key_received = Rc::new(Cell::new(false));
    let window = cx.open_window(size(px(900.), px(700.)), {
        let selected = selected.clone();
        let background_clicks = background_clicks.clone();
        let trigger_key_received = trigger_key_received.clone();
        move |_, _| DropdownHarness {
            selected,
            background_clicks,
            trigger_key_received,
        }
    });
    cx.run_until_parked();

    DropdownTestContext {
        cx: VisualTestContext::from_window(window.into(), cx),
        selected,
        background_clicks,
        trigger_key_received,
    }
}

fn click_trigger(cx: &mut VisualTestContext) {
    let trigger = cx
        .debug_bounds("test-menu-trigger")
        .expect("dropdown trigger");
    cx.simulate_click(trigger.center(), Modifiers::default());
}

#[gpui::test]
fn menu_matches_paper_geometry_without_reflow(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    let sentinel_before = test
        .cx
        .debug_bounds("DROPDOWN_SENTINEL")
        .expect("sentinel bounds");

    click_trigger(&mut test.cx);

    let trigger = test
        .cx
        .debug_bounds("test-menu-trigger")
        .expect("trigger bounds");
    let panel = test.cx.debug_bounds("test-menu-panel").expect("menu panel");
    let sentinel_after = test
        .cx
        .debug_bounds("DROPDOWN_SENTINEL")
        .expect("sentinel bounds");

    assert_eq!(panel.size.width, px(240.));
    assert_eq!(panel.top() - trigger.bottom(), px(6.));
    assert_eq!(panel.left(), trigger.left());
    assert_eq!(sentinel_after, sentinel_before);
}

#[gpui::test]
fn pointer_action_closes_menu(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    click_trigger(&mut test.cx);
    let delete = test
        .cx
        .debug_bounds("test-menu-item-4")
        .expect("delete item");

    test.cx
        .simulate_click(delete.center(), Modifiers::default());

    assert_eq!(test.selected.get(), 3);
    assert!(test.cx.debug_bounds("test-menu-panel").is_none());
}

#[gpui::test]
fn disabled_item_does_not_activate_or_close(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    click_trigger(&mut test.cx);
    let disabled = test
        .cx
        .debug_bounds("test-menu-item-2")
        .expect("disabled item");

    test.cx
        .simulate_click(disabled.center(), Modifiers::default());

    assert_eq!(test.selected.get(), 0);
    assert!(test.cx.debug_bounds("test-menu-panel").is_some());
}

#[gpui::test]
fn arrows_skip_non_actions_and_enter_submenu(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    click_trigger(&mut test.cx);

    test.cx.simulate_keystrokes("down right enter");

    assert_eq!(test.selected.get(), 2);
    assert!(test.cx.debug_bounds("test-menu-panel").is_none());
}

#[gpui::test]
fn pointer_can_activate_nested_item(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    click_trigger(&mut test.cx);
    let export = test
        .cx
        .debug_bounds("test-menu-item-3")
        .expect("export submenu item");
    test.cx
        .simulate_click(export.center(), Modifiers::default());
    let png = test
        .cx
        .debug_bounds("test-menu-submenu-3-item-0")
        .expect("nested PNG item");

    test.cx.simulate_click(png.center(), Modifiers::default());

    assert_eq!(test.selected.get(), 2);
    assert!(test.cx.debug_bounds("test-menu-panel").is_none());
}

#[gpui::test]
fn outside_click_closes_and_reaches_background(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    click_trigger(&mut test.cx);
    let background = test
        .cx
        .debug_bounds("DROPDOWN_BACKGROUND")
        .expect("background bounds");

    test.cx
        .simulate_click(background.center(), Modifiers::default());

    assert!(test.cx.debug_bounds("test-menu-panel").is_none());
    assert_eq!(test.background_clicks.get(), 1);
}

#[gpui::test]
fn outside_click_closes_open_submenu(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    click_trigger(&mut test.cx);
    let export = test
        .cx
        .debug_bounds("test-menu-item-3")
        .expect("export submenu item");
    test.cx
        .simulate_click(export.center(), Modifiers::default());
    assert!(test.cx.debug_bounds("test-menu-submenu-3-panel").is_some());
    let background = test
        .cx
        .debug_bounds("DROPDOWN_BACKGROUND")
        .expect("background bounds");

    test.cx
        .simulate_click(background.center(), Modifiers::default());

    assert!(test.cx.debug_bounds("test-menu-panel").is_none());
    assert_eq!(test.background_clicks.get(), 1);
}

#[gpui::test]
fn escape_closes_and_restores_trigger_focus(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    click_trigger(&mut test.cx);

    test.cx.simulate_keystrokes("escape x");

    assert!(test.cx.debug_bounds("test-menu-panel").is_none());
    assert!(
        test.trigger_key_received.get(),
        "focus should return to the dropdown trigger"
    );
}
