use std::rc::Rc;

use gpui::{
    anchored, deferred, div, point, prelude::*, px, relative, AnyElement, App, Entity, FocusHandle,
    FontWeight, IntoElement, KeyDownEvent, MouseButton, ParentElement, Pixels, Point, RenderOnce,
    SharedString, StyleRefinement, Styled, Window,
};

use crate::chrome::box_shadow;
use crate::compat::{AccessibilityExt, Role, StyleCompatExt};

use crate::icon::{Icon, IconName};
use crate::kbd::Kbd;
use crate::motion::{Motion, StyledSlot};
use crate::theme::{paint, ActiveTheme, Theme, ThemeKind};

type MenuAction = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
pub(crate) type OpenChangeHandler = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

#[derive(Clone)]
pub enum DropdownMenuEntry {
    Item(DropdownMenuItem),
    Separator,
}

impl DropdownMenuEntry {
    pub fn item(label: impl Into<SharedString>) -> Self {
        Self::Item(DropdownMenuItem::new(label))
    }

    pub fn separator() -> Self {
        Self::Separator
    }
}

#[derive(Clone)]
pub struct DropdownMenuItem {
    label: SharedString,
    disabled: bool,
    destructive: bool,
    shortcut: Option<SharedString>,
    submenu: Vec<DropdownMenuEntry>,
    on_select: Option<MenuAction>,
}

impl DropdownMenuItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            disabled: false,
            destructive: false,
            shortcut: None,
            submenu: Vec::new(),
            on_select: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn submenu(mut self, entries: impl IntoIterator<Item = DropdownMenuEntry>) -> Self {
        self.submenu = entries.into_iter().collect();
        self
    }

    pub fn on_select(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(listener));
        self
    }
}

impl From<DropdownMenuItem> for DropdownMenuEntry {
    fn from(item: DropdownMenuItem) -> Self {
        Self::Item(item)
    }
}

pub(crate) struct DropdownMenuState {
    pub(crate) focus_handle: FocusHandle,
    pub(crate) open: bool,
    pub(crate) highlighted: Option<usize>,
    pub(crate) submenu: Option<SubmenuState>,
    pub(crate) origin: Point<Pixels>,
    pub(crate) origin_window: bool,
    pub(crate) previous_focus: Option<FocusHandle>,
}

#[derive(Clone, Copy)]
pub(crate) struct SubmenuState {
    pub(crate) parent: usize,
    pub(crate) highlighted: Option<usize>,
}

pub(crate) fn item_at(entries: &[DropdownMenuEntry], index: usize) -> Option<&DropdownMenuItem> {
    match entries.get(index) {
        Some(DropdownMenuEntry::Item(item)) => Some(item),
        _ => None,
    }
}

fn enabled(entries: &[DropdownMenuEntry], index: usize) -> bool {
    item_at(entries, index).is_some_and(|item| !item.disabled)
}

pub(crate) fn initial_highlight(entries: &[DropdownMenuEntry]) -> Option<usize> {
    entries
        .iter()
        .enumerate()
        .find_map(|(index, _)| enabled(entries, index).then_some(index))
}

pub(crate) fn next_enabled(
    entries: &[DropdownMenuEntry],
    current: Option<usize>,
    forward: bool,
) -> Option<usize> {
    if entries.is_empty() {
        return None;
    }

    match current {
        Some(start) => (1..=entries.len())
            .map(|offset| {
                if forward {
                    (start + offset) % entries.len()
                } else {
                    (start + entries.len() - (offset % entries.len())) % entries.len()
                }
            })
            .find(|index| enabled(entries, *index)),
        None if forward => initial_highlight(entries),
        None => entries
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, _)| enabled(entries, index).then_some(index)),
    }
}

pub(crate) fn set_open(
    state: &Entity<DropdownMenuState>,
    open: bool,
    entries: &[DropdownMenuEntry],
    on_open_change: Option<&OpenChangeHandler>,
    window: &mut Window,
    cx: &mut App,
) {
    if state.read(cx).open == open {
        return;
    }

    state.update(cx, |menu, cx| {
        menu.open = open;
        menu.highlighted = open.then(|| initial_highlight(entries)).flatten();
        menu.submenu = None;
        cx.notify();
    });

    if let Some(on_open_change) = on_open_change {
        on_open_change(open, window, cx);
    }
    window.refresh();
}

