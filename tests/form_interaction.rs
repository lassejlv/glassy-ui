use std::{cell::Cell, cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Context, Modifiers, Render, SharedString, TestAppContext,
    VisualTestContext, Window,
};
use glassy_ui::{init as init_ui, init_theme, CheckState, Checkbox, Input, Radio, Switch};

struct FormHarness {
    check: CheckState,
    switch_on: bool,
    radio: SharedString,
    field: SharedString,
    last_check: Rc<Cell<Option<CheckState>>>,
    last_switch: Rc<Cell<Option<bool>>>,
    last_radio: Rc<RefCell<Option<SharedString>>>,
    last_field: Rc<RefCell<Option<SharedString>>>,
}

impl Render for FormHarness {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let check_entity = cx.entity();
        let switch_entity = cx.entity();
        let radio_png = cx.entity();
        let radio_svg = cx.entity();
        let field_entity = cx.entity();
        let last_check_ok = self.last_check.clone();
        let last_check_refused = self.last_check.clone();
        let last_switch = self.last_switch.clone();
        let last_radio_png = self.last_radio.clone();
        let last_radio_svg = self.last_radio.clone();
        let last_field = self.last_field.clone();

        div()
            .flex()
            .flex_col()
            .gap(px(16.))
            .p(px(40.))
            .child(Checkbox::new("gallery-check").label("Show grid"))
            .child(
                Checkbox::new("controlled-check")
                    .label("Lock")
                    .state(self.check)
                    .on_change(move |state, _, cx| {
                        last_check_ok.set(Some(state));
                        check_entity.update(cx, |harness, cx| {
                            harness.check = state;
                            cx.notify();
                        });
                    }),
            )
            .child(
                Checkbox::new("refused-check")
                    .label("Stay off")
                    .checked(false)
                    .on_change(move |state, _, _| {
                        last_check_refused.set(Some(state));
                    }),
            )
            .child(Switch::new("gallery-switch").label("Snap").on(true))
            .child(
                Switch::new("controlled-switch")
                    .label("Auto-save")
                    .on(self.switch_on)
                    .on_change(move |on, _, cx| {
                        last_switch.set(Some(on));
                        switch_entity.update(cx, |harness, cx| {
                            harness.switch_on = on;
                            cx.notify();
                        });
                    }),
            )
            .child(
                Radio::new("png")
                    .group("export")
                    .label("PNG")
                    .selected(self.radio.as_ref() == "png")
                    .on_change(move |id, _, cx| {
                        *last_radio_png.borrow_mut() = Some(id.clone());
                        radio_png.update(cx, |harness, cx| {
                            harness.radio = id.clone();
                            cx.notify();
                        });
                    }),
            )
            .child(
                Radio::new("svg")
                    .group("export")
                    .label("SVG")
                    .selected(self.radio.as_ref() == "svg")
                    .on_change(move |id, _, cx| {
                        *last_radio_svg.borrow_mut() = Some(id.clone());
                        radio_svg.update(cx, |harness, cx| {
                            harness.radio = id.clone();
                            cx.notify();
                        });
                    }),
            )
            .child(
                Input::new("name")
                    .placeholder("Project name")
                    .value(self.field.clone())
                    .on_change(move |value, _, cx| {
                        *last_field.borrow_mut() = Some(value.clone());
                        field_entity.update(cx, |harness, cx| {
                            harness.field = value;
                            cx.notify();
                        });
                    }),
            )
    }
}

struct FormTest {
    cx: VisualTestContext,
    last_check: Rc<Cell<Option<CheckState>>>,
    last_switch: Rc<Cell<Option<bool>>>,
    last_radio: Rc<RefCell<Option<SharedString>>>,
    last_field: Rc<RefCell<Option<SharedString>>>,
}

