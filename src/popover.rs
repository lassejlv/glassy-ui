use std::rc::Rc;

use gpui::{
    anchored, deferred, div, point, prelude::*, px, relative, AnyElement, App, Corner, Div, Entity,
    FocusHandle, FontWeight, IntoElement, KeyDownEvent, MouseButton, ParentElement, Pixels,
    RenderOnce, SharedString, StyleRefinement, Styled, Window,
};

use crate::chrome::box_shadow;
use crate::compat::{AccessibilityExt, Role, StyleCompatExt};
use crate::motion::{Motion, StyledSlot};
use crate::theme::{paint, ActiveTheme, Theme, ThemeKind};

type PopoverOpenChangeHandler = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

struct PopoverState {
    focus_handle: FocusHandle,
    open: bool,
}

fn set_open(
    state: &Entity<PopoverState>,
    open: bool,
    on_open_change: Option<&PopoverOpenChangeHandler>,
    window: &mut Window,
    cx: &mut App,
) {
    if state.read(cx).open == open {
        return;
    }

    state.update(cx, |state, cx| {
        state.open = open;
        cx.notify();
    });

    if let Some(on_open_change) = on_open_change {
        on_open_change(open, window, cx);
    }
    window.refresh();
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverPlacement {
    Top,
    #[default]
    Bottom,
    Start,
    End,
}

impl PopoverPlacement {
    fn anchor(self) -> Corner {
        match self {
            Self::Top => Corner::BottomLeft,
            Self::Bottom => Corner::TopLeft,
            Self::Start => Corner::TopRight,
            Self::End => Corner::TopLeft,
        }
    }

    fn offset(self, gap: Pixels) -> gpui::Point<Pixels> {
        match self {
            Self::Top => point(px(0.), -gap),
            Self::Bottom => point(px(0.), gap),
            Self::Start => point(-gap, px(0.)),
            Self::End => point(gap, px(0.)),
        }
    }

    fn marker(self) -> Div {
        let marker = div().absolute().size(px(0.));

        match self {
            Self::Top => marker.left(px(0.)).top(px(0.)),
            Self::Bottom => marker.left(px(0.)).top(relative(1.)),
            Self::Start => marker.left(px(0.)).top(relative(0.5)),
            Self::End => marker.left(relative(1.)).top(relative(0.5)),
        }
    }
}

/// Trigger-anchored non-modal content with click, Escape, and outside dismissal.
#[derive(IntoElement)]
pub struct Popover {
    id: SharedString,
    controlled_open: Option<bool>,
    default_open: bool,
    placement: PopoverPlacement,
    gap: Pixels,
    trigger_label: SharedString,
    trigger: Option<AnyElement>,
    on_open_change: Option<PopoverOpenChangeHandler>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Popover {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            controlled_open: None,
            default_open: false,
            placement: PopoverPlacement::Bottom,
            gap: px(6.),
            trigger_label: SharedString::from("Toggle popover"),
            trigger: None,
            on_open_change: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.controlled_open = Some(open);
        self
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    pub fn placement(mut self, placement: PopoverPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into();
        self
    }

    pub fn trigger_label(mut self, label: impl Into<SharedString>) -> Self {
        self.trigger_label = label.into();
        self
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn on_open_change(
        mut self,
        listener: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(listener));
        self
    }
}

impl Styled for Popover {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Popover {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Popover {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let initial_open = self.controlled_open.unwrap_or(self.default_open);
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, cx| PopoverState {
            focus_handle: cx.focus_handle().tab_stop(true),
            open: initial_open,
        });

        if let Some(controlled_open) = self.controlled_open {
            if state.read(cx).open != controlled_open {
                state.update(cx, |state, _| state.open = controlled_open);
            }
        }

        let open = state.read(cx).open;
        let focus_handle = state.read(cx).focus_handle.clone();
        let focused = focus_handle.is_focused(window);
        let trigger_selector = format!("{}-trigger", self.id);
        let content_selector = format!("{}-content", self.id);
        let surface_id = format!("{}-surface", self.id);
        let trigger_id = SharedString::from(trigger_selector.clone());
        let content_id = SharedString::from(content_selector.clone());
        let click_state = state.clone();
        let click_change = self.on_open_change.clone();
        let click_focus = focus_handle.clone();
        let click_next_open = !open;
        let keyboard_state = state.clone();
        let keyboard_change = self.on_open_change.clone();
        let keyboard_focus = focus_handle.clone();
        let dismiss_state = state.clone();
        let dismiss_change = self.on_open_change.clone();
        let ring = if cx.theme().is_dark() {
            paint(0xFFFFFF24)
        } else {
            paint(0x18181B24)
        };

        let trigger = div()
            .id(trigger_id)
            .debug_selector(move || trigger_selector.clone())
            .role(Role::Button)
            .aria_label(self.trigger_label)
            .aria_expanded(open)
            .track_focus(&focus_handle)
            .tab_stop(true)
            .relative()
            .flex()
            .items_center()
            .rounded(px(6.))
            .cursor_pointer()
            .when(focused, |el| {
                el.shadow(vec![box_shadow(0., 0., ring, 0., 3.)])
            })
            .on_key_down(move |event: &KeyDownEvent, window, cx| {
                if event.keystroke.modifiers.modified() {
                    return;
                }

                match event.keystroke.key.as_str() {
                    "enter" | "space" => {
                        let next_open = !keyboard_state.read(cx).open;
                        set_open(
                            &keyboard_state,
                            next_open,
                            keyboard_change.as_ref(),
                            window,
                            cx,
                        );
                        cx.stop_propagation();
                    }
                    "escape" if keyboard_state.read(cx).open => {
                        set_open(&keyboard_state, false, keyboard_change.as_ref(), window, cx);
                        keyboard_focus.focus(window);
                        cx.stop_propagation();
                    }
                    _ => {}
                }
            })
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                click_focus.focus(window);
                set_open(
                    &click_state,
                    click_next_open,
                    click_change.as_ref(),
                    window,
                    cx,
                );
            })
            .when_some(self.trigger, |el, trigger| el.child(trigger));

        let placement = self.placement;
        let gap = self.gap;
        let surface = Motion::new().id(surface_id).surface_in().child(
            div()
                .id(content_id)
                .debug_selector(move || content_selector.clone())
                .role(Role::Dialog)
                .occlude()
                .on_mouse_down_out(move |_, window, cx| {
                    set_open(&dismiss_state, false, dismiss_change.as_ref(), window, cx);
                })
                .children(self.children),
        );
        let popup = match placement {
            PopoverPlacement::Top | PopoverPlacement::Bottom => placement
                .marker()
                .child(
                    deferred(
                        anchored()
                            .anchor(placement.anchor())
                            .offset(placement.offset(gap))
                            .snap_to_window_with_margin(px(8.))
                            .child(surface),
                    )
                    .with_priority(2),
                )
                .into_any_element(),
            PopoverPlacement::Start => div()
                .absolute()
                .top(px(0.))
                .bottom(px(0.))
                .right(relative(1.))
                .w(px(0.))
                .flex()
                .items_center()
                .justify_end()
                .child(deferred(div().mr(gap).child(surface)).with_priority(2))
                .into_any_element(),
            PopoverPlacement::End => div()
                .absolute()
                .top(px(0.))
                .bottom(px(0.))
                .left(relative(1.))
                .w(px(0.))
                .flex()
                .items_center()
                .justify_start()
                .child(deferred(div().ml(gap).child(surface)).with_priority(2))
                .into_any_element(),
        };

        div()
            .relative()
            .flex()
            .flex_none()
            .self_start()
            .items_center()
            .refine_style(&self.style)
            .child(trigger)
            .when(open, |el| el.child(popup))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PopoverChrome {
    background: gpui::Hsla,
    border: gpui::Hsla,
    inset: gpui::Hsla,
    shadow: gpui::Hsla,
}

fn popover_chrome(theme: Theme) -> PopoverChrome {
    match theme.kind {
        ThemeKind::Light => PopoverChrome {
            background: paint(0xFFFFFF85),
            border: paint(0xFFFFFFB8),
            inset: paint(0xFFFFFFE6),
            shadow: paint(0x0F172A0F),
        },
        ThemeKind::Dark => PopoverChrome {
            background: paint(0xFFFFFF12),
            border: paint(0xFFFFFF1A),
            inset: paint(0xFFFFFF1F),
            shadow: paint(0x00000047),
        },
    }
}

/// Secondary-glass popover panel matching the design spec `Glassy UI` → Popovers.
#[derive(IntoElement)]
pub struct PopoverContent {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl PopoverContent {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for PopoverContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for PopoverContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for PopoverContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PopoverContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let chrome = popover_chrome(cx.theme());

        div()
            .flex()
            .flex_col()
            .w(px(220.))
            .flex_shrink_0()
            .p(px(12.))
            .gap(px(4.))
            .rounded(px(6.))
            .border_1()
            .border_color(chrome.border)
            .bg(chrome.background)
            .shadow(vec![
                box_shadow(0., 1., chrome.inset, 0., 0.),
                box_shadow(0., 6., chrome.shadow, 16., 0.),
            ])
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Medium page-meta heading used inside a [`PopoverContent`].
#[derive(IntoElement)]
pub struct PopoverTitle {
    text: SharedString,
    style: StyleRefinement,
}

impl PopoverTitle {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            style: StyleRefinement::default(),
        }
    }
}

impl Styled for PopoverTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for PopoverTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .font_family(theme.font_family)
            .font_weight(FontWeight::MEDIUM)
            .text_size(px(14.))
            .line_height(px(18.))
            .text_color(theme.ink)
            .refine_style(&self.style)
            .child(self.text)
    }
}

