//! Design-spec hex+alpha is grouped as `RRGGBB_AA`.
#![allow(clippy::unusual_byte_groupings)]

use std::rc::Rc;

use gpui::{
    actions, div, prelude::*, px, App, Context, FocusHandle, FontWeight, KeyBinding, Render,
    RenderOnce, SharedString, StyleRefinement, Styled, Window,
};

use crate::chrome::{box_shadow, focus_ring};
use crate::compat::{AccessibilityExt, Role};
use crate::icon::IconName;
use crate::input::Input;
use crate::kbd::Kbd;
use crate::motion::StyledSlot;
use crate::spinner::{Spinner, SpinnerSize};
use crate::theme::{paint, ActiveTheme, Theme, ThemeKind};

type CommandSelectHandler = Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>;
type CommandQueryHandler = Rc<dyn Fn(SharedString, &mut Window, &mut App) + 'static>;
type CommandDismissHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

actions!(
    glassy_command,
    [CommandNext, CommandPrevious, CommandAccept, CommandDismiss]
);

pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("down", CommandNext, Some("KitCommandInput")),
        KeyBinding::new("up", CommandPrevious, Some("KitCommandInput")),
        KeyBinding::new("enter", CommandAccept, Some("KitCommandInput")),
        KeyBinding::new("escape", CommandDismiss, Some("KitCommandInput")),
    ]);
}

/// Width and internal spacing used by the full palette and compact state specimens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CommandSize {
    #[default]
    Default,
    Compact,
}

/// One selectable command row.
#[derive(Clone)]
pub struct CommandItem {
    id: SharedString,
    label: SharedString,
    keywords: Vec<SharedString>,
    shortcut: Option<SharedString>,
    disabled: bool,
    on_select: Option<CommandSelectHandler>,
}

impl CommandItem {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            keywords: Vec::new(),
            shortcut: None,
            disabled: false,
            on_select: None,
        }
    }

    pub fn keywords(mut self, keywords: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.keywords = keywords.into_iter().map(Into::into).collect();
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_select(
        mut self,
        listener: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(listener));
        self
    }

    fn matches(&self, normalized_query: &str) -> bool {
        normalized_query.is_empty()
            || self.label.to_lowercase().contains(normalized_query)
            || self
                .keywords
                .iter()
                .any(|keyword| keyword.to_lowercase().contains(normalized_query))
    }
}

/// A labeled collection of command rows.
#[derive(Clone)]
pub struct CommandGroup {
    label: SharedString,
    items: Vec<CommandItem>,
}

impl CommandGroup {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            items: Vec::new(),
        }
    }

    pub fn items(mut self, items: impl IntoIterator<Item = CommandItem>) -> Self {
        self.items = items.into_iter().collect();
        self
    }

    pub fn item(mut self, item: CommandItem) -> Self {
        self.items.push(item);
        self
    }
}

struct CommandState {
    id: SharedString,
    focus_handle: FocusHandle,
    query: SharedString,
    highlighted_id: Option<SharedString>,
    groups: Vec<CommandGroup>,
    placeholder: SharedString,
    empty_label: SharedString,
    loading_label: SharedString,
    footer: Option<SharedString>,
    filtering: bool,
    loading: bool,
    size: CommandSize,
    on_query_change: Option<CommandQueryHandler>,
    on_select: Option<CommandSelectHandler>,
    on_dismiss: Option<CommandDismissHandler>,
    style: StyleRefinement,
}

#[derive(Clone, Copy)]
struct CommandChrome {
    surface: gpui::Hsla,
    border: gpui::Hsla,
    inset: gpui::Hsla,
    shadow: gpui::Hsla,
    selected: gpui::Hsla,
}

fn command_chrome(theme: Theme) -> CommandChrome {
    match theme.kind {
        ThemeKind::Light => {
            // Resolve the design-source glass against the opaque kit canvas.
            // GPUI 0.2.2 composites translucent element fills differently from
            // the design source, so painting the exact source alpha directly
            // is visibly too bright in the native renderer.
            let surface = theme.canvas.blend(paint(0xFFFFFF_85));
            CommandChrome {
                surface,
                border: paint(0xFFFFFF_B8),
                inset: paint(0xFFFFFF_E6),
                shadow: paint(0x0F172A_0F),
                selected: surface.blend(paint(0xFFFFFF_47)),
            }
        }
        ThemeKind::Dark => {
            let surface = theme.canvas.blend(paint(0xFFFFFF_12));
            CommandChrome {
                surface,
                border: paint(0xFFFFFF_1A),
                inset: paint(0xFFFFFF_1F),
                shadow: paint(0x000000_47),
                selected: surface.blend(paint(0xFFFFFF_12)),
            }
        }
    }
}