pub(crate) fn activate_item(
    item: &DropdownMenuItem,
    state: &Entity<DropdownMenuState>,
    entries: &[DropdownMenuEntry],
    on_open_change: Option<&OpenChangeHandler>,
    focus_handle: &FocusHandle,
    window: &mut Window,
    cx: &mut App,
) {
    if item.disabled {
        return;
    }

    set_open(state, false, entries, on_open_change, window, cx);
    focus_handle.focus(window);
    if let Some(on_select) = &item.on_select {
        on_select(window, cx);
    }
}

pub(crate) fn handle_menu_keydown(
    event: &KeyDownEvent,
    state: &Entity<DropdownMenuState>,
    entries: &[DropdownMenuEntry],
    on_open_change: Option<&OpenChangeHandler>,
    focus_handle: &FocusHandle,
    window: &mut Window,
    cx: &mut App,
) {
    if event.keystroke.modifiers.modified() {
        return;
    }

    let key = event.keystroke.key.as_str();
    let snapshot = state.read(cx);
    let was_open = snapshot.open;
    let root_highlighted = snapshot.highlighted;
    let submenu_state = snapshot.submenu;

    match key {
        "down" | "up" => {
            let forward = key == "down";
            state.update(cx, |menu, cx| {
                if !menu.open {
                    menu.open = true;
                    menu.highlighted = initial_highlight(entries);
                } else if let Some(submenu) = &mut menu.submenu {
                    if let Some(parent) = item_at(entries, submenu.parent) {
                        submenu.highlighted =
                            next_enabled(&parent.submenu, submenu.highlighted, forward);
                    }
                } else {
                    menu.highlighted = next_enabled(entries, menu.highlighted, forward);
                }
                cx.notify();
            });
            if !was_open {
                if let Some(on_open_change) = on_open_change {
                    on_open_change(true, window, cx);
                }
            }
            cx.stop_propagation();
        }
        "enter" | "space" if !was_open => {
            set_open(state, true, entries, on_open_change, window, cx);
            cx.stop_propagation();
        }
        "right" if was_open && submenu_state.is_none() => {
            if let Some((parent, item)) = root_highlighted
                .and_then(|index| item_at(entries, index).map(|item| (index, item)))
                .filter(|(_, item)| !item.disabled && !item.submenu.is_empty())
            {
                state.update(cx, |menu, cx| {
                    menu.submenu = Some(SubmenuState {
                        parent,
                        highlighted: initial_highlight(&item.submenu),
                    });
                    cx.notify();
                });
                cx.stop_propagation();
            }
        }
        "left" if submenu_state.is_some() => {
            state.update(cx, |menu, cx| {
                menu.submenu = None;
                cx.notify();
            });
            cx.stop_propagation();
        }
        "enter" | "space" if was_open => {
            let picked = if let Some(submenu) = submenu_state {
                item_at(entries, submenu.parent).and_then(|parent| {
                    submenu
                        .highlighted
                        .and_then(|index| item_at(&parent.submenu, index))
                })
            } else {
                root_highlighted.and_then(|index| item_at(entries, index))
            };

            if let Some(item) = picked.filter(|item| !item.disabled) {
                if !item.submenu.is_empty() && submenu_state.is_none() {
                    let parent = root_highlighted.expect("highlighted root item");
                    state.update(cx, |menu, cx| {
                        menu.submenu = Some(SubmenuState {
                            parent,
                            highlighted: initial_highlight(&item.submenu),
                        });
                        cx.notify();
                    });
                    window.refresh();
                } else {
                    activate_item(
                        item,
                        state,
                        entries,
                        on_open_change,
                        focus_handle,
                        window,
                        cx,
                    );
                }
            }
            cx.stop_propagation();
        }
        "escape" if was_open => {
            set_open(state, false, entries, on_open_change, window, cx);
            focus_handle.focus(window);
            cx.stop_propagation();
        }
        _ => {}
    }
}

