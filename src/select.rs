use std::rc::Rc;

use crate::motion::{Motion, StyledSlot};
use crate::theme::{rgb, ActiveTheme, Theme, ThemeKind};
use gpui::{
    anchored, deferred, div, point, prelude::*, px, App, BoxShadow, ClickEvent, FocusHandle,
    FontWeight, IntoElement, KeyDownEvent, RenderOnce, Role, SharedString, StyleRefinement, Styled,
    Window,
};

use crate::button::ButtonVariant;
use crate::chrome::{button_chrome, field_chrome, FieldState};
use crate::icon::{Icon, IconName};

type SelectClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;
type SelectPickHandler = Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>;
type SelectOpenHandler = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

struct SelectState {
    focus_handle: FocusHandle,
    open: bool,
    value: Option<SharedString>,
    last_value_prop: Option<SharedString>,
    highlighted: Option<usize>,
}

fn initial_highlight(items: &[SelectItem], value: Option<&SharedString>) -> Option<usize> {
    value
        .and_then(|value| {
            items
                .iter()
                .position(|item| !item.disabled && item.value.as_ref() == value.as_ref())
        })
        .or_else(|| items.iter().position(|item| !item.disabled))
}

fn next_enabled(items: &[SelectItem], current: Option<usize>, forward: bool) -> Option<usize> {
    if items.is_empty() {
        return None;
    }

    match current {
        Some(start) => (1..=items.len())
            .map(|offset| {
                if forward {
                    (start + offset) % items.len()
                } else {
                    (start + items.len() - (offset % items.len())) % items.len()
                }
            })
            .find(|index| !items[*index].disabled),
        None if forward => items.iter().position(|item| !item.disabled),
        None => items.iter().rposition(|item| !item.disabled),
    }
}

fn select_menu_bg(theme: Theme) -> gpui::Hsla {
    match theme.kind {
        ThemeKind::Light => rgb(0xF4F5F7),
        ThemeKind::Dark => rgb(0x1C1D20),
    }
}

/// One row in the open list.
#[derive(Clone, Debug)]
pub struct SelectItem {
    pub value: SharedString,
    pub disabled: bool,
}

