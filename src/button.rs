use std::rc::Rc;

use crate::motion::StyledSlot;
use gpui::{
    div, prelude::*, px, App, ClickEvent, FocusHandle, FontWeight, IntoElement, KeyDownEvent,
    ParentElement, RenderOnce, SharedString, StyleRefinement, Styled, Window,
};

use crate::compat::{AccessibilityExt, Role};

use crate::theme::{ActiveTheme, Theme};

use crate::chrome::{box_shadow, button_chrome, focus_ring};
use crate::icon::{Icon, IconName};
use crate::spinner::Spinner;
use crate::tooltip::Tooltip;

type ButtonClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

struct ButtonState {
    focus_handle: FocusHandle,
}

/// Visual treatment from the Paper Buttons page.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Outline,
    OutlineDestructive,
    Ghost,
}

/// Height / type scale from Paper.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
    Icon,
}

impl ButtonSize {
    pub fn height(self) -> f32 {
        match self {
            Self::Small => 28.0,
            Self::Medium | Self::Icon => 36.0,
            Self::Large => 44.0,
        }
    }

    pub fn pad_x(self) -> f32 {
        match self {
            Self::Small => 12.0,
            Self::Medium => 16.0,
            Self::Large => 20.0,
            Self::Icon => 0.0,
        }
    }

    pub fn icon_pad_x(self) -> f32 {
        match self {
            Self::Icon => 0.0,
            _ => 14.0,
        }
    }

    pub fn font_size(self) -> f32 {
        match self {
            Self::Small => 12.0,
            Self::Medium | Self::Icon => 14.0,
            Self::Large => 15.0,
        }
    }

    pub fn line_height(self) -> f32 {
        match self {
            Self::Small => 16.0,
            Self::Medium | Self::Large | Self::Icon => 18.0,
        }
    }
}

/// Clickable glass button matching Paper `Glassy UI` → Buttons.
#[derive(IntoElement)]
pub struct Button {
    id: SharedString,
    label: Option<SharedString>,
    variant: ButtonVariant,
    size: ButtonSize,
    theme: Option<Theme>,
    disabled: bool,
    loading: bool,
    muted: bool,
    grouped: bool,
    leading_icon: Option<IconName>,
    trailing_icon: Option<IconName>,
    tooltip: Option<Tooltip>,
    focus_handle: Option<FocusHandle>,
    style: StyleRefinement,
    on_click: Option<ButtonClickHandler>,
}

