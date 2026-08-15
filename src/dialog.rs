use std::rc::Rc;

use gpui::{
    deferred, div, prelude::*, px, AnyElement, App, BoxShadow, Entity, FocusHandle, FontWeight,
    KeyDownEvent, MouseButton, RenderOnce, SharedString, StyleRefinement, Styled, Window,
};

use crate::motion::{Motion, StyledSlot};
use crate::theme::{paint, ActiveTheme, Theme, ThemeKind};

type DialogDismissHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

struct DialogState {
    focus_handle: FocusHandle,
    previous_focus: Option<FocusHandle>,
    open: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct DialogChrome {
    background: gpui::Hsla,
    border: gpui::Hsla,
    inset: gpui::Hsla,
    shadow: gpui::Hsla,
    scrim: gpui::Hsla,
}

fn dialog_chrome(theme: Theme) -> DialogChrome {
    match theme.kind {
        ThemeKind::Light => DialogChrome {
            background: paint(0xFFFFFF85),
            border: paint(0xFFFFFFB8),
            inset: paint(0xFFFFFFE6),
            shadow: paint(0x0F172A0F),
            scrim: paint(0x18181B47),
        },
        ThemeKind::Dark => DialogChrome {
            background: paint(0xFFFFFF12),
            border: paint(0xFFFFFF1A),
            inset: paint(0xFFFFFF1F),
            shadow: paint(0x00000047),
            scrim: paint(0x00000073),
        },
    }
}

fn restore_focus(state: &Entity<DialogState>, window: &mut Window, cx: &mut App) {
    let (dialog_focus, previous_focus) = {
        let state = state.read(cx);
        (state.focus_handle.clone(), state.previous_focus.clone())
    };

    if dialog_focus.contains_focused(window, cx) {
        if let Some(previous_focus) = previous_focus {
            previous_focus.focus(window, cx);
        }
    }

    state.update(cx, |state, cx| {
        state.open = false;
        state.previous_focus = None;
        cx.notify();
    });
}

fn dismiss_dialog(
    state: &Entity<DialogState>,
    on_dismiss: Option<&DialogDismissHandler>,
    window: &mut Window,
    cx: &mut App,
) {
    restore_focus(state, window, cx);
    if let Some(on_dismiss) = on_dismiss {
        on_dismiss(window, cx);
    }
}

/// Controlled modal overlay with Escape, scrim dismissal, and focus restoration.
#[derive(IntoElement)]
pub struct Dialog {
    id: SharedString,
    open: bool,
    dismiss_on_scrim: bool,
    initial_focus: Option<FocusHandle>,
    focus_cycle: Vec<FocusHandle>,
    on_dismiss: Option<DialogDismissHandler>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Dialog {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            open: false,
            dismiss_on_scrim: true,
            initial_focus: None,
            focus_cycle: Vec::new(),
            on_dismiss: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn dismiss_on_scrim(mut self, dismiss_on_scrim: bool) -> Self {
        self.dismiss_on_scrim = dismiss_on_scrim;
        self
    }

    pub fn initial_focus(mut self, focus_handle: FocusHandle) -> Self {
        self.initial_focus = Some(focus_handle);
        self
    }

    pub fn focus_cycle(mut self, focus_handles: impl IntoIterator<Item = FocusHandle>) -> Self {
        self.focus_cycle = focus_handles.into_iter().collect();
        self
    }

    pub fn on_dismiss(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(listener));
        self
    }
}

impl Styled for Dialog {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Dialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Dialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, cx| DialogState {
            focus_handle: cx.focus_handle().tab_stop(true),
            previous_focus: None,
            open: false,
        });
        let focus_handle = state.read(cx).focus_handle.clone();
        let was_open = state.read(cx).open;

        if self.open && !was_open {
            let previous_focus = window.focused(cx);
            state.update(cx, |state, _| {
                state.open = true;
                state.previous_focus = previous_focus;
            });
            self.initial_focus
                .as_ref()
                .unwrap_or(&focus_handle)
                .focus(window, cx);
        } else if !self.open && was_open {
            restore_focus(&state, window, cx);
        }

        if !self.open {
            return div().into_any_element();
        }

        let chrome = dialog_chrome(cx.theme());
        let overlay_selector = format!("{}-overlay", self.id);
        let panel_selector = format!("{}-panel", self.id);
        let surface_id = format!("{}-surface", self.id);
        let overlay_id = SharedString::from(overlay_selector.clone());
        let panel_id = SharedString::from(panel_selector.clone());
        let scrim_state = state.clone();
        let scrim_dismiss = self.on_dismiss.clone();
        let keyboard_state = state.clone();
        let keyboard_dismiss = self.on_dismiss.clone();
        let keyboard_focus_cycle = self.focus_cycle;

