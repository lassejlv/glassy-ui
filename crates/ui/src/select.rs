use std::rc::Rc;

use gpui::{
    div, prelude::*, px, App, BoxShadow, ClickEvent, FontWeight, IntoElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window,
};
use gpui_kit_motion::StyledSlot;
use gpui_kit_theme::ActiveTheme;

use crate::button::ButtonVariant;
use crate::chrome::{button_chrome, field_chrome, FieldState};
use crate::icon::{Icon, IconName};

type SelectClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;
type SelectPickHandler = Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>;

struct SelectState {
    open: bool,
    value: Option<SharedString>,
}

/// One row in the open list.
#[derive(Clone, Debug)]
pub struct SelectItem {
    pub value: SharedString,
    pub disabled: bool,
}

impl SelectItem {
    pub fn new(value: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Closed field is Input chrome plus a chevron. Open list is secondary glass.
#[derive(IntoElement)]
pub struct Select {
    id: SharedString,
    placeholder: SharedString,
    value: Option<SharedString>,
    items: Vec<SelectItem>,
    disabled: bool,
    focused: bool,
    open: bool,
    style: StyleRefinement,
    on_click: Option<SelectClickHandler>,
    on_select: Option<SelectPickHandler>,
}

impl Select {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            placeholder: SharedString::from("Choose format"),
            value: None,
            items: Vec::new(),
            disabled: false,
            focused: false,
            open: false,
            style: StyleRefinement::default(),
            on_click: None,
            on_select: None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = SelectItem>) -> Self {
        self.items = items.into_iter().collect();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn on_click(
        mut self,
        listener: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(listener));
        self
    }

    pub fn on_select(
        mut self,
        listener: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(listener));
        self
    }
}

impl Styled for Select {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Select {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let initial_open = self.open;
        let initial_value = self.value.clone();
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, _| SelectState {
            open: initial_open,
            value: initial_value,
        });
        let open = state.read(cx).open;
        let value = state.read(cx).value.clone();
        let theme = cx.theme();
        let field_state = if self.disabled {
            FieldState::Disabled
        } else if self.focused || open {
            FieldState::Focus
        } else {
            FieldState::Rest
        };
        let chrome = field_chrome(theme, field_state);
        let list_chrome = button_chrome(theme, ButtonVariant::Secondary);
        let ghost = button_chrome(theme, ButtonVariant::Ghost);

        let mut shadows = vec![BoxShadow::new(px(0.), px(1.), chrome.inset).inset()];
        if chrome.shadow_blur > 0.0 {
            shadows.push(
                BoxShadow::new(px(0.), px(chrome.shadow_y), chrome.shadow)
                    .blur_radius(px(chrome.shadow_blur)),
            );
        }
        if let Some(ring) = chrome.ring {
            shadows.push(BoxShadow::new(px(0.), px(0.), ring).spread_radius(px(3.)));
        }

        let has_value = value.is_some();
        let label = value.clone().unwrap_or_else(|| self.placeholder.clone());
        let label_color = if self.disabled || !has_value {
            chrome.placeholder
        } else {
            chrome.fg
        };

        let interactive = !self.disabled;
        let on_click = self.on_click.clone();
        let trigger_state = state.clone();
        let mut trigger = div()
            .id(self.id.clone())
            .flex()
            .items_center()
            .gap(px(8.))
            .w(px(280.))
            .h(px(36.))
            .flex_shrink_0()
            .px(px(14.))
            .rounded(px(6.))
            .border_1()
            .border_color(chrome.border)
            .bg(chrome.bg)
            .shadow(shadows)
            .when(interactive, |el| el.cursor_pointer())
            .when(!interactive, |el| el.cursor_default())
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.))
                    .font_family(theme.font_family)
                    .font_weight(FontWeight::NORMAL)
                    .text_size(px(14.))
                    .line_height(px(18.))
                    .text_color(label_color)
                    .child(label),
            )
            .child(
                Icon::new(IconName::ChevronDown)
                    .px(px(16.))
                    .color(theme.label),
            );

        if interactive {
            trigger = trigger.on_click(move |event, window, cx| {
                trigger_state.update(cx, |select, cx| {
                    select.open = !select.open;
                    cx.notify();
                });
                if let Some(on_click) = &on_click {
                    on_click(event, window, cx);
                }
            });
        }

        let mut list_shadows = vec![BoxShadow::new(px(0.), px(1.), list_chrome.inset).inset()];
        if list_chrome.shadow_blur > 0.0 {
            list_shadows.push(
                BoxShadow::new(px(0.), px(list_chrome.shadow_y), list_chrome.shadow)
                    .blur_radius(px(list_chrome.shadow_blur)),
            );
        }

        let selected = value.clone();
        let on_select = self.on_select.clone();
        let ghost_bg = ghost.bg;
        let ghost_hover = ghost.hover_bg;
        let select_id = self.id.clone();
        let list = div()
            .flex()
            .flex_col()
            .w(px(280.))
            .flex_shrink_0()
            .p(px(4.))
            .rounded(px(6.))
            .border_1()
            .border_color(list_chrome.border)
            .bg(list_chrome.bg)
            .shadow(list_shadows)
            .children(self.items.into_iter().enumerate().map(|(index, item)| {
                let is_selected = selected
                    .as_ref()
                    .is_some_and(|value| value.as_ref() == item.value.as_ref());
                let enabled = !item.disabled;
                let text_color = if item.disabled {
                    theme.label
                } else {
                    theme.ink
                };
                let item_value = item.value.clone();
                let on_select = on_select.clone();
                let row_state = state.clone();
                let row_id = SharedString::from(format!("{select_id}-{index}"));

                let mut row = div()
                    .id(row_id)
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .h(px(32.))
                    .flex_shrink_0()
                    .px(px(8.))
                    .rounded(px(6.))
                    .when(is_selected, |el| el.bg(ghost_bg))
                    .when(enabled, |el| {
                        el.cursor_pointer().hover(move |s| s.bg(ghost_hover))
                    })
                    .when(!enabled, |el| el.cursor_default())
                    .child(div().size(px(16.)).flex_shrink_0().when(is_selected, |el| {
                        el.child(Icon::new(IconName::Check).px(px(16.)).color(theme.ink))
                    }))
                    .child(
                        div()
                            .font_family(theme.font_family)
                            .font_weight(FontWeight::NORMAL)
                            .text_size(px(14.))
                            .line_height(px(18.))
                            .text_color(text_color)
                            .child(item.value.clone()),
                    );

                if enabled {
                    row = row.on_click(move |_, window, cx| {
                        row_state.update(cx, |select, cx| {
                            select.value = Some(item_value.clone());
                            select.open = false;
                            cx.notify();
                        });
                        if let Some(on_select) = &on_select {
                            on_select(&item_value, window, cx);
                        }
                    });
                }

                row
            }));

        div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .w(px(280.))
            .flex_shrink_0()
            .refine_style(&self.style)
            .child(trigger)
            .when(open && !self.disabled, |el| el.child(list))
    }
}
