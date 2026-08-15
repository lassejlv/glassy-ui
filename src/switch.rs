use std::time::Duration;

use crate::motion::{Ease, Motion, MotionStyle, StyledSlot, Transition};
use crate::theme::ActiveTheme;
use gpui::{
    div, prelude::*, px, App, BoxShadow, ClickEvent, FontWeight, IntoElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window,
};

use crate::button::ButtonVariant;
use crate::chrome::button_chrome;

type SwitchClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

struct SwitchState {
    on: bool,
    rendered: bool,
}

/// 36×20 pill matching Paper `Grafik UI` → Switches.
#[derive(IntoElement)]
pub struct Switch {
    id: SharedString,
    on: bool,
    disabled: bool,
    label: Option<SharedString>,
    style: StyleRefinement,
    on_click: Option<SwitchClickHandler>,
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
        self.on_click = Some(Box::new(listener));
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
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, _| SwitchState {
            on: initial,
            rendered: false,
        });
        let on = state.read(cx).on;
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

        let mut shadows = vec![BoxShadow::new(px(0.), px(1.), chrome.inset).inset()];
        if chrome.shadow_blur > 0.0 {
            shadows.push(
                BoxShadow::new(px(0.), px(chrome.shadow_y), chrome.shadow)
                    .blur_radius(px(chrome.shadow_blur)),
            );
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
                                BoxShadow::new(px(0.), px(1.), theme.on_solid.opacity(0.22))
                                    .inset(),
                                BoxShadow::new(px(0.), px(2.), chrome.shadow).blur_radius(px(6.)),
                            ]),
                    ),
            );

        let interactive = !self.disabled;
        let label_color = if self.disabled {
            theme.muted_fg()
        } else {
            theme.ink
        };

        let el = div()
            .id(self.id)
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
            el.on_click(move |event, window, cx| {
                state.update(cx, |switch, cx| {
                    switch.on = !switch.on;
                    cx.notify();
                });
                if let Some(on_click) = &on_click {
                    on_click(event, window, cx);
                }
            })
        } else {
            el
        }
    }
}
