use gpui::{
    div, prelude::*, px, App, FontWeight, IntoElement, RenderOnce, SharedString, StyleRefinement,
    Styled, Window,
};
use gpui_kit_motion::StyledSlot;
use gpui_kit_theme::ActiveTheme;

/// 13/500 name that sits above a field. Never inside it.
#[derive(IntoElement)]
pub struct Label {
    text: SharedString,
    required: bool,
    optional: bool,
    style: StyleRefinement,
}

impl Label {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            required: false,
            optional: false,
            style: StyleRefinement::default(),
        }
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }
}

impl Styled for Label {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .flex()
            .items_center()
            .gap(px(4.))
            .font_family(theme.font_family)
            .font_weight(FontWeight::MEDIUM)
            .text_size(px(13.))
            .line_height(px(16.))
            .text_color(theme.label)
            .refine_style(&self.style)
            .child(self.text)
            .when(self.required, |el| {
                el.child(
                    div()
                        .font_family(theme.font_family)
                        .font_weight(FontWeight::MEDIUM)
                        .text_size(px(13.))
                        .line_height(px(16.))
                        .text_color(theme.destructive)
                        .child("*"),
                )
            })
            .when(self.optional, |el| {
                el.child(
                    div()
                        .font_family(theme.font_family)
                        .font_weight(FontWeight::MEDIUM)
                        .text_size(px(12.))
                        .line_height(px(16.))
                        .text_color(theme.body)
                        .child("Optional"),
                )
            })
    }
}
