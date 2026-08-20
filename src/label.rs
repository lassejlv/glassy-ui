use crate::motion::StyledSlot;
use crate::theme::ActiveTheme;
use gpui::{
    div, prelude::*, px, App, FocusHandle, FontWeight, IntoElement, RenderOnce, Role, SharedString,
    StyleRefinement, Styled, Window,
};

/// 13/500 name that sits above a field. Never inside it.
#[derive(IntoElement)]
pub struct Label {
    id: SharedString,
    text: SharedString,
    required: bool,
    optional: bool,
    focus_handle: Option<FocusHandle>,
    style: StyleRefinement,
}

impl Label {
    pub fn new(text: impl Into<SharedString>) -> Self {
        let text = text.into();
        Self {
            id: SharedString::from(format!("label-{text}")),
            text,
            required: false,
            optional: false,
            focus_handle: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }

    /// Clicking the label focuses this control.
    pub fn focus_handle(mut self, focus_handle: FocusHandle) -> Self {
        self.focus_handle = Some(focus_handle);
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
        let focus_handle = self.focus_handle;
        div()
            .id(self.id)
            .role(Role::Label)
            .flex()
            .items_center()
            .gap(px(4.))
            .font_family(theme.font_family)
            .font_weight(FontWeight::MEDIUM)
            .text_size(px(13.))
            .line_height(px(16.))
            .text_color(theme.label)
            .refine_style(&self.style)
            .when(focus_handle.is_some(), |el| el.cursor_pointer())
            .when_some(focus_handle, |el, focus_handle| {
                el.on_click(move |_, window, cx| {
                    focus_handle.focus(window, cx);
                })
            })
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
