use crate::motion::StyledSlot;
use crate::theme::ActiveTheme;
use gpui::{
    div, prelude::*, px, App, BoxShadow, FontWeight, IntoElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window,
};

use crate::button::ButtonVariant;
use crate::chrome::button_chrome;

/// Ghost glass chip matching Paper `Grafik UI` → Kbd. Not a button.
#[derive(IntoElement)]
pub struct Kbd {
    keys: SharedString,
    style: StyleRefinement,
}

impl Kbd {
    pub fn new(keys: impl Into<SharedString>) -> Self {
        Self {
            keys: keys.into(),
            style: StyleRefinement::default(),
        }
    }
}

impl Styled for Kbd {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Kbd {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let chrome = button_chrome(theme, ButtonVariant::Ghost);

        div()
            .flex()
            .items_center()
            .justify_center()
            .h(px(22.))
            .min_w(px(22.))
            .flex_shrink_0()
            .px(px(6.))
            .rounded(px(6.))
            .border_1()
            .border_color(chrome.border)
            .bg(chrome.bg)
            .shadow(vec![BoxShadow::new(px(0.), px(1.), chrome.inset).inset()])
            .cursor_default()
            .refine_style(&self.style)
            .child(
                div()
                    .font_family(theme.font_family)
                    .font_weight(FontWeight::MEDIUM)
                    .text_size(px(12.))
                    .line_height(px(16.))
                    .text_color(chrome.fg)
                    .child(self.keys),
            )
    }
}
