use std::rc::Rc;
use std::time::Duration;

use crate::motion::{Ease, Motion, MotionStyle, StyledSlot, Transition};
use crate::theme::ActiveTheme;
use gpui::{
    div, prelude::*, px, App, ClickEvent, FocusHandle, FontWeight, IntoElement, KeyDownEvent,
    RenderOnce, SharedString, StyleRefinement, Styled, Window,
};

use crate::compat::{AccessibilityExt, Role, Toggled};

use crate::button::ButtonVariant;
use crate::chrome::{box_shadow, button_chrome, focus_ring};

type SwitchClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;
type SwitchChangeHandler = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

struct SwitchState {
    focus_handle: FocusHandle,
    on: bool,
    last_prop: bool,
    rendered: bool,
}

/// 36×20 pill matching Paper `Glassy UI` → Switches.
///
/// Without a listener the thumb keeps its own state. With [`Switch::on_change`]
/// or [`Switch::on_click`], [`Switch::on`] is the source of truth each render.
#[derive(IntoElement)]
pub struct Switch {
    id: SharedString,
    on: bool,
    disabled: bool,
    label: Option<SharedString>,
    style: StyleRefinement,
    on_click: Option<SwitchClickHandler>,
    on_change: Option<SwitchChangeHandler>,
}

impl Switch {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            on: false,
            disabled: false,
            label: None,
            style: StyleRefinement::default(),
            on_click: None,
            on_change: None,
        }
    }

    pub fn on(mut self, on: bool) -> Self {
        self.on = on;
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

    /// New on/off after a pointer or keyboard activation.
    pub fn on_change(mut self, listener: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(listener));
        self
    }
}

impl Styled for Switch {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Switch {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let motion_id = self.id.clone();
        let initial = self.on;
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, cx| SwitchState {
            focus_handle: cx.focus_handle(),
            on: initial,
            last_prop: initial,
            rendered: false,
        });
        let controlled = self.on_change.is_some() || self.on_click.is_some();
        if controlled {
            if state.read(cx).on != self.on {
                state.update(cx, |switch, _| switch.on = self.on);
            }
        } else if state.read(cx).last_prop != self.on {
            state.update(cx, |switch, _| {
                switch.on = self.on;
                switch.last_prop = self.on;
            });
        }

        let on = if controlled {
            self.on
        } else {
            state.read(cx).on
        };
        let animate_thumb = state.read(cx).rendered;
        if !animate_thumb {
            state.update(cx, |switch, _| switch.rendered = true);
        }
        let theme = cx.theme();
        let variant = if self.disabled {
            ButtonVariant::Ghost
        } else if on {
            ButtonVariant::Primary
        } else {
            ButtonVariant::Outline
        };
        let chrome = button_chrome(theme, variant);
        let thumb = if self.disabled {
            theme.muted_fg()
        } else if on || theme.is_dark() {
            theme.on_solid
        } else {
            button_chrome(theme, ButtonVariant::Primary).bg
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

        let track = div()
            .flex()
            .items_center()
            .when(on, |el| el.justify_end())
            .when(!on, |el| el.justify_start())
            .w(px(36.))
            .h(px(20.))
            .flex_shrink_0()
            .p(px(1.))
            .rounded(px(10.))
            .border_1()
            .border_color(chrome.border)
            .bg(chrome.bg)
            .shadow(shadows)
            .child(
                Motion::new()
                    .id(format!("{motion_id}-thumb-{on}"))
                    .initial(MotionStyle::new().x(px(if animate_thumb {
                        if on {
                            -16.
                        } else {
                            16.
                        }
                    } else {
                        0.
                    })))
                    .animate(MotionStyle::new().x(px(0.)))
                    .transition(Transition::tween(Duration::from_millis(180)).ease(Ease::EaseOut))
                    .child(
                        div()
                            .size(px(16.))
                            .flex_shrink_0()
                            .rounded(px(8.))
                            .bg(thumb)
                            .shadow(vec![
                                box_shadow(0., 1., theme.on_solid.opacity(0.22), 0., 0.),
                                box_shadow(0., 2., chrome.shadow, 6., 0.),
                            ]),
                    ),
            );

        let label_color = if self.disabled {
            theme.muted_fg()
        } else {
            theme.ink
        };
        let aria_label = self.label.clone();
        let debug_selector = format!("{}-{}", self.id, if on { "on" } else { "off" });

        let el = div()
            .id(self.id)
            .debug_selector(move || debug_selector.clone())
            .role(Role::Switch)
            .aria_toggled(if on { Toggled::True } else { Toggled::False })
            .when_some(aria_label, |el, label| el.aria_label(label))
            .track_focus(&focus_handle)
            .tab_stop(interactive)
            .flex()
            .items_center()
            .gap(px(8.))
            .refine_style(&self.style)
            .when(interactive, |el| el.cursor_pointer())
            .when(!interactive, |el| el.cursor_default())
            .child(track)
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
                    let next = !state.read(cx).on;
                    if !controlled {
                        state.update(cx, |switch, cx| {
                            switch.on = next;
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
