use gpui::{
    div, prelude::*, px, App, BoxShadow, ClickEvent, FontWeight, IntoElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window,
};
use gpui_kit_motion::StyledSlot;
use gpui_kit_theme::ActiveTheme;

use crate::button::ButtonVariant;
use crate::chrome::button_chrome;
use crate::icon::{Icon, IconName};

type CheckboxClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

struct CheckboxState {
    state: CheckState,
}

/// Unchecked / checked / mixed, from Paper Checkboxes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckState {
    #[default]
    Off,
    On,
    Mixed,
}

/// 16×16 radius-6 mark matching Paper `Grafik UI` → Checkboxes.
#[derive(IntoElement)]
pub struct Checkbox {
    id: SharedString,
    state: CheckState,
    disabled: bool,
    label: Option<SharedString>,
    style: StyleRefinement,
    on_click: Option<CheckboxClickHandler>,
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
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        if checked {
            self.state = CheckState::On;
        }
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
        self.on_click = Some(Box::new(listener));
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
        let initial = self.state;
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, _| CheckboxState {
            state: initial,
        });
        let check = state.read(cx).state;
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

        let mut shadows = vec![BoxShadow::new(px(0.), px(1.), chrome.inset).inset()];
        if chrome.shadow_blur > 0.0 {
            shadows.push(
                BoxShadow::new(px(0.), px(chrome.shadow_y), chrome.shadow)
                    .blur_radius(px(chrome.shadow_blur)),
            );
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
                el.child(Icon::new(IconName::Check).px(px(10.)).color(mark))
            })
            .when(check == CheckState::Mixed, |el| {
                el.child(
                    div()
                        .w(px(8.))
                        .h(px(1.5))
                        .flex_shrink_0()
                        .rounded(px(1.))
                        .bg(mark),
                )
            });

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
            el.on_click(move |event, window, cx| {
                state.update(cx, |checkbox, cx| {
                    checkbox.state = match checkbox.state {
                        CheckState::Off => CheckState::On,
                        CheckState::On => CheckState::Off,
                        CheckState::Mixed => CheckState::On,
                    };
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