impl Button {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: Some(label.into()),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            theme: None,
            disabled: false,
            loading: false,
            muted: false,
            grouped: false,
            leading_icon: None,
            trailing_icon: None,
            tooltip: None,
            focus_handle: None,
            style: StyleRefinement::default(),
            on_click: None,
        }
    }

    pub fn icon_only(id: impl Into<SharedString>, icon: IconName) -> Self {
        Self {
            id: id.into(),
            label: None,
            variant: ButtonVariant::Primary,
            size: ButtonSize::Icon,
            theme: None,
            disabled: false,
            loading: false,
            muted: false,
            grouped: false,
            leading_icon: Some(icon),
            trailing_icon: None,
            tooltip: None,
            focus_handle: None,
            style: StyleRefinement::default(),
            on_click: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Override the active app theme for this button only.
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Ghost/skip treatment: same chrome, muted label.
    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    /// Flush half of a [`ButtonGroup`].
    pub fn grouped(mut self, grouped: bool) -> Self {
        self.grouped = grouped;
        self
    }

    pub fn leading_icon(mut self, icon: IconName) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    pub fn trailing_icon(mut self, icon: IconName) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    pub fn tooltip(mut self, tooltip: Tooltip) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    pub fn focus_handle(mut self, focus_handle: FocusHandle) -> Self {
        self.focus_handle = Some(focus_handle);
        self
    }

    pub fn on_click(
        mut self,
        listener: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(listener));
        self
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, cx| ButtonState {
            focus_handle: cx.focus_handle(),
        });
        let theme = self.theme.unwrap_or_else(|| cx.theme());
        let variant = if self.disabled {
            ButtonVariant::Secondary
        } else {
            self.variant
        };
        let chrome = button_chrome(theme, variant);
        let fg = if self.disabled || self.muted {
            theme.muted_fg()
        } else {
            chrome.fg
        };

        let has_icon = self.loading || self.leading_icon.is_some() || self.trailing_icon.is_some();
        let pad_x = if self.size == ButtonSize::Icon {
            0.0
        } else if has_icon {
            self.size.icon_pad_x()
        } else {
            self.size.pad_x()
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

        let icon_size = if self.loading { 14.0 } else { 16.0 };
        let hover_bg = chrome.hover_bg;
        let interactive = !self.disabled && !self.loading;
        let focus_handle = self
            .focus_handle
            .unwrap_or_else(|| state.read(cx).focus_handle.clone())
            .tab_stop(interactive);
        let focused = focus_handle.is_focused(window);
        let aria_label = self.label.clone();
        let button_id = self.id.clone();
        let debug_selector = self.id.to_string();

        if focused {
            shadows.push(focus_ring(theme));
        }

        let el = div()
            .id(self.id)
            .debug_selector(move || debug_selector.clone())
            .role(Role::Button)
            .when_some(aria_label, |el, label| el.aria_label(label))
            .track_focus(&focus_handle)
            .tab_stop(interactive)
            .flex()
            .items_center()
            .justify_center()
            .when(has_icon && self.size != ButtonSize::Icon, |el| {
                el.gap(px(8.))
            })
            .when(self.grouped, |el| el.h_full())
            .when(!self.grouped, |el| el.h(px(self.size.height())))
            .when(self.size == ButtonSize::Icon, |el| {
                el.w(px(36.)).flex_shrink_0()
            })
            .when(self.size != ButtonSize::Icon, |el| el.px(px(pad_x)))
            .when(!self.grouped, |el| el.rounded(px(6.)))
            .border_1()
            .border_color(chrome.border)
            .bg(chrome.bg)
            .shadow(shadows)
            .text_color(fg)
            .font_family(theme.font_family)
            .font_weight(FontWeight::MEDIUM)
            .text_size(px(self.size.font_size()))
            .line_height(px(self.size.line_height()))
            .when(interactive, |el| {
                el.cursor_pointer().hover(move |s| s.bg(hover_bg))
            })
            .when(!interactive, |el| el.cursor_default())
            .refine_style(&self.style)
            .when(self.loading, |el| {
                el.child(Spinner::new().px(px(icon_size)).color(fg))
            })
            .when_some(self.leading_icon.filter(|_| !self.loading), |el, icon| {
                el.child(Icon::new(icon).px(px(icon_size)).color(fg))
            })
            .when_some(self.label, |el, label| el.child(label))
            .when_some(self.trailing_icon.filter(|_| !self.loading), |el, icon| {
                el.child(Icon::new(icon).px(px(icon_size)).color(fg))
            });

        let el = if interactive {
            if let Some(on_click) = self.on_click {
                let keyboard_click = on_click.clone();
                let click_focus = focus_handle.clone();
                el.on_key_down(move |event: &KeyDownEvent, window, cx| {
                    if event.keystroke.modifiers.modified() {
                        return;
                    }

                    if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                        keyboard_click(&ClickEvent::default(), window, cx);
                        window.refresh();
                        cx.stop_propagation();
                    }
                })
                .on_click(move |event, window, cx| {
                    click_focus.focus(window);
                    on_click(event, window, cx);
                    window.refresh();
                })
            } else {
                el
            }
        } else {
            el
        };

        if let Some(tooltip) = self.tooltip {
            tooltip.attach(button_id, el, window, cx).into_any_element()
        } else {
            el.into_any_element()
        }
    }
}

/// Clipped outline pill used for the Paper "Save draft | Publish" pair.
#[derive(IntoElement)]
pub struct ButtonGroup {
    theme: Option<Theme>,
    style: StyleRefinement,
    children: Vec<gpui::AnyElement>,
}

impl ButtonGroup {
    pub fn new() -> Self {
        Self {
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    /// Override the active app theme for this group only.
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl Default for ButtonGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for ButtonGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ButtonGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = self.theme.unwrap_or_else(|| cx.theme());
        let chrome = button_chrome(theme, ButtonVariant::Outline);
        div()
            .flex()
            .items_center()
            .h(px(36.))
            .rounded(px(6.))
            .overflow_hidden()
            .border_1()
            .border_color(chrome.border)
            .bg(chrome.bg)
            .shadow(vec![
                box_shadow(0., 1., chrome.inset, 0., 0.),
                box_shadow(0., chrome.shadow_y, chrome.shadow, chrome.shadow_blur, 0.),
            ])
            .refine_style(&self.style)
            .children(self.children)
    }
}
