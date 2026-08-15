use crate::motion::{Motion, StyledSlot};
use crate::theme::ActiveTheme;
use gpui::{
    div, prelude::*, px, App, BoxShadow, ClickEvent, FontWeight, IntoElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window,
};

use crate::button::ButtonVariant;
use crate::chrome::button_chrome;

type RadioClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

struct RadioGroupState {
    selected: Option<SharedString>,
}

/// 16×16 circle matching Paper `Grafik UI` → Radios.
#[derive(IntoElement)]
pub struct Radio {
    id: SharedString,
    group: Option<SharedString>,
    selected: bool,
    disabled: bool,
    label: Option<SharedString>,
    style: StyleRefinement,
    on_click: Option<RadioClickHandler>,
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
        self.on_click = Some(Box::new(listener));
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
        let state_key = self.group.clone().unwrap_or_else(|| self.id.clone());
        let state = window.use_keyed_state(state_key, cx, move |_, _| RadioGroupState {
            selected: if initially_selected { Some(seed) } else { None },
        });
        state.update(cx, |group, _| {
            if group.selected.is_none() && initially_selected {
                group.selected = Some(radio_id.clone());
            }
        });
        let selected = state
            .read(cx)
            .selected
            .as_ref()
            .is_some_and(|id| id.as_ref() == self.id.as_ref());
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

        let mut shadows = vec![BoxShadow::new(px(0.), px(1.), chrome.inset).inset()];
        if chrome.shadow_blur > 0.0 {
            shadows.push(
                BoxShadow::new(px(0.), px(chrome.shadow_y), chrome.shadow)
                    .blur_radius(px(chrome.shadow_blur)),
            );
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
            el.on_click(move |event, window, cx| {
                state.update(cx, |group, cx| {
                    group.selected = Some(radio_id.clone());
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
