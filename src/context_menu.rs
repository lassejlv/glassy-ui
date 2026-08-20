use std::rc::Rc;

use gpui::{
    anchored, deferred, div, point, prelude::*, px, AnchoredPositionMode, AnyElement, App,
    IntoElement, KeyDownEvent, MouseButton, ParentElement, Pixels, Point, RenderOnce, SharedString,
    StyleRefinement, Styled, Window,
};

use crate::compat::StyleCompatExt;

use crate::dropdown_menu::{
    handle_menu_keydown, initial_highlight, render_panel, set_open, DropdownMenuEntry,
    DropdownMenuState, MenuPanelContext, OpenChangeHandler,
};
use crate::motion::StyledSlot;

/// Pointer-anchored menu. Same items as [`crate::DropdownMenu`], origin at the click.
#[derive(IntoElement)]
pub struct ContextMenu {
    id: SharedString,
    controlled_open: Option<bool>,
    default_open: bool,
    position: Option<Point<Pixels>>,
    entries: Vec<DropdownMenuEntry>,
    on_open_change: Option<OpenChangeHandler>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl ContextMenu {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            controlled_open: None,
            default_open: false,
            position: None,
            entries: Vec::new(),
            on_open_change: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.controlled_open = Some(open);
        self
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Local origin used when the menu is shown without a pointer event.
    pub fn position(mut self, origin: Point<Pixels>) -> Self {
        self.position = Some(origin);
        self
    }

    pub fn entries(mut self, entries: impl IntoIterator<Item = DropdownMenuEntry>) -> Self {
        self.entries = entries.into_iter().collect();
        self
    }

    pub fn on_open_change(
        mut self,
        listener: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(listener));
        self
    }
}

impl Styled for ContextMenu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ContextMenu {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ContextMenu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let initial_open = self.controlled_open.unwrap_or(self.default_open);
        let initial_entries = self.entries.clone();
        let initial_origin = self.position.unwrap_or_else(|| point(px(0.), px(0.)));
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, cx| DropdownMenuState {
            focus_handle: cx.focus_handle().tab_stop(true),
            open: initial_open,
            highlighted: initial_open
                .then(|| initial_highlight(&initial_entries))
                .flatten(),
            submenu: None,
            origin: initial_origin,
            origin_window: false,
            previous_focus: None,
        });

        if let Some(controlled_open) = self.controlled_open {
            if state.read(cx).open != controlled_open {
                state.update(cx, |menu, _| {
                    menu.open = controlled_open;
                    menu.highlighted = controlled_open
                        .then(|| initial_highlight(&self.entries))
                        .flatten();
                    menu.submenu = None;
                });
            }
        }
        if let Some(position) = self.position {
            if state.read(cx).origin != position || state.read(cx).origin_window {
                state.update(cx, |menu, _| {
                    menu.origin = position;
                    menu.origin_window = false;
                });
            }
        }

        let open = state.read(cx).open;
        let highlighted = state.read(cx).highlighted;
        let submenu = state.read(cx).submenu;
        let origin = state.read(cx).origin;
        let origin_window = state.read(cx).origin_window;
        let focus_handle = state.read(cx).focus_handle.clone();
        let restore_focus = state
            .read(cx)
            .previous_focus
            .clone()
            .unwrap_or_else(|| focus_handle.clone());

        let user_change = self.on_open_change.clone();
        let restore_state = state.clone();
        let on_open_change: OpenChangeHandler = Rc::new(move |open, window, cx| {
            if !open {
                let previous = restore_state.read(cx).previous_focus.clone();
                if let Some(previous) = previous {
                    previous.focus(window);
                }
            }
            if let Some(user_change) = &user_change {
                user_change(open, window, cx);
            }
        });

        let click_state = state.clone();
        let click_entries = self.entries.clone();
        let click_change = on_open_change.clone();
        let click_focus = focus_handle.clone();
        let keyboard_state = state.clone();
        let keyboard_entries = self.entries.clone();
        let keyboard_change = on_open_change.clone();
        let keyboard_restore = restore_focus.clone();

        let target = div()
            .id(SharedString::from(format!("{}-target", self.id)))
            .debug_selector({
                let selector = format!("{}-target", self.id);
                move || selector.clone()
            })
            .relative()
            .flex()
            .flex_none()
            .self_start()
            .refine_style(&self.style)
            .on_mouse_down(MouseButton::Right, move |event, window, cx| {
                let previous = window.focused(cx);
                click_state.update(cx, |menu, cx| {
                    menu.origin = event.position;
                    menu.origin_window = true;
                    menu.previous_focus = previous;
                    cx.notify();
                });
                set_open(
                    &click_state,
                    true,
                    &click_entries,
                    Some(&click_change),
                    window,
                    cx,
                );
                click_focus.focus(window);
                cx.stop_propagation();
            })
            .children(self.children);

        let panel_context = MenuPanelContext {
            state: state.clone(),
            root_entries: self.entries.clone(),
            focus_handle: focus_handle.clone(),
            on_open_change: Some(on_open_change.clone()),
        };
        let panel = render_panel(
            self.id.clone(),
            self.entries,
            highlighted,
            submenu.map(|submenu| submenu.parent),
            panel_context,
            submenu.is_none(),
            cx,
        );

        let menu = deferred(
            anchored()
                .anchor(gpui::Corner::TopLeft)
                .position(origin)
                .position_mode(if origin_window {
                    AnchoredPositionMode::Window
                } else {
                    AnchoredPositionMode::Local
                })
                .snap_to_window_with_margin(px(8.))
                .child(
                    div()
                        .track_focus(&focus_handle)
                        .on_key_down(move |event: &KeyDownEvent, window, cx| {
                            handle_menu_keydown(
                                event,
                                &keyboard_state,
                                &keyboard_entries,
                                Some(&keyboard_change),
                                &keyboard_restore,
                                window,
                                cx,
                            );
                        })
                        .child(panel),
                ),
        )
        .with_priority(2);

        target.when(open, |target| target.child(menu))
    }
}