#[derive(Clone, Copy)]
struct MenuChrome {
    background: gpui::Hsla,
    border: gpui::Hsla,
    inset: gpui::Hsla,
    shadow: gpui::Hsla,
    highlight: gpui::Hsla,
}

fn menu_chrome(theme: Theme) -> MenuChrome {
    match theme.kind {
        ThemeKind::Light => MenuChrome {
            background: paint(0xFFFFFF85),
            border: paint(0xFFFFFFB8),
            inset: paint(0xFFFFFFE6),
            shadow: paint(0x0F172A0F),
            highlight: paint(0xFFFFFF47),
        },
        ThemeKind::Dark => MenuChrome {
            background: paint(0xFFFFFF12),
            border: paint(0xFFFFFF1A),
            inset: paint(0xFFFFFF1F),
            shadow: paint(0x00000047),
            highlight: paint(0xFFFFFF12),
        },
    }
}

#[derive(Clone)]
pub(crate) struct MenuPanelContext {
    pub(crate) state: Entity<DropdownMenuState>,
    pub(crate) root_entries: Vec<DropdownMenuEntry>,
    pub(crate) focus_handle: FocusHandle,
    pub(crate) on_open_change: Option<OpenChangeHandler>,
}

pub(crate) fn render_panel(
    id: SharedString,
    entries: Vec<DropdownMenuEntry>,
    highlighted: Option<usize>,
    submenu_parent: Option<usize>,
    context: MenuPanelContext,
    outside_dismiss: bool,
    cx: &mut App,
) -> AnyElement {
    let theme = cx.theme();
    let chrome = menu_chrome(theme);
    let panel_selector = format!("{id}-panel");
    let surface_id = format!("{id}-surface");
    let dismiss_state = context.state.clone();
    let dismiss_entries = context.root_entries.clone();
    let dismiss_change = context.on_open_change.clone();

    let panel = div()
        .id(SharedString::from(panel_selector.clone()))
        .debug_selector(move || panel_selector.clone())
        .role(Role::Menu)
        .flex()
        .flex_col()
        .w(px(240.))
        .flex_shrink_0()
        .p(px(4.))
        .gap(px(2.))
        .rounded(px(6.))
        .border_1()
        .border_color(chrome.border)
        .bg(chrome.background)
        .shadow(vec![
            box_shadow(0., 1., chrome.inset, 0., 0.),
            box_shadow(0., 6., chrome.shadow, 16., 0.),
        ])
        .occlude()
        .when(outside_dismiss, |panel| {
            panel.on_mouse_down_out(move |_, window, cx| {
                set_open(
                    &dismiss_state,
                    false,
                    &dismiss_entries,
                    dismiss_change.as_ref(),
                    window,
                    cx,
                );
            })
        })
        .children(entries.into_iter().enumerate().map(|(index, entry)| {
            let DropdownMenuEntry::Item(item) = entry else {
                return div()
                    .id(SharedString::from(format!("{id}-separator-{index}")))
                    .debug_selector({
                        let selector = format!("{id}-separator-{index}");
                        move || selector.clone()
                    })
                    .w(px(232.))
                    .h(px(1.))
                    .flex_shrink_0()
                    .bg(if theme.is_dark() {
                        paint(0xFAFAFA1F)
                    } else {
                        paint(0x18181B1F)
                    })
                    .into_any_element();
            };

            let row_selector = format!("{id}-item-{index}");
            let is_highlighted = highlighted == Some(index);
            let is_submenu_open = submenu_parent == Some(index);
            let has_submenu = !item.submenu.is_empty();
            let interactive = !item.disabled;
            let text_color = if item.disabled {
                theme.label
            } else if item.destructive {
                theme.destructive
            } else {
                theme.ink
            };
            let click_item = item.clone();
            let click_state = context.state.clone();
            let click_entries = context.root_entries.clone();
            let click_focus = context.focus_handle.clone();
            let click_change = context.on_open_change.clone();
            let submenu_entries = item.submenu.clone();

            let mut row = div()
                .id(SharedString::from(row_selector.clone()))
                .debug_selector(move || row_selector.clone())
                .role(Role::MenuItem)
                .aria_selected(is_highlighted)
                .when(has_submenu, |row| row.aria_expanded(is_submenu_open))
                .relative()
                .flex()
                .items_center()
                .justify_between()
                .h(px(32.))
                .flex_shrink_0()
                .px(px(10.))
                .rounded(px(4.))
                .when(is_highlighted, |row| row.bg(chrome.highlight))
                .when(interactive, |row| {
                    row.cursor_pointer()
                        .hover(move |style| style.bg(chrome.highlight))
                })
                .when(!interactive, |row| row.cursor_default())
                .child(
                    div()
                        .min_w(px(0.))
                        .font_family(theme.font_family)
                        .font_weight(FontWeight::NORMAL)
                        .text_size(px(14.))
                        .line_height(px(18.))
                        .text_color(text_color)
                        .child(item.label.clone()),
                )
                .when_some(item.shortcut.clone(), |row, shortcut| {
                    row.child(Kbd::new(shortcut))
                })
                .when(has_submenu, |row| {
                    row.child(
                        Icon::new(IconName::ChevronRight)
                            .px(px(16.))
                            .color(theme.label),
                    )
                });

            if interactive {
                row = row.on_click(move |_, window, cx| {
                    if has_submenu {
                        click_state.update(cx, |menu, cx| {
                            menu.highlighted = Some(index);
                            menu.submenu = Some(SubmenuState {
                                parent: index,
                                highlighted: initial_highlight(&submenu_entries),
                            });
                            cx.notify();
                        });
                        window.refresh();
                    } else {
                        activate_item(
                            &click_item,
                            &click_state,
                            &click_entries,
                            click_change.as_ref(),
                            &click_focus,
                            window,
                            cx,
                        );
                    }
                });
            }

            if is_submenu_open {
                let submenu = item.submenu.clone();
                let submenu_highlighted = context
                    .state
                    .read(cx)
                    .submenu
                    .and_then(|menu| menu.highlighted);
                let submenu_panel = render_panel(
                    SharedString::from(format!("{id}-submenu-{index}")),
                    submenu,
                    submenu_highlighted,
                    None,
                    context.clone(),
                    true,
                    cx,
                );
                row = row.child(
                    div().absolute().left(relative(1.)).top(px(-4.)).child(
                        anchored()
                            .offset(point(px(6.), px(0.)))
                            .snap_to_window_with_margin(px(8.))
                            .child(submenu_panel),
                    ),
                );
            }

            row.into_any_element()
        }));

    Motion::new()
        .id(surface_id)
        .surface_in()
        .child(panel)
        .into_any_element()
}