        let mut overlay = div()
            .id(overlay_id)
            .debug_selector(move || overlay_selector.clone())
            .absolute()
            .inset_0()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(chrome.scrim)
            .occlude()
            .track_focus(&focus_handle)
            .tab_group()
            .tab_stop(false)
            .refine_style(&self.style)
            .on_key_down(move |event: &KeyDownEvent, window, cx| {
                match event.keystroke.key.as_str() {
                    "escape" => {
                        dismiss_dialog(&keyboard_state, keyboard_dismiss.as_ref(), window, cx);
                        cx.stop_propagation();
                    }
                    "tab" => {
                        if keyboard_focus_cycle.is_empty() {
                            let focus_handle = keyboard_state.read(cx).focus_handle.clone();
                            focus_handle.focus(window, cx);
                        } else {
                            let current = keyboard_focus_cycle
                                .iter()
                                .position(|focus_handle| focus_handle.is_focused(window));
                            let next = if event.keystroke.modifiers.shift {
                                current
                                    .map(|index| {
                                        (index + keyboard_focus_cycle.len() - 1)
                                            % keyboard_focus_cycle.len()
                                    })
                                    .unwrap_or(keyboard_focus_cycle.len() - 1)
                            } else {
                                current
                                    .map(|index| (index + 1) % keyboard_focus_cycle.len())
                                    .unwrap_or(0)
                            };
                            keyboard_focus_cycle[next].focus(window, cx);
                        }
                        cx.stop_propagation();
                    }
                    _ => {}
                }
            })
            .child(
                div()
                    .id(panel_id)
                    .debug_selector(move || panel_selector.clone())
                    .role(gpui::Role::Dialog)
                    .occlude()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .children(self.children),
            );

        if self.dismiss_on_scrim {
            overlay = overlay.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                dismiss_dialog(&scrim_state, scrim_dismiss.as_ref(), window, cx);
            });
        }

        deferred(
            Motion::new()
                .id(surface_id)
                .surface_in()
                .absolute()
                .inset_0()
                .size_full()
                .child(overlay),
        )
        .with_priority(10)
        .into_any_element()
    }
}

/// Radius-10 dialog panel matching the Paper material.
#[derive(IntoElement)]
pub struct DialogContent {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DialogContent {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for DialogContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DialogContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DialogContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let chrome = dialog_chrome(cx.theme());

        div()
            .flex()
            .flex_col()
            .w(px(400.))
            .flex_shrink_0()
            .p(px(24.))
            .gap(px(16.))
            .rounded(px(10.))
            .border_1()
            .border_color(chrome.border)
            .bg(chrome.background)
            .shadow(vec![
                BoxShadow::new(px(0.), px(1.), chrome.inset).inset(),
                BoxShadow::new(px(0.), px(6.), chrome.shadow).blur_radius(px(16.)),
            ])
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Groups a dialog title and supporting description.
#[derive(IntoElement)]
pub struct DialogHeader {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DialogHeader {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for DialogHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DialogHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DialogHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Dialog heading text.
#[derive(IntoElement)]
pub struct DialogTitle {
    text: SharedString,
    style: StyleRefinement,
}

impl DialogTitle {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            style: StyleRefinement::default(),
        }
    }
}

impl Styled for DialogTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for DialogTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .font_family(theme.font_family)
            .font_weight(FontWeight::SEMIBOLD)
            .text_size(px(16.))
            .line_height(px(22.))
            .text_color(theme.heading)
            .refine_style(&self.style)
            .child(self.text)
    }
}

/// Supporting copy beneath a dialog title.
#[derive(IntoElement)]
pub struct DialogDescription {
    text: SharedString,
    style: StyleRefinement,
}

impl DialogDescription {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            style: StyleRefinement::default(),
        }
    }
}

impl Styled for DialogDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for DialogDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .font_family(theme.font_family)
            .font_weight(FontWeight::NORMAL)
            .text_size(px(15.))
            .line_height(px(24.))
            .text_color(theme.body)
            .refine_style(&self.style)
            .child(self.text)
    }
}

/// Right-aligned dialog action row.
#[derive(IntoElement)]
pub struct DialogFooter {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DialogFooter {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for DialogFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DialogFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DialogFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_end()
            .gap(px(8.))
            .refine_style(&self.style)
            .children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_material_matches_paper() {
        let chrome = dialog_chrome(Theme::light());
        assert_eq!(chrome.background, paint(0xFFFFFF85));
        assert_eq!(chrome.border, paint(0xFFFFFFB8));
        assert_eq!(chrome.inset, paint(0xFFFFFFE6));
        assert_eq!(chrome.shadow, paint(0x0F172A0F));
        assert_eq!(chrome.scrim, paint(0x18181B47));
    }

    #[test]
    fn dark_material_matches_paper() {
        let chrome = dialog_chrome(Theme::dark());
        assert_eq!(chrome.background, paint(0xFFFFFF12));
        assert_eq!(chrome.border, paint(0xFFFFFF1A));
        assert_eq!(chrome.inset, paint(0xFFFFFF1F));
        assert_eq!(chrome.shadow, paint(0x00000047));
        assert_eq!(chrome.scrim, paint(0x00000073));
    }
}