fn visible_items(groups: &[CommandGroup], query: &str, filtering: bool) -> Vec<CommandItem> {
    let normalized_query = query.trim().to_lowercase();
    groups
        .iter()
        .flat_map(|group| group.items.iter())
        .filter(|item| !filtering || item.matches(&normalized_query))
        .cloned()
        .collect()
}

fn first_enabled(items: &[CommandItem]) -> Option<SharedString> {
    items
        .iter()
        .find(|item| !item.disabled)
        .map(|item| item.id.clone())
}

fn next_enabled(
    items: &[CommandItem],
    highlighted_id: Option<&SharedString>,
    forward: bool,
) -> Option<SharedString> {
    if items.is_empty() {
        return None;
    }

    let current = highlighted_id.and_then(|id| items.iter().position(|item| item.id == *id));
    match current {
        Some(start) => (1..=items.len())
            .map(|offset| {
                if forward {
                    (start + offset) % items.len()
                } else {
                    (start + items.len() - (offset % items.len())) % items.len()
                }
            })
            .find(|index| !items[*index].disabled)
            .map(|index| items[index].id.clone()),
        None if forward => first_enabled(items),
        None => items
            .iter()
            .rfind(|item| !item.disabled)
            .map(|item| item.id.clone()),
    }
}

fn activate_item(
    item: &CommandItem,
    on_select: Option<&CommandSelectHandler>,
    window: &mut Window,
    cx: &mut App,
) {
    if item.disabled {
        return;
    }
    if let Some(item_handler) = item.on_select.as_ref() {
        item_handler(&item.id, window, cx);
    }
    if let Some(on_select) = on_select {
        on_select(&item.id, window, cx);
    }
}

fn move_highlight(
    state: &gpui::Entity<CommandState>,
    items: &[CommandItem],
    forward: bool,
    cx: &mut App,
) {
    if items.is_empty() {
        return;
    }
    let current = state.read(cx).highlighted_id.clone();
    let next = next_enabled(items, current.as_ref(), forward);
    state.update(cx, |command, cx| {
        command.highlighted_id = next;
        cx.notify();
    });
}

fn accept_highlighted(
    state: &gpui::Entity<CommandState>,
    items: &[CommandItem],
    on_select: Option<&CommandSelectHandler>,
    window: &mut Window,
    cx: &mut App,
) {
    let highlighted = state.read(cx).highlighted_id.clone();
    if let Some(item) = highlighted
        .as_ref()
        .and_then(|id| items.iter().find(|item| item.id == *id))
    {
        activate_item(item, on_select, window, cx);
    }
}

/// Searchable grouped command sheet with pointer and keyboard navigation.
#[derive(IntoElement)]
pub struct Command {
    id: SharedString,
    groups: Vec<CommandGroup>,
    default_query: SharedString,
    placeholder: SharedString,
    empty_label: SharedString,
    loading_label: SharedString,
    footer: Option<SharedString>,
    filtering: bool,
    loading: bool,
    size: CommandSize,
    focus_handle: Option<FocusHandle>,
    on_query_change: Option<CommandQueryHandler>,
    on_select: Option<CommandSelectHandler>,
    on_dismiss: Option<CommandDismissHandler>,
    style: StyleRefinement,
}

impl Command {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            groups: Vec::new(),
            default_query: SharedString::default(),
            placeholder: SharedString::from("Search…"),
            empty_label: SharedString::from("No results match."),
            loading_label: SharedString::from("Searching"),
            footer: Some(SharedString::from("↑↓ to move · ↵ to open")),
            filtering: true,
            loading: false,
            size: CommandSize::Default,
            focus_handle: None,
            on_query_change: None,
            on_select: None,
            on_dismiss: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn groups(mut self, groups: impl IntoIterator<Item = CommandGroup>) -> Self {
        self.groups = groups.into_iter().collect();
        self
    }

    pub fn group(mut self, group: CommandGroup) -> Self {
        self.groups.push(group);
        self
    }