impl SelectItem {
    pub fn new(value: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Closed field is Input chrome plus a chevron. Open list is secondary glass.
#[derive(IntoElement)]
pub struct Select {
    id: SharedString,
    placeholder: SharedString,
    value: Option<SharedString>,
    items: Vec<SelectItem>,
    disabled: bool,
    focused: bool,
    controlled_open: Option<bool>,
    default_open: bool,
    style: StyleRefinement,
    on_click: Option<SelectClickHandler>,
    on_select: Option<SelectPickHandler>,
    on_open_change: Option<SelectOpenHandler>,
}

impl Select {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            placeholder: SharedString::from("Choose format"),
            value: None,
            items: Vec::new(),
            disabled: false,
            focused: false,
            controlled_open: None,
            default_open: false,
            style: StyleRefinement::default(),
            on_click: None,
            on_select: None,
            on_open_change: None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = SelectItem>) -> Self {
        self.items = items.into_iter().collect();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Keep the list open or closed from parent state.
    pub fn open(mut self, open: bool) -> Self {
        self.controlled_open = Some(open);
        self
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    pub fn on_click(
        mut self,
        listener: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(listener));
        self
    }

    pub fn on_select(
        mut self,
        listener: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(listener));
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

impl Styled for Select {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

fn set_open(
    state: &gpui::Entity<SelectState>,
    open: bool,
    items: &[SelectItem],
    on_open_change: Option<&SelectOpenHandler>,
    window: &mut Window,
    cx: &mut App,
) {
    if state.read(cx).open == open {
        return;
    }
    state.update(cx, |select, cx| {
        select.open = open;
        select.highlighted = open
            .then(|| initial_highlight(items, select.value.as_ref()))
            .flatten();
        if !open {
            select.highlighted = None;
        }
        cx.notify();
    });
    if let Some(on_open_change) = on_open_change {
        on_open_change(open, window, cx);
    }
}

impl RenderOnce for Select {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let initial_open = self.controlled_open.unwrap_or(self.default_open);
        let initial_value = self.value.clone();
        let initial_items = self.items.clone();
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, cx| SelectState {
            focus_handle: cx.focus_handle().tab_stop(true),
            open: initial_open,
            highlighted: initial_open
                .then(|| initial_highlight(&initial_items, initial_value.as_ref()))
                .flatten(),
            value: initial_value.clone(),
            last_value_prop: initial_value,
        });

        if let Some(controlled_open) = self.controlled_open {
            if state.read(cx).open != controlled_open {
                state.update(cx, |select, _| {
                    select.open = controlled_open;
                    select.highlighted = controlled_open
                        .then(|| initial_highlight(&self.items, select.value.as_ref()))
                        .flatten();
                });
            }
        }
        if state.read(cx).last_value_prop != self.value {
            state.update(cx, |select, _| {
                select.value = self.value.clone();
                select.last_value_prop = self.value.clone();
            });
        }

        let open = state.read(cx).open;
        let value = state.read(cx).value.clone();
        let highlighted = state.read(cx).highlighted;
        let focus_handle = state.read(cx).focus_handle.clone();
        let theme = cx.theme();
        let field_state = if self.disabled {
            FieldState::Disabled
        } else if self.focused || open || focus_handle.is_focused(window) {
            FieldState::Focus
        } else {
            FieldState::Rest
        };
        let chrome = field_chrome(theme, field_state);
        let list_chrome = button_chrome(theme, ButtonVariant::Secondary);
        let list_bg = select_menu_bg(theme);
        let ghost = button_chrome(theme, ButtonVariant::Ghost);

        let mut shadows = vec![BoxShadow::new(px(0.), px(1.), chrome.inset).inset()];
        if chrome.shadow_blur > 0.0 {
            shadows.push(
                BoxShadow::new(px(0.), px(chrome.shadow_y), chrome.shadow)
                    .blur_radius(px(chrome.shadow_blur)),
            );
        }
        if let Some(ring) = chrome.ring {
            shadows.push(BoxShadow::new(px(0.), px(0.), ring).spread_radius(px(3.)));
        }

        let has_value = value.is_some();
        let label = value.clone().unwrap_or_else(|| self.placeholder.clone());
        let label_color = if self.disabled || !has_value {
            chrome.placeholder
        } else {
            chrome.fg
        };

        let interactive = !self.disabled;
        let on_click = self.on_click.clone();
        let on_open_change = self.on_open_change.clone();
        let trigger_state = state.clone();
        let trigger_items = self.items.clone();
        let trigger_focus = focus_handle.clone();
        let trigger_open_change = on_open_change.clone();
        let next_open = !open;
        let trigger_debug_selector = format!("{}-trigger", self.id);
        let mut trigger = div()
            .id(self.id.clone())
            .debug_selector(move || trigger_debug_selector)
            .role(Role::ComboBox)
            .aria_expanded(open)
            .flex()
            .items_center()
            .gap(px(8.))
            .w(px(280.))
            .h(px(36.))
            .flex_shrink_0()
            .px(px(14.))
            .rounded(px(6.))
            .border_1()
            .border_color(chrome.border)
            .bg(chrome.bg)
            .shadow(shadows)
            .when(interactive, |el| el.cursor_pointer())
            .when(!interactive, |el| el.cursor_default())
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.))
                    .font_family(theme.font_family)
                    .font_weight(FontWeight::NORMAL)
                    .text_size(px(14.))
                    .line_height(px(18.))
                    .text_color(label_color)
                    .child(label),
            )
            .child(
                Icon::new(IconName::ChevronDown)
                    .px(px(16.))
                    .color(theme.label),
            );

        if interactive {
            trigger = trigger.on_click(move |event, window, cx| {
                trigger_focus.focus(window, cx);
                set_open(
                    &trigger_state,
                    next_open,
                    &trigger_items,
                    trigger_open_change.as_ref(),
                    window,
                    cx,
                );
                if let Some(on_click) = &on_click {
                    on_click(event, window, cx);
                }
            });
        }

        let mut list_shadows = vec![BoxShadow::new(px(0.), px(1.), list_chrome.inset).inset()];
        if list_chrome.shadow_blur > 0.0 {
            list_shadows.push(
                BoxShadow::new(px(0.), px(list_chrome.shadow_y), list_chrome.shadow)
                    .blur_radius(px(list_chrome.shadow_blur)),
            );
        }

