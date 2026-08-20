use std::{cell::Cell, cell::RefCell, rc::Rc};

use glassy_ui::{init as init_ui, init_theme, Command, CommandGroup, CommandItem, CommandSize};
use gpui::{
    div, prelude::*, px, size, Context, FocusHandle, Modifiers, Render, SharedString,
    TestAppContext, VisualTestContext, Window,
};

struct CommandHarness {
    focus_handle: FocusHandle,
    selected: Rc<RefCell<Option<SharedString>>>,
    query: Rc<RefCell<SharedString>>,
    dismissals: Rc<Cell<usize>>,
}

impl CommandHarness {
    fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        selected: Rc<RefCell<Option<SharedString>>>,
        query: Rc<RefCell<SharedString>>,
        dismissals: Rc<Cell<usize>>,
    ) -> Self {
        let focus_handle = cx.focus_handle().tab_stop(true);
        focus_handle.focus(window);
        Self {
            focus_handle,
            selected,
            query,
            dismissals,
        }
    }
}

impl Render for CommandHarness {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.selected.clone();
        let query = self.query.clone();
        let dismissals = self.dismissals.clone();

        div()
            .flex()
            .flex_col()
            .gap(px(24.))
            .p(px(40.))
            .child(
                Command::new("test-command")
                    .focus_handle(self.focus_handle.clone())
                    .groups([CommandGroup::new("Pages").items([
                        CommandItem::new("home", "Home"),
                        CommandItem::new("disabled", "Disabled").disabled(true),
                        CommandItem::new("buttons", "Buttons").keywords(["controls"]),
                    ])])
                    .on_query_change(move |value, _, _| {
                        *query.borrow_mut() = value;
                    })
                    .on_select(move |id, _, _| {
                        *selected.borrow_mut() = Some(id.clone());
                    })
                    .on_dismiss(move |_, _| {
                        dismissals.set(dismissals.get() + 1);
                    }),
            )
            .child(
                Command::new("loading-command")
                    .size(CommandSize::Compact)
                    .loading(true)
                    .loading_label("Searching pages")
                    .show_footer(false),
            )
            .child(
                Command::new("empty-command")
                    .size(CommandSize::Compact)
                    .default_query("xyzzy")
                    .empty_label("No pages match.")
                    .show_footer(false),
            )
    }
}

struct CommandTest {
    cx: VisualTestContext,
    selected: Rc<RefCell<Option<SharedString>>>,
    query: Rc<RefCell<SharedString>>,
    dismissals: Rc<Cell<usize>>,
}

fn setup(cx: &mut TestAppContext) -> CommandTest {
    cx.update(|cx| {
        init_theme(cx);
        init_ui(cx);
    });
    let selected = Rc::new(RefCell::new(None));
    let query = Rc::new(RefCell::new(SharedString::default()));
    let dismissals = Rc::new(Cell::new(0));
    let window = cx.add_window({
        let selected = selected.clone();
        let query = query.clone();
        let dismissals = dismissals.clone();
        move |window, cx| CommandHarness::new(window, cx, selected, query, dismissals)
    });
    cx.simulate_window_resize(window.into(), size(px(800.), px(800.)));
    cx.run_until_parked();
    CommandTest {
        cx: VisualTestContext::from_window(window.into(), cx),
        selected,
        query,
        dismissals,
    }
}

#[gpui::test]
fn command_matches_the_authored_width_and_search_height(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    let command = test.cx.debug_bounds("test-command").expect("command");
    let search = test.cx.debug_bounds("test-command-search").expect("search");

    assert_eq!(command.size.width, px(400.));
    assert_eq!(search.size.height, px(36.));
}

#[gpui::test]
fn typing_filters_rows_and_reports_the_query(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    test.cx.simulate_input("but");
    test.cx.simulate_keystrokes("enter");

    assert_eq!(test.query.borrow().as_ref(), "but");
    assert_eq!(
        test.selected.borrow().as_ref().map(AsRef::as_ref),
        Some("buttons")
    );
}

#[gpui::test]
fn arrows_skip_disabled_rows_and_enter_activates(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    test.cx.simulate_keystrokes("down");
    test.cx.simulate_keystrokes("enter");

    assert_eq!(
        test.selected.borrow().as_ref().map(AsRef::as_ref),
        Some("buttons")
    );
}

#[gpui::test]
fn pointer_activation_reports_the_stable_item_id(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    let row = test
        .cx
        .debug_bounds("test-command-item-buttons")
        .expect("buttons row");
    test.cx.simulate_click(row.center(), Modifiers::default());

    assert_eq!(
        test.selected.borrow().as_ref().map(AsRef::as_ref),
        Some("buttons")
    );
}

#[gpui::test]
fn escape_requests_dismissal(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    test.cx.simulate_keystrokes("escape");

    assert_eq!(test.dismissals.get(), 1);
}

#[gpui::test]
fn loading_and_empty_states_render_without_rows(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    assert!(test.cx.debug_bounds("COMMAND_LOADING").is_some());
    assert!(test.cx.debug_bounds("COMMAND_EMPTY").is_some());
    assert_eq!(
        test.cx
            .debug_bounds("loading-command")
            .expect("loading command")
            .size
            .width,
        px(280.)
    );
    assert_eq!(
        test.cx
            .debug_bounds("empty-command")
            .expect("empty command")
            .size
            .width,
        px(280.)
    );
}