fn setup(cx: &mut TestAppContext) -> FormTest {
    cx.update(|cx| {
        init_theme(cx);
        init_ui(cx);
    });
    let last_check = Rc::new(Cell::new(None));
    let last_switch = Rc::new(Cell::new(None));
    let last_radio = Rc::new(RefCell::new(None));
    let last_field = Rc::new(RefCell::new(None));
    let window = cx.open_window(size(px(800.), px(700.)), {
        let last_check = last_check.clone();
        let last_switch = last_switch.clone();
        let last_radio = last_radio.clone();
        let last_field = last_field.clone();
        move |_, _| FormHarness {
            check: CheckState::Off,
            switch_on: false,
            radio: "png".into(),
            field: SharedString::default(),
            last_check,
            last_switch,
            last_radio,
            last_field,
        }
    });
    cx.run_until_parked();
    FormTest {
        cx: VisualTestContext::from_window(window.into(), cx),
        last_check,
        last_switch,
        last_radio,
        last_field,
    }
}

fn click_selector(cx: &mut VisualTestContext, selector: &'static str) {
    let bounds = cx.debug_bounds(selector).expect(selector);
    cx.simulate_click(bounds.center(), Modifiers::default());
}

#[gpui::test]
fn uncontrolled_checkbox_toggles(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    assert!(test.cx.debug_bounds("gallery-check-off").is_some());
    click_selector(&mut test.cx, "gallery-check-off");
    assert!(test.cx.debug_bounds("gallery-check-on").is_some());
}

#[gpui::test]
fn controlled_checkbox_follows_parent(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    click_selector(&mut test.cx, "controlled-check-off");
    assert_eq!(test.last_check.get(), Some(CheckState::On));
    assert!(test.cx.debug_bounds("controlled-check-on").is_some());
}

#[gpui::test]
fn controlled_checkbox_can_refuse_the_change(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    click_selector(&mut test.cx, "refused-check-off");
    assert_eq!(test.last_check.get(), Some(CheckState::On));
    assert!(test.cx.debug_bounds("refused-check-off").is_some());
    assert!(test.cx.debug_bounds("refused-check-on").is_none());
}

#[gpui::test]
fn checkbox_space_toggles(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    click_selector(&mut test.cx, "gallery-check-off");
    assert!(test.cx.debug_bounds("gallery-check-on").is_some());
    test.cx.simulate_keystrokes("space");
    assert!(test.cx.debug_bounds("gallery-check-off").is_some());
}

#[gpui::test]
fn uncontrolled_switch_starts_on_and_toggles(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    assert!(test.cx.debug_bounds("gallery-switch-on").is_some());
    click_selector(&mut test.cx, "gallery-switch-on");
    assert!(test.cx.debug_bounds("gallery-switch-off").is_some());
}

#[gpui::test]
fn controlled_switch_follows_parent(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    click_selector(&mut test.cx, "controlled-switch-off");
    assert_eq!(test.last_switch.get(), Some(true));
    assert!(test.cx.debug_bounds("controlled-switch-on").is_some());
}

#[gpui::test]
fn radio_group_selects_one(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    assert!(test.cx.debug_bounds("png-selected").is_some());
    click_selector(&mut test.cx, "svg-unselected");
    assert_eq!(
        test.last_radio
            .borrow()
            .as_ref()
            .map(|value| value.as_ref()),
        Some("svg")
    );
    assert!(test.cx.debug_bounds("svg-selected").is_some());
    assert!(test.cx.debug_bounds("png-unselected").is_some());
}

#[gpui::test]
fn input_on_change_receives_typed_text(cx: &mut TestAppContext) {
    let mut test = setup(cx);
    let bounds = test.cx.debug_bounds("name").expect("input");
    test.cx
        .simulate_click(bounds.center(), Modifiers::default());
    test.cx.simulate_input("gpui");
    assert_eq!(
        test.last_field
            .borrow()
            .as_ref()
            .map(|value| value.as_ref()),
        Some("gpui")
    );
}