        let selected = value.clone();
        let on_select = self.on_select.clone();
        let keyboard_items = self.items.clone();
        let keyboard_on_select = self.on_select.clone();
        let ghost_bg = ghost.bg;
        let ghost_hover = ghost.hover_bg;
        let select_id = self.id.clone();
        let popup_debug_selector = format!("{select_id}-popup");
        let dismiss_state = state.clone();
        let dismiss_items = self.items.clone();
        let dismiss_open_change = on_open_change.clone();
        let list = div()
            .id(SharedString::from(format!("{select_id}-popup")))
            .debug_selector(move || popup_debug_selector)
            .flex()
            .flex_col()
            .w(px(280.))
            .flex_shrink_0()
            .p(px(4.))
            .rounded(px(6.))
            .border_1()
            .border_color(list_chrome.border)
            .bg(list_bg)
            .shadow(list_shadows)
            .occlude()
            .on_mouse_down_out(move |_, window, cx| {
                set_open(
                    &dismiss_state,
                    false,
                    &dismiss_items,
                    dismiss_open_change.as_ref(),
                    window,
                    cx,
                );
            })
            .children(self.items.into_iter().enumerate().map(|(index, item)| {
                let is_selected = selected
                    .as_ref()
                    .is_some_and(|value| value.as_ref() == item.value.as_ref());
                let is_highlighted = highlighted == Some(index);
                let enabled = !item.disabled;
                let text_color = if item.disabled {
                    theme.label
                } else {
                    theme.ink
                };
                let item_value = item.value.clone();
                let on_select = on_select.clone();
                let row_open_change = on_open_change.clone();
                let row_state = state.clone();
                let row_id = SharedString::from(format!("{select_id}-{index}"));
                let selected_motion_id = format!("{select_id}-{index}-selected");

                let mut row = div()
                    .id(row_id)
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .h(px(32.))
                    .flex_shrink_0()
                    .px(px(8.))
                    .rounded(px(6.))
                    .when(is_selected || is_highlighted, |el| el.bg(ghost_bg))
                    .when(enabled, |el| {
                        el.cursor_pointer().hover(move |s| s.bg(ghost_hover))
                    })
                    .when(!enabled, |el| el.cursor_default())
                    .child(div().size(px(16.)).flex_shrink_0().when(is_selected, |el| {
                        el.child(
                            Motion::new()
                                .id(selected_motion_id)
                                .selection_in()
                                .child(Icon::new(IconName::Check).px(px(16.)).color(theme.ink)),
                        )
                    }))
                    .child(
                        div()
                            .font_family(theme.font_family)
                            .font_weight(FontWeight::NORMAL)
                            .text_size(px(14.))
                            .line_height(px(18.))
                            .text_color(text_color)
                            .child(item.value.clone()),
                    );

                if enabled {
                    row = row.on_click(move |_, window, cx| {
                        row_state.update(cx, |select, cx| {
                            select.value = Some(item_value.clone());
                            cx.notify();
                        });
                        set_open(&row_state, false, &[], row_open_change.as_ref(), window, cx);
                        if let Some(on_select) = &on_select {
                            on_select(&item_value, window, cx);
                        }
                    });
                }

                row
            }));

        let list = Motion::new()
            .id(format!("{select_id}-surface"))
            .surface_in()
            .child(list);
        let popup = deferred(anchored().offset(point(px(0.), px(8.))).child(list)).with_priority(1);

        let keyboard_state = state.clone();
        let keyboard_open_change = on_open_change.clone();

        div()
            .relative()
            .w(px(280.))
            .h(px(36.))
            .flex_shrink_0()
            .refine_style(&self.style)
            .track_focus(&focus_handle)
            .tab_stop(interactive)
            .on_key_down(move |event: &KeyDownEvent, window, cx| {
                if !interactive || event.keystroke.modifiers.modified() {
                    return;
                }

                let key = event.keystroke.key.as_str();
                let (was_open, current) = {
                    let snapshot = keyboard_state.read(cx);
                    (snapshot.open, snapshot.highlighted)
                };

                match key {
                    "down" | "up" => {
                        let next = next_enabled(&keyboard_items, current, key == "down");
                        if !was_open {
                            set_open(
                                &keyboard_state,
                                true,
                                &keyboard_items,
                                keyboard_open_change.as_ref(),
                                window,
                                cx,
                            );
                        }
                        keyboard_state.update(cx, |select, cx| {
                            select.highlighted = next;
                            cx.notify();
                        });
                        cx.stop_propagation();
                    }
                    "enter" | "space" if !was_open => {
                        set_open(
                            &keyboard_state,
                            true,
                            &keyboard_items,
                            keyboard_open_change.as_ref(),
                            window,
                            cx,
                        );
                        cx.stop_propagation();
                    }
                    "enter" | "space" => {
                        let picked = current
                            .and_then(|index| keyboard_items.get(index))
                            .filter(|item| !item.disabled)
                            .map(|item| item.value.clone());
                        if let Some(picked) = picked {
                            keyboard_state.update(cx, |select, cx| {
                                select.value = Some(picked.clone());
                                cx.notify();
                            });
                            set_open(
                                &keyboard_state,
                                false,
                                &keyboard_items,
                                keyboard_open_change.as_ref(),
                                window,
                                cx,
                            );
                            if let Some(on_select) = &keyboard_on_select {
                                on_select(&picked, window, cx);
                            }
                        }
                        cx.stop_propagation();
                    }
                    "escape" if was_open => {
                        set_open(
                            &keyboard_state,
                            false,
                            &keyboard_items,
                            keyboard_open_change.as_ref(),
                            window,
                            cx,
                        );
                        cx.stop_propagation();
                    }
                    _ => {}
                }
            })
            .child(trigger)
            .when(open && !self.disabled, |el| el.child(popup))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_backdrop_is_opaque_and_matches_the_secondary_composite() {
        let light = select_menu_bg(Theme::light());
        let dark = select_menu_bg(Theme::dark());

        assert_eq!(light, rgb(0xF4F5F7));
        assert_eq!(dark, rgb(0x1C1D20));
        assert_eq!(light.a, 1.0);
        assert_eq!(dark.a, 1.0);
    }
}