/// Supporting page metadata used inside a [`PopoverContent`].
#[derive(IntoElement)]
pub struct PopoverDescription {
    text: SharedString,
    style: StyleRefinement,
}

impl PopoverDescription {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            style: StyleRefinement::default(),
        }
    }
}

impl Styled for PopoverDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for PopoverDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .font_family(theme.font_family)
            .font_weight(FontWeight::NORMAL)
            .text_size(px(13.))
            .line_height(px(18.))
            .text_color(theme.body)
            .refine_style(&self.style)
            .child(self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_material_matches_spec() {
        let chrome = popover_chrome(Theme::light());
        assert_eq!(chrome.background, paint(0xFFFFFF85));
        assert_eq!(chrome.border, paint(0xFFFFFFB8));
        assert_eq!(chrome.inset, paint(0xFFFFFFE6));
        assert_eq!(chrome.shadow, paint(0x0F172A0F));
    }

    #[test]
    fn dark_material_matches_spec() {
        let chrome = popover_chrome(Theme::dark());
        assert_eq!(chrome.background, paint(0xFFFFFF12));
        assert_eq!(chrome.border, paint(0xFFFFFF1A));
        assert_eq!(chrome.inset, paint(0xFFFFFF1F));
        assert_eq!(chrome.shadow, paint(0x00000047));
    }
}