    pub fn default_query(mut self, query: impl Into<SharedString>) -> Self {
        self.default_query = query.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn empty_label(mut self, label: impl Into<SharedString>) -> Self {
        self.empty_label = label.into();
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn loading_label(mut self, label: impl Into<SharedString>) -> Self {
        self.loading_label = label.into();
        self
    }

    pub fn footer(mut self, footer: impl Into<SharedString>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    pub fn show_footer(mut self, show: bool) -> Self {
        if !show {
            self.footer = None;
        }
        self
    }

    /// Disable local filtering when results are supplied by an external search.
    pub fn filtering(mut self, filtering: bool) -> Self {
        self.filtering = filtering;
        self
    }

    pub fn size(mut self, size: CommandSize) -> Self {
        self.size = size;
        self
    }

    /// Share a focus handle with a modal host for deterministic initial focus.
    pub fn focus_handle(mut self, focus_handle: FocusHandle) -> Self {
        self.focus_handle = Some(focus_handle);
        self
    }

    pub fn on_query_change(
        mut self,
        listener: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_query_change = Some(Rc::new(listener));
        self
    }

    pub fn on_select(
        mut self,
        listener: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(listener));
        self
    }

    pub fn on_dismiss(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(listener));
        self
    }
}

impl Styled for Command {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Command {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state_id = SharedString::from(format!("{}-state", self.id));
        let state = window.use_keyed_state(state_id, cx, |_, cx| {
            let visible = visible_items(&self.groups, &self.default_query, self.filtering);
            CommandState {
                id: self.id.clone(),
                focus_handle: self
                    .focus_handle
                    .clone()
                    .unwrap_or_else(|| cx.focus_handle().tab_stop(true)),
                query: self.default_query.clone(),
                highlighted_id: first_enabled(&visible),
                groups: self.groups.clone(),
                placeholder: self.placeholder.clone(),
                empty_label: self.empty_label.clone(),
                loading_label: self.loading_label.clone(),
                footer: self.footer.clone(),
                filtering: self.filtering,
                loading: self.loading,
                size: self.size,
                on_query_change: self.on_query_change.clone(),
                on_select: self.on_select.clone(),
                on_dismiss: self.on_dismiss.clone(),
                style: self.style.clone(),
            }
        });
        state.update(cx, |command, _| {
            command.id = self.id;
            command.groups = self.groups;
            command.placeholder = self.placeholder;
            command.empty_label = self.empty_label;
            command.loading_label = self.loading_label;
            command.footer = self.footer;
            command.filtering = self.filtering;
            command.loading = self.loading;
            command.size = self.size;
            command.on_query_change = self.on_query_change;
            command.on_select = self.on_select;
            command.on_dismiss = self.on_dismiss;
            command.style = self.style;
            if let Some(focus_handle) = self.focus_handle {
                command.focus_handle = focus_handle;
            }
        });
        state
    }
}

impl Render for CommandState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = cx.entity();

        let query = self.query.clone();
        let mut items = visible_items(&self.groups, &query, self.filtering);
        let current_highlight = self.highlighted_id.clone();
        let valid_highlight = current_highlight
            .as_ref()
            .is_some_and(|id| items.iter().any(|item| item.id == *id && !item.disabled));
        if !valid_highlight {
            self.highlighted_id = first_enabled(&items);
        }
        if self.loading {
            items.clear();
        }

        let highlighted_id = self.highlighted_id.clone();
        let focus_handle = self.focus_handle.clone();
        let theme = cx.theme();
        let chrome = command_chrome(theme);
        let width = match self.size {
            CommandSize::Default => 400.0,
            CommandSize::Compact => 280.0,
        };
        let padding = match self.size {
            CommandSize::Default => 8.0,
            CommandSize::Compact => 16.0,
        };
        let compact = self.size == CommandSize::Compact;
        let root_selector = self.id.to_string();
        let search_id = SharedString::from(format!("{}-search", self.id));
        let query_state = state.clone();
        let query_groups = self.groups.clone();
        let query_filtering = self.filtering;
        let on_query_change = self.on_query_change.clone();

        let search = Input::new(search_id)
            .placeholder(self.placeholder.clone())
            .value(query)
            .leading_icon(IconName::Search)
            .focus_handle(focus_handle.clone())
            .content_padding_x(px(10.))
            .composite_base(chrome.surface)
            .key_context("KitCommandInput")
            .w_full()
            .on_change(move |query, window, cx| {
                let visible = visible_items(&query_groups, &query, query_filtering);
                query_state.update(cx, |command, cx| {
                    command.query = query.clone();
                    command.highlighted_id = first_enabled(&visible);
                    cx.notify();
                });
                cx.refresh_windows();
                if let Some(on_query_change) = on_query_change.as_ref() {
                    on_query_change(query, window, cx);
                }
            });

