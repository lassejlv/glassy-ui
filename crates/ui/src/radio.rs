use gpui::{
    div, prelude::*, px, App, BoxShadow, ClickEvent, FontWeight, IntoElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window,
};
use gpui_kit_motion::StyledSlot;
use gpui_kit_theme::ActiveTheme;

use crate::button::ButtonVariant;
use crate::chrome::button_chrome;

type RadioClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// 16×16 circle matching Paper `Grafik UI` → Radios.
#[derive(IntoElement)]
pub struct Radio {
    id: SharedString,
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
            selected: false,
            disabled: false,
            label: None,
            style: StyleRefinement::default(),
            on_click: None,
        }
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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let variant = if self.disabled {
            ButtonVariant::Ghost
        } else if self.selected {
            ButtonVariant::Primary
        } else {
            ButtonVariant::Outline
        };
        let chrome = button_chrome(theme, variant);
        let dot = if self.disabled && self.selected {
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
            .when(self.selected, |el| {
                el.child(div().size(px(6.)).flex_shrink_0().rounded(px(3.)).bg(dot))
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
            if let Some(on_click) = self.on_click {
                el.on_click(on_click)
            } else {
                el
            }
        } else {
            el
        }
    }
}
