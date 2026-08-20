use std::rc::Rc;

use crate::motion::{Motion, StyledSlot};
use crate::theme::ActiveTheme;
use gpui::{
    div, prelude::*, px, App, ClickEvent, FocusHandle, FontWeight, IntoElement, KeyDownEvent,
    RenderOnce, SharedString, StyleRefinement, Styled, Window,
};

use crate::compat::{AccessibilityExt, Role};

use crate::button::ButtonVariant;
use crate::chrome::{box_shadow, button_chrome, focus_ring};

type RadioClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;
type RadioChangeHandler = Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>;

struct RadioGroupState {
    selected: Option<SharedString>,
    last_selected_prop: Option<SharedString>,
}

struct RadioFocusState {
    focus_handle: FocusHandle,
}

/// 16×16 circle matching Paper `Glassy UI` → Radios.
///
/// Radios that share [`Radio::group`] keep one selection. Without a listener
/// the group owns that selection; with [`Radio::on_change`] / [`Radio::on_click`],
/// [`Radio::selected`] is the source of truth each render.
#[derive(IntoElement)]
pub struct Radio {
    id: SharedString,
    group: Option<SharedString>,
    selected: bool,
    disabled: bool,
    label: Option<SharedString>,
    style: StyleRefinement,
    on_click: Option<RadioClickHandler>,
    on_change: Option<RadioChangeHandler>,
}

impl Radio {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            group: None,
            selected: false,
            disabled: false,
            label: None,
            style: StyleRefinement::default(),
            on_click: None,
            on_change: None,
        }
    }

    /// Radios that share a group keep one selection.
    pub fn group(mut self, group: impl Into<SharedString>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn on_click(
        mut self,
        listener: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(listener));
        self
    }

    /// Selected radio id after a pointer or keyboard activation.
    pub fn on_change(
        mut self,
        listener: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(listener));
        self
    }
}

impl Styled for Radio {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Radio {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let radio_id = self.id.clone();
        let initially_selected = self.selected;
        let seed = radio_id.clone();
        let seed_prop = radio_id.clone();
        let state_key = self.group.clone().unwrap_or_else(|| self.id.clone());
        let state = window.use_keyed_state(state_key, cx, move |_, _| RadioGroupState {
            selected: if initially_selected { Some(seed) } else { None },
            last_selected_prop: if initially_selected {
                Some(seed_prop)
            } else {
                None
            },
        });
        let focus = window.use_keyed_state(
            SharedString::from(format!("{}-focus", self.id)),
            cx,
            |_, cx| RadioFocusState {
                focus_handle: cx.focus_handle(),
            },
        );
        let controlled = self.on_change.is_some() || self.on_click.is_some();
        if !controlled && self.selected {
            let already = state
                .read(cx)
                .last_selected_prop
                .as_ref()
                .is_some_and(|id| id.as_ref() == self.id.as_ref());
            if !already {
                state.update(cx, |group, _| {
                    group.selected = Some(self.id.clone());
                    group.last_selected_prop = Some(self.id.clone());
                });
            }
        }

        let selected = if controlled {
            self.selected
        } else {
            state
                .read(cx)
                .selected
                .as_ref()
                .is_some_and(|id| id.as_ref() == self.id.as_ref())
        };
        let theme = cx.theme();
        let variant = if self.disabled {
            ButtonVariant::Ghost
        } else if selected {
            ButtonVariant::Primary
        } else {
            ButtonVariant::Outline
        };
        let chrome = button_chrome(theme, variant);
        let dot = if self.disabled && selected {
            theme.muted_fg()
        } else {
            theme.on_solid
        };

        let mut shadows = vec![box_shadow(0., 1., chrome.inset, 0., 0.)];
        if chrome.shadow_blur > 0.0 {
            shadows.push(box_shadow(
                0.,
                chrome.shadow_y,
                chrome.shadow,
                chrome.shadow_blur,
                0.,
            ));
        }

        let interactive = !self.disabled;
        let focus_handle = focus.read(cx).focus_handle.clone().tab_stop(interactive);
        let focused = focus_handle.is_focused(window);
        if focused {
            shadows.push(focus_ring(theme));
        }

        let mark = div()
            .flex()
            .items_center()
            .justify_center()
            .size(px(16.))
            .flex_shrink_0()
            .rounded(px(8.))
            .border_1()
            .border_color(chrome.border)
            .bg(chrome.bg)
            .shadow(shadows)
            .when(selected, |el| {
                el.child(
                    Motion::new()
                        .id(format!("{}-dot", self.id))
                        .selection_in()
                        .child(div().size(px(6.)).flex_shrink_0().rounded(px(3.)).bg(dot)),
                )
            });

        let label_color = if self.disabled {
            theme.muted_fg()
        } else {
            theme.ink
        };
        let aria_label = self.label.clone();
        let debug_selector = format!(
            "{}-{}",
            self.id,
            if selected { "selected" } else { "unselected" }
        );

        let el = div()
            .id(self.id.clone())
            .debug_selector(move || debug_selector.clone())
            .role(Role::RadioButton)
            .aria_selected(selected)
            .when_some(aria_label, |el, label| el.aria_label(label))
            .track_focus(&focus_handle)
            .tab_stop(interactive)
            .flex()
            .items_center()
            .gap(px(8.))
            .refine_style(&self.style)
            .when(interactive, |el| el.cursor_pointer())
            .when(!interactive, |el| el.cursor_default())
            .child(mark)
            .when_some(self.label, |el, label| {
                el.child(
                    div()
                        .font_family(theme.font_family)
                        .font_weight(FontWeight::NORMAL)
                        .text_size(px(14.))
                        .line_height(px(18.))
                        .text_color(label_color)
                        .child(label),
                )
            });

        if interactive {
            let on_click = self.on_click;
            let on_change = self.on_change;
            let activate = Rc::new(
                move |event: &ClickEvent, window: &mut Window, cx: &mut App| {
                    if !controlled {
                        state.update(cx, |group, cx| {
                            group.selected = Some(radio_id.clone());
                            cx.notify();
                        });
                    }
                    if let Some(on_change) = &on_change {
                        on_change(&radio_id, window, cx);
                    }
                    if let Some(on_click) = &on_click {
                        on_click(event, window, cx);
                    }
                },
            );
            let keyboard = activate.clone();
            let click_focus = focus_handle.clone();
            el.on_key_down(move |event: &KeyDownEvent, window, cx| {
                if event.keystroke.modifiers.modified() {
                    return;
                }
                if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                    keyboard(&ClickEvent::default(), window, cx);
                    cx.stop_propagation();
                }
            })
            .on_click(move |event, window, cx| {
                click_focus.focus(window);
                activate(event, window, cx);
            })
        } else {
            el
        }
    }
}