        let on_select = self.on_select.clone();
        let groups = self.groups.clone();
        let normalized_query = self.query.trim().to_lowercase();
        let filtering = self.filtering;
        let mut content = div().flex().flex_col();

        if self.loading {
            content = content.child(
                div()
                    .debug_selector(|| "COMMAND_LOADING".into())
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(8.))
                    .py(px(12.))
                    .child(Spinner::new().size(SpinnerSize::Small))
                    .child(
                        div()
                            .font_family(theme.font_family)
                            .font_weight(FontWeight::MEDIUM)
                            .text_size(px(14.))
                            .line_height(px(18.))
                            .text_color(theme.ink)
                            .child(self.loading_label.clone()),
                    ),
            );
        } else if items.is_empty() {
            content = content.child(
                div()
                    .debug_selector(|| "COMMAND_EMPTY".into())
                    .flex()
                    .items_center()
                    .justify_center()
                    .py(px(16.))
                    .font_family(theme.font_family)
                    .font_weight(FontWeight::NORMAL)
                    .text_size(px(14.))
                    .line_height(px(18.))
                    .text_color(theme.body)
                    .child(self.empty_label.clone()),
            );
        } else {
            for (group_index, group) in groups.into_iter().enumerate() {
                let visible_group_items = group
                    .items
                    .into_iter()
                    .filter(|item| !filtering || item.matches(&normalized_query))
                    .collect::<Vec<_>>();
                if visible_group_items.is_empty() {
                    continue;
                }

                let mut group_element = div()
                    .id(SharedString::from(format!(
                        "{}-group-{group_index}",
                        self.id
                    )))
                    .flex()
                    .flex_col()
                    .flex_shrink_0()
                    .pt(px(8.))
                    .gap(px(2.))
                    .child(
                        div()
                            .px(px(10.))
                            .font_family(theme.font_family)
                            .font_weight(FontWeight::MEDIUM)
                            .text_size(px(13.))
                            .line_height(px(16.))
                            .text_color(theme.label)
                            .child(group.label),
                    );

                for item in visible_group_items {
                    let is_highlighted = highlighted_id.as_ref().is_some_and(|id| *id == item.id);
                    let enabled = !item.disabled;
                    let row_selector = format!("{}-item-{}", self.id, item.id);
                    let row_id = SharedString::from(row_selector.clone());
                    let hover_color = chrome.selected;
                    let row_state = state.clone();
                    let row_item = item.clone();
                    let row_on_select = on_select.clone();
                    let mut row = div()
                        .id(row_id)
                        .debug_selector(move || row_selector.clone())
                        .role(Role::Option)
                        .aria_selected(is_highlighted)
                        .flex()
                        .items_center()
                        .justify_between()
                        .h(px(32.))
                        .flex_shrink_0()
                        .px(px(10.))
                        .rounded(px(4.))
                        .when(is_highlighted, |el| el.bg(chrome.selected))
                        .when(enabled, |el| {
                            el.cursor_pointer()
                                .hover(move |style| style.bg(hover_color))
                        })
                        .when(!enabled, |el| el.cursor_default())
                        .child(
                            div()
                                .min_w(px(0.))
                                .font_family(theme.font_family)
                                .font_weight(FontWeight::NORMAL)
                                .text_size(px(14.))
                                .line_height(px(18.))
                                .text_color(if enabled { theme.ink } else { theme.label })
                                .child(item.label.clone()),
                        )
                        .when_some(item.shortcut.clone(), |el, shortcut| {
                            el.child(Kbd::new(shortcut))
                        });

                    if enabled {
                        row = row.on_click(move |_, window, cx| {
                            row_state.update(cx, |command, cx| {
                                command.highlighted_id = Some(row_item.id.clone());
                                cx.notify();
                            });
                            activate_item(&row_item, row_on_select.as_ref(), window, cx);
                        });
                    }
                    group_element = group_element.child(row);
                }
                content = content.child(group_element);
            }
        }

        let next_state = state.clone();
        let next_items = items.clone();
        let previous_state = state.clone();
        let previous_items = items.clone();
        let accept_state = state.clone();
        let accept_items = items;
        let accept_on_select = self.on_select.clone();
        let dismiss_handler = self.on_dismiss.clone();
        let mut shadows = vec![box_shadow(0., 1., chrome.inset, 0., 0.)];
        shadows.push(box_shadow(0., 6., chrome.shadow, 16., 0.));
        if focus_handle.is_focused(window) {
            shadows.push(focus_ring(theme));
        }

        div()
            .id(self.id.clone())
            .debug_selector(move || root_selector.clone())
            .role(Role::ListBox)
            .track_focus(&focus_handle)
            .flex()
            .flex_col()
            .flex_shrink_0()
            .w(px(width))
            .p(px(padding))
            .when(compact, |el| el.gap(px(12.)))
            .rounded(px(10.))
            .border_1()
            .border_color(chrome.border)
            .bg(chrome.surface)
            .shadow(shadows)
            .occlude()
            .refine_style(&self.style)
            .on_action(move |_: &CommandNext, window, cx| {
                move_highlight(&next_state, &next_items, true, cx);
                window.refresh();
                cx.stop_propagation();
            })
            .on_action(move |_: &CommandPrevious, window, cx| {
                move_highlight(&previous_state, &previous_items, false, cx);
                window.refresh();
                cx.stop_propagation();
            })
            .on_action(move |_: &CommandAccept, window, cx| {
                accept_highlighted(
                    &accept_state,
                    &accept_items,
                    accept_on_select.as_ref(),
                    window,
                    cx,
                );
                cx.stop_propagation();
            })
            .on_action(move |_: &CommandDismiss, window, cx| {
                if let Some(dismiss_handler) = dismiss_handler.as_ref() {
                    dismiss_handler(window, cx);
                }
                cx.stop_propagation();
            })
            .child(search)
            .child(content)
            .when(!compact, |el| {
                el.when_some(self.footer.clone(), |el, footer| {
                    el.child(
                        div()
                            .pt(px(8.))
                            .pb(px(4.))
                            .px(px(10.))
                            .font_family(theme.font_family)
                            .font_weight(FontWeight::MEDIUM)
                            .text_size(px(12.))
                            .line_height(px(16.))
                            .text_color(theme.label)
                            .child(footer),
                    )
                })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn groups() -> Vec<CommandGroup> {
        vec![CommandGroup::new("Pages").items([
            CommandItem::new("home", "Home"),
            CommandItem::new("buttons", "Buttons").keywords(["controls"]),
            CommandItem::new("disabled", "Disabled").disabled(true),
        ])]
    }

    #[test]
    fn filtering_matches_labels_and_keywords() {
        assert_eq!(visible_items(&groups(), "but", true).len(), 1);
        assert_eq!(visible_items(&groups(), "controls", true).len(), 1);
        assert_eq!(visible_items(&groups(), "missing", true).len(), 0);
        assert_eq!(visible_items(&groups(), "missing", false).len(), 3);
    }

    #[test]
    fn navigation_wraps_and_skips_disabled_items() {
        let items = visible_items(&groups(), "", true);
        assert_eq!(
            next_enabled(&items, Some(&items[0].id), true)
                .as_ref()
                .map(AsRef::as_ref),
            Some("buttons")
        );
        assert_eq!(
            next_enabled(&items, Some(&items[1].id), true)
                .as_ref()
                .map(AsRef::as_ref),
            Some("home")
        );
        assert_eq!(
            next_enabled(&items, Some(&items[0].id), false)
                .as_ref()
                .map(AsRef::as_ref),
            Some("buttons")
        );
    }

    #[test]
    fn surface_resolves_the_exact_source_alpha_against_the_canvas() {
        fn assert_raster_color(actual: gpui::Hsla, expected: gpui::Hsla) {
            let actual = actual.to_rgb();
            let expected = expected.to_rgb();
            let channel_tolerance = 1.0 / 255.0;

            assert!((actual.r - expected.r).abs() <= channel_tolerance);
            assert!((actual.g - expected.g).abs() <= channel_tolerance);
            assert!((actual.b - expected.b).abs() <= channel_tolerance);
            assert_eq!(actual.a, 1.0);
        }

        assert_raster_color(
            command_chrome(Theme::light()).surface,
            crate::theme::rgb(0xF4F5F7),
        );
        assert_raster_color(
            command_chrome(Theme::dark()).surface,
            crate::theme::rgb(0x1C1D20),
        );
    }
}
