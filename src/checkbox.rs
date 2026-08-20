use std::rc::Rc;

use crate::motion::{Motion, StyledSlot};
use crate::theme::ActiveTheme;
use gpui::{
    div, prelude::*, px, App, ClickEvent, FocusHandle, FontWeight, IntoElement, KeyDownEvent,
    RenderOnce, SharedString, StyleRefinement, Styled, Window,
};

use crate::compat::{AccessibilityExt, Role, Toggled};

use crate::button::ButtonVariant;
use crate::chrome::{box_shadow, button_chrome, focus_ring};
use crate::icon::{Icon, IconName};

type CheckboxClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;
type CheckboxChangeHandler = Rc<dyn Fn(CheckState, &mut Window, &mut App) + 'static>;

struct CheckboxState {
    focus_handle: FocusHandle,
    state: CheckState,
    last_prop: CheckState,
}

/// Unchecked / checked / mixed, from Paper Checkboxes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckState {
    #[default]
    Off,
    On,
    Mixed,
}

impl CheckState {
    fn next(self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
            Self::Mixed => Self::On,
        }
    }

    fn debug_name(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::On => "on",
            Self::Mixed => "mixed",
        }
    }

    fn toggled(self) -> Toggled {
        match self {
            Self::Off => Toggled::False,
            Self::On => Toggled::True,
            Self::Mixed => Toggled::Mixed,
        }
    }
}

/// 16×16 radius-6 mark matching Paper `Glassy UI` → Checkboxes.
///
/// Without [`Checkbox::on_change`] / [`Checkbox::on_click`] the mark keeps its
/// own state (gallery specimens). With a listener, [`Checkbox::state`] /
/// [`Checkbox::checked`] is the source of truth each render.
#[derive(IntoElement)]
pub struct Checkbox {
    id: SharedString,
    state: CheckState,
    disabled: bool,
    label: Option<SharedString>,
    style: StyleRefinement,
    on_click: Option<CheckboxClickHandler>,
    on_change: Option<CheckboxChangeHandler>,
}

impl Checkbox {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            state: CheckState::Off,
            disabled: false,
            label: None,
            style: StyleRefinement::default(),
            on_click: None,
            on_change: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.state = if checked {
            CheckState::On
        } else {
            CheckState::Off
        };
        self
    }

    pub fn state(mut self, state: CheckState) -> Self {
        self.state = state;
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

    /// New mark after a pointer or keyboard activation.
    pub fn on_change(
        mut self,
        listener: impl Fn(CheckState, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(listener));
        self
    }
}

impl Styled for Checkbox {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Checkbox {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let motion_id = self.id.clone();
        let initial = self.state;
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, cx| CheckboxState {
            focus_handle: cx.focus_handle(),
            state: initial,
            last_prop: initial,
        });
        let controlled = self.on_change.is_some() || self.on_click.is_some();
        if controlled {
            if state.read(cx).state != self.state {
                state.update(cx, |checkbox, _| checkbox.state = self.state);
            }
        } else if state.read(cx).last_prop != self.state {
            state.update(cx, |checkbox, _| {
                checkbox.state = self.state;
                checkbox.last_prop = self.state;
            });
        }

        let check = if controlled {
            self.state
        } else {
            state.read(cx).state
        };
        let theme = cx.theme();
        let filled = matches!(check, CheckState::On | CheckState::Mixed);
        let variant = if self.disabled {
            ButtonVariant::Ghost
        } else if filled {
            ButtonVariant::Primary
        } else {
            ButtonVariant::Outline
        };
        let chrome = button_chrome(theme, variant);
        let mark = if self.disabled && filled {
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
        let focus_handle = state.read(cx).focus_handle.clone().tab_stop(interactive);
        let focused = focus_handle.is_focused(window);
        if focused {
            shadows.push(focus_ring(theme));
        }

        let box_el = div()
            .flex()
            .items_center()
            .justify_center()
            .size(px(16.))
            .flex_shrink_0()
            .rounded(px(6.))
            .border_1()
            .border_color(chrome.border)
            .bg(chrome.bg)
            .shadow(shadows)
            .when(check == CheckState::On, |el| {
                el.child(
                    Motion::new()
                        .id(format!("{motion_id}-check-on"))
                        .selection_in()
                        .child(Icon::new(IconName::Check).px(px(10.)).color(mark)),
                )
            })
            .when(check == CheckState::Mixed, |el| {
                el.child(
                    Motion::new()
                        .id(format!("{motion_id}-check-mixed"))
                        .selection_in()
                        .child(
                            div()
                                .w(px(8.))
                                .h(px(1.5))
                                .flex_shrink_0()
                                .rounded(px(1.))
                                .bg(mark),
                        ),
                )
            });

        let label_color = if self.disabled {
            theme.muted_fg()
        } else {
            theme.ink
        };
        let aria_label = self.label.clone();
        let debug_selector = format!("{}-{}", self.id, check.debug_name());

        let el = div()
            .id(self.id)
            .debug_selector(move || debug_selector.clone())
            .role(Role::CheckBox)
            .aria_toggled(check.toggled())
            .when_some(aria_label, |el, label| el.aria_label(label))
            .track_focus(&focus_handle)
            .tab_stop(interactive)
            .flex()
            .items_center()
            .gap(px(8.))
            .refine_style(&self.style)
            .when(interactive, |el| el.cursor_pointer())
            .when(!interactive, |el| el.cursor_default())
            .child(box_el)
            .when_some(self.label, |el, label| {
                el.child(
                    div()
                        .font_family(theme.font_family)
                        .font_weight(FontWeight::NORMAL)
                        .text_size(px(14.))
                        .line_height(px(20.))
                        .text_color(label_color)
                        .child(label),
                )
            });

        if interactive {
            let on_click = self.on_click;
            let on_change = self.on_change;
            let activate = Rc::new(
                move |event: &ClickEvent, window: &mut Window, cx: &mut App| {
                    let next = state.read(cx).state.next();
                    if !controlled {
                        state.update(cx, |checkbox, cx| {
                            checkbox.state = next;
                            cx.notify();
                        });
                    }
                    if let Some(on_change) = &on_change {
                        on_change(next, window, cx);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_false_clears_the_mark() {
        assert_eq!(Checkbox::new("x").checked(true).state, CheckState::On);
        assert_eq!(
            Checkbox::new("x").checked(true).checked(false).state,
            CheckState::Off
        );
    }

    #[test]
    fn mixed_activates_to_on() {
        assert_eq!(CheckState::Mixed.next(), CheckState::On);
        assert_eq!(CheckState::On.next(), CheckState::Off);
        assert_eq!(CheckState::Off.next(), CheckState::On);
    }
}