/// Trigger-anchored menu with pointer, keyboard, focus, and one-level submenu behavior.
#[derive(IntoElement)]
pub struct DropdownMenu {
    id: SharedString,
    controlled_open: Option<bool>,
    default_open: bool,
    trigger_label: SharedString,
    trigger: Option<AnyElement>,
    entries: Vec<DropdownMenuEntry>,
    on_open_change: Option<OpenChangeHandler>,
    style: StyleRefinement,
}

impl DropdownMenu {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            controlled_open: None,
            default_open: false,
            trigger_label: SharedString::from("Open menu"),
            trigger: None,
            entries: Vec::new(),
            on_open_change: None,
            style: StyleRefinement::default(),
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

    pub fn trigger_label(mut self, label: impl Into<SharedString>) -> Self {
        self.trigger_label = label.into();
        self
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn entries(mut self, entries: impl IntoIterator<Item = DropdownMenuEntry>) -> Self {
        self.entries = entries.into_iter().collect();
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

impl Styled for DropdownMenu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for DropdownMenu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let has_trigger = self.trigger.is_some();
        let initial_open = self.controlled_open.unwrap_or(self.default_open);
        let initial_entries = self.entries.clone();
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, cx| DropdownMenuState {
            focus_handle: cx.focus_handle().tab_stop(true),
            open: initial_open,
            highlighted: initial_open
                .then(|| initial_highlight(&initial_entries))
                .flatten(),
            submenu: None,
            origin: point(px(0.), px(0.)),
            origin_window: false,
            previous_focus: None,
        });

        if let Some(controlled_open) = self.controlled_open {
            if state.read(cx).open != controlled_open {
                state.update(cx, |menu, _| {
                    menu.open = controlled_open;
                    menu.highlighted = controlled_open
                        .then(|| initial_highlight(&self.entries))
                        .flatten();
                    menu.submenu = None;
                });
            }
        }

        let open = state.read(cx).open;
        let highlighted = state.read(cx).highlighted;
        let submenu = state.read(cx).submenu;
        let focus_handle = state.read(cx).focus_handle.clone();
        let trigger_selector = format!("{}-trigger", self.id);
        let trigger_state = state.clone();
        let trigger_entries = self.entries.clone();
        let trigger_change = self.on_open_change.clone();
        let trigger_focus = focus_handle.clone();
        let keyboard_state = state.clone();
        let keyboard_entries = self.entries.clone();
        let keyboard_change = self.on_open_change.clone();
        let keyboard_focus = focus_handle.clone();

        let trigger = div()
            .id(SharedString::from(trigger_selector.clone()))
            .debug_selector(move || trigger_selector.clone())
            .role(Role::Button)
            .aria_label(self.trigger_label)
            .aria_expanded(open)
            .track_focus(&focus_handle)
            .tab_stop(true)
            .relative()
            .flex()
            .items_center()
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                trigger_focus.focus(window);
                let next_open = !trigger_state.read(cx).open;
                set_open(
                    &trigger_state,
                    next_open,
                    &trigger_entries,
                    trigger_change.as_ref(),
                    window,
                    cx,
                );
            })
            .on_key_down(move |event: &KeyDownEvent, window, cx| {
                handle_menu_keydown(
                    event,
                    &keyboard_state,
                    &keyboard_entries,
                    keyboard_change.as_ref(),
                    &keyboard_focus,
                    window,
                    cx,
                );
            })
            .when_some(self.trigger, |trigger, content| trigger.child(content));

        let panel_context = MenuPanelContext {
            state,
            root_entries: self.entries.clone(),
            focus_handle,
            on_open_change: self.on_open_change,
        };
        let panel = render_panel(
            self.id.clone(),
            self.entries.clone(),
            highlighted,
            submenu.map(|submenu| submenu.parent),
            panel_context,
            submenu.is_none(),
            cx,
        );
        let content = if has_trigger {
            div()
                .absolute()
                .left(px(0.))
                .top(relative(1.))
                .child(
                    deferred(
                        anchored()
                            .offset(point(px(0.), px(6.)))
                            .snap_to_window_with_margin(px(8.))
                            .child(panel),
                    )
                    .with_priority(2),
                )
                .into_any_element()
        } else {
            panel
        };

        div()
            .relative()
            .flex()
            .flex_none()
            .self_start()
            .items_center()
            .refine_style(&self.style)
            .when(has_trigger, |menu| menu.child(trigger))
            .when(open, |menu| menu.child(content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn materials_match_paper() {
        let light = menu_chrome(Theme::light());
        assert_eq!(light.background, paint(0xFFFFFF85));
        assert_eq!(light.border, paint(0xFFFFFFB8));
        assert_eq!(light.highlight, paint(0xFFFFFF47));

        let dark = menu_chrome(Theme::dark());
        assert_eq!(dark.background, paint(0xFFFFFF12));
        assert_eq!(dark.border, paint(0xFFFFFF1A));
        assert_eq!(dark.highlight, paint(0xFFFFFF12));
    }

    #[test]
    fn navigation_skips_separators_and_disabled_items() {
        let entries = vec![
            DropdownMenuEntry::separator(),
            DropdownMenuItem::new("Disabled").disabled(true).into(),
            DropdownMenuItem::new("Ready").into(),
        ];
        assert_eq!(initial_highlight(&entries), Some(2));
        assert_eq!(next_enabled(&entries, Some(2), true), Some(2));
    }
}
