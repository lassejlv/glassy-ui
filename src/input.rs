use std::ops::Range;
use std::rc::Rc;

use crate::motion::StyledSlot;
use crate::theme::ActiveTheme;
use gpui::{
    actions, div, fill, point, prelude::*, px, relative, size, App, Bounds, BoxShadow,
    ClipboardItem, CursorStyle, Element, ElementId, ElementInputHandler, Entity,
    EntityInputHandler, FocusHandle, Focusable, FontWeight, GlobalElementId, Hsla, IntoElement,
    KeyBinding, LayoutId, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, PaintQuad,
    Pixels, Point, RenderOnce, Role, ShapedLine, SharedString, Style, StyleRefinement, Styled,
    TextRun, UTF16Selection, UnderlineStyle, Window,
};

use crate::chrome::{field_chrome, FieldState};
use crate::icon::{Icon, IconName};

type InputChangeHandler = Rc<dyn Fn(SharedString, &mut Window, &mut App) + 'static>;

actions!(
    glassy_input,
    [
        Backspace,
        Delete,
        Left,
        Right,
        SelectLeft,
        SelectRight,
        SelectAll,
        Home,
        End,
        Paste,
        Cut,
        Copy,
        Newline,
    ]
);

/// Bind field keys. Call once at startup.
pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("backspace", Backspace, Some("KitInput")),
        KeyBinding::new("delete", Delete, Some("KitInput")),
        KeyBinding::new("left", Left, Some("KitInput")),
        KeyBinding::new("right", Right, Some("KitInput")),
        KeyBinding::new("shift-left", SelectLeft, Some("KitInput")),
        KeyBinding::new("shift-right", SelectRight, Some("KitInput")),
        KeyBinding::new("cmd-a", SelectAll, Some("KitInput")),
        KeyBinding::new("ctrl-a", SelectAll, Some("KitInput")),
        KeyBinding::new("cmd-v", Paste, Some("KitInput")),
        KeyBinding::new("ctrl-v", Paste, Some("KitInput")),
        KeyBinding::new("cmd-c", Copy, Some("KitInput")),
        KeyBinding::new("ctrl-c", Copy, Some("KitInput")),
        KeyBinding::new("cmd-x", Cut, Some("KitInput")),
        KeyBinding::new("ctrl-x", Cut, Some("KitInput")),
        KeyBinding::new("home", Home, Some("KitInput")),
        KeyBinding::new("end", End, Some("KitInput")),
        KeyBinding::new("enter", Newline, Some("KitInput")),
    ]);
}

pub(crate) struct InputState {
    focus_handle: FocusHandle,
    content: SharedString,
    placeholder: SharedString,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    last_layout: Option<ShapedLine>,
    last_bounds: Option<Bounds<Pixels>>,
    is_selecting: bool,
    multiline: bool,
    disabled: bool,
    value_prop: SharedString,
    on_change: Option<InputChangeHandler>,
}

impl InputState {
    fn new(
        cx: &mut gpui::Context<Self>,
        placeholder: SharedString,
        content: SharedString,
        multiline: bool,
    ) -> Self {
        let len = content.len();
        Self {
            focus_handle: cx.focus_handle(),
            content: content.clone(),
            placeholder,
            selected_range: len..len,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
            multiline,
            disabled: false,
            value_prop: content,
            on_change: None,
        }
    }

    fn emit_change(&self, window: &mut Window, cx: &mut gpui::Context<Self>) {
        if let Some(on_change) = self.on_change.clone() {
            on_change(self.content.clone(), window, cx);
        }
    }

    fn clamp_selection(&mut self) {
        let len = self.content.len();
        if self.selected_range.start > len {
            self.selected_range.start = len;
        }
        if self.selected_range.end > len {
            self.selected_range.end = len;
        }
    }

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut gpui::Context<Self>) {
        if self.disabled {
            return;
        }
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx);
        }
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut gpui::Context<Self>) {
        if self.disabled {
            return;
        }
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.selected_range.end), cx);
        } else {
            self.move_to(self.selected_range.end, cx);
        }
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut gpui::Context<Self>) {
        if !self.disabled {
            self.select_to(self.previous_boundary(self.cursor_offset()), cx);
        }
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut gpui::Context<Self>) {
        if !self.disabled {
            self.select_to(self.next_boundary(self.cursor_offset()), cx);
        }
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut gpui::Context<Self>) {
        if self.disabled {
            return;
        }
        self.move_to(0, cx);
        self.select_to(self.content.len(), cx);
    }

    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut gpui::Context<Self>) {
        if !self.disabled {
            self.move_to(0, cx);
        }
    }

    fn end(&mut self, _: &End, _: &mut Window, cx: &mut gpui::Context<Self>) {
        if !self.disabled {
            self.move_to(self.content.len(), cx);
        }
    }

    fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut gpui::Context<Self>) {
        if self.disabled {
            return;
        }
        if self.selected_range.is_empty() {
            let prev = self.previous_boundary(self.cursor_offset());
            if self.cursor_offset() == prev {
                return;
            }
            self.select_to(prev, cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut gpui::Context<Self>) {
        if self.disabled {
            return;
        }
        if self.selected_range.is_empty() {
            let next = self.next_boundary(self.cursor_offset());
            if self.cursor_offset() == next {
                return;
            }
            self.select_to(next, cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        window.focus(&self.focus_handle, cx);
        self.is_selecting = true;
        if event.modifiers.shift {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        } else {
            self.move_to(self.index_for_mouse_position(event.position), cx);
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Window, _: &mut gpui::Context<Self>) {
        self.is_selecting = false;
    }

    fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        if self.is_selecting && !self.disabled {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        }
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut gpui::Context<Self>) {
        if self.disabled {
            return;
        }
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            let text = if self.multiline {
                text
            } else {
                text.replace('\n', " ")
            };
            self.replace_text_in_range(None, &text, window, cx);
        }
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut gpui::Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
        }
    }

    fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut gpui::Context<Self>) {
        if self.disabled || self.selected_range.is_empty() {
            return;
        }
        cx.write_to_clipboard(ClipboardItem::new_string(
            self.content[self.selected_range.clone()].to_string(),
        ));
        self.replace_text_in_range(None, "", window, cx);
    }

    fn newline(&mut self, _: &Newline, window: &mut Window, cx: &mut gpui::Context<Self>) {
        if self.disabled || !self.multiline {
            return;
        }
        self.replace_text_in_range(None, "\n", window, cx);
    }

    fn move_to(&mut self, offset: usize, cx: &mut gpui::Context<Self>) {
        self.selected_range = offset..offset;
        self.selection_reversed = false;
        cx.notify();
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.content.is_empty() {
            return 0;
        }
        let (Some(bounds), Some(line)) = (self.last_bounds.as_ref(), self.last_layout.as_ref())
        else {
            return 0;
        };
        if position.y < bounds.top() {
            return 0;
        }
        if position.y > bounds.bottom() {
            return self.content.len();
        }
        line.closest_index_for_x(position.x - bounds.left())
    }

    fn select_to(&mut self, offset: usize, cx: &mut gpui::Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset;
        } else {
            self.selected_range.end = offset;
        }
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        cx.notify();
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;
        for ch in self.content.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }
        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;
        for ch in self.content.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }
        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        if offset == 0 {
            return 0;
        }
        let mut i = offset - 1;
        while i > 0 && !self.content.is_char_boundary(i) {
            i -= 1;
        }
        i
    }

    fn next_boundary(&self, offset: usize) -> usize {
        if offset >= self.content.len() {
            return self.content.len();
        }
        let mut i = offset + 1;
        while i < self.content.len() && !self.content.is_char_boundary(i) {
            i += 1;
        }
        i
    }
}

impl Focusable for InputState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EntityInputHandler for InputState {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _: &mut Window,
        _: &mut gpui::Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _: bool,
        _: &mut Window,
        _: &mut gpui::Context<Self>,
    ) -> Option<UTF16Selection> {
        if self.disabled {
            return None;
        }
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(
        &self,
        _: &mut Window,
        _: &mut gpui::Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        let new_text = if self.multiline {
            new_text.to_string()
        } else {
            new_text.replace('\n', " ")
        };
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or_else(|| self.marked_range.clone())
            .unwrap_or_else(|| self.selected_range.clone());

        self.content =
            (self.content[0..range.start].to_owned() + &new_text + &self.content[range.end..])
                .into();
        let cursor = range.start + new_text.len();
        self.selected_range = cursor..cursor;
        self.marked_range.take();
        self.emit_change(window, cx);
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or_else(|| self.marked_range.clone())
            .unwrap_or_else(|| self.selected_range.clone());

        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                .into();
        if !new_text.is_empty() {
            self.marked_range = Some(range.start..range.start + new_text.len());
        } else {
            self.marked_range = None;
        }
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.end)
            .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());
        self.emit_change(window, cx);
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _: &mut Window,
        _: &mut gpui::Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let last_layout = self.last_layout.as_ref()?;
        let range = self.range_from_utf16(&range_utf16);
        Some(Bounds::from_corners(
            point(
                bounds.left() + last_layout.x_for_index(range.start),
                bounds.top(),
            ),
            point(
                bounds.left() + last_layout.x_for_index(range.end),
                bounds.bottom(),
            ),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _: &mut Window,
        _: &mut gpui::Context<Self>,
    ) -> Option<usize> {
        let line_point = self.last_bounds?.localize(&point)?;
        let last_layout = self.last_layout.as_ref()?;
        let utf8_index = last_layout.index_for_x(point.x - line_point.x)?;
        Some(self.offset_to_utf16(utf8_index))
    }

    fn accepts_text_input(&self, _: &mut Window, _: &mut gpui::Context<Self>) -> bool {
        !self.disabled
    }
}

struct TextElement {
    input: Entity<InputState>,
    color: Hsla,
    placeholder: Hsla,
    caret: Hsla,
    show_caret: bool,
    fill: bool,
}

struct PrepaintState {
    line: Option<ShapedLine>,
    cursor: Option<PaintQuad>,
    selection: Option<PaintQuad>,
}

impl IntoElement for TextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = if self.fill {
            relative(1.).into()
        } else {
            window.line_height().into()
        };
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let input = self.input.read(cx);
        let content = input.content.clone();
        let selected_range = input.selected_range.clone();
        let cursor = input.cursor_offset();
        let empty = content.is_empty();
        let (display_text, text_color) = if empty {
            (input.placeholder.clone(), self.placeholder)
        } else {
            (SharedString::from(content.replace('\n', " ")), self.color)
        };

        let run = TextRun {
            len: display_text.len(),
            font: window.text_style().font(),
            color: text_color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let runs = if let Some(marked_range) = input.marked_range.as_ref() {
            vec![
                TextRun {
                    len: marked_range.start,
                    ..run.clone()
                },
                TextRun {
                    len: marked_range.end - marked_range.start,
                    underline: Some(UnderlineStyle {
                        color: Some(run.color),
                        thickness: px(1.0),
                        wavy: false,
                    }),
                    ..run.clone()
                },
                TextRun {
                    len: display_text.len() - marked_range.end,
                    ..run
                },
            ]
            .into_iter()
            .filter(|run| run.len > 0)
            .collect()
        } else {
            vec![run]
        };

        let font_size = window.text_style().font_size.to_pixels(window.rem_size());
        let line = window
            .text_system()
            .shape_line(display_text, font_size, &runs, None);

        let cursor_pos = if empty {
            px(0.)
        } else {
            line.x_for_index(cursor)
        };
        let (selection, cursor_quad) = if empty || selected_range.is_empty() {
            (
                None,
                Some(fill(
                    Bounds::new(
                        point(bounds.left() + cursor_pos, bounds.top() + px(1.)),
                        size(px(1.), px(16.)),
                    ),
                    self.caret,
                )),
            )
        } else {
            (
                Some(fill(
                    Bounds::from_corners(
                        point(
                            bounds.left() + line.x_for_index(selected_range.start),
                            bounds.top(),
                        ),
                        point(
                            bounds.left() + line.x_for_index(selected_range.end),
                            bounds.bottom(),
                        ),
                    ),
                    self.caret.opacity(0.18),
                )),
                None,
            )
        };
        PrepaintState {
            line: Some(line),
            cursor: cursor_quad,
            selection,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        let disabled = self.input.read(cx).disabled;
        if !disabled {
            window.handle_input(
                &focus_handle,
                ElementInputHandler::new(bounds, self.input.clone()),
                cx,
            );
        }
        if let Some(selection) = prepaint.selection.take() {
            window.paint_quad(selection);
        }
        let line = prepaint.line.take().unwrap();
        line.paint(
            bounds.origin,
            window.line_height(),
            gpui::TextAlign::Left,
            None,
            window,
            cx,
        )
        .ok();

        if self.show_caret {
            if let Some(cursor) = prepaint.cursor.take() {
                window.paint_quad(cursor);
            }
        }

        self.input.update(cx, |input, _cx| {
            input.last_layout = Some(line);
            input.last_bounds = Some(bounds);
        });
    }
}

/// Single-line field matching Paper `Glassy UI` → Inputs.
#[derive(IntoElement)]
pub struct Input {
    id: SharedString,
    placeholder: SharedString,
    value: SharedString,
    disabled: bool,
    invalid: bool,
    show_focus: bool,
    leading_icon: Option<IconName>,
    trailing: Option<SharedString>,
    helper: Option<SharedString>,
    multiline: bool,
    on_change: Option<InputChangeHandler>,
    style: StyleRefinement,
}

impl Input {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            placeholder: SharedString::default(),
            value: SharedString::default(),
            disabled: false,
            invalid: false,
            show_focus: false,
            leading_icon: None,
            trailing: None,
            helper: None,
            multiline: false,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Paint the Paper focus ring and caret even when the field is not focused.
    pub fn show_focus(mut self, show_focus: bool) -> Self {
        self.show_focus = show_focus;
        self
    }

    pub fn leading_icon(mut self, icon: IconName) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    pub fn trailing(mut self, trailing: impl Into<SharedString>) -> Self {
        self.trailing = Some(trailing.into());
        self
    }

    pub fn helper(mut self, helper: impl Into<SharedString>) -> Self {
        self.helper = Some(helper.into());
        self
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    /// Fired after the field text changes. `value` is the current contents.
    pub fn on_change(
        mut self,
        listener: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(listener));
        self
    }
}

impl Styled for Input {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Input {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let placeholder = self.placeholder.clone();
        let value = self.value.clone();
        let multiline = self.multiline;
        let disabled = self.disabled;
        let on_change = self.on_change.clone();
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, cx| {
            InputState::new(cx, placeholder, value, multiline)
        });
        state.update(cx, |input, _| {
            input.disabled = disabled;
            input.multiline = multiline;
            input.on_change = on_change;
            if input.value_prop.as_ref() != self.value.as_ref() {
                input.content = self.value.clone();
                input.value_prop = self.value.clone();
                input.clamp_selection();
            }
        });

        let focus_handle = state.read(cx).focus_handle.clone();
        let focused = focus_handle.is_focused(window);
        let field_state = if self.disabled {
            FieldState::Disabled
        } else if self.invalid {
            FieldState::Invalid
        } else if self.show_focus || focused {
            FieldState::Focus
        } else {
            FieldState::Rest
        };
        let chrome = field_chrome(theme, field_state);
        let show_caret = !self.disabled && (self.show_focus || focused);
        let has_icon = self.leading_icon.is_some();
        let pad_x = if has_icon || self.trailing.is_some() {
            14.0
        } else {
            16.0
        };
        let width = if multiline { px(320.) } else { px(280.) };
        let height = if multiline { px(96.) } else { px(36.) };
        let line_height = if multiline { 20.0 } else { 18.0 };
        let helper = self.helper.clone();
        let helper_color = theme.destructive;

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

        let input_debug_selector = self.id.to_string();
        let field = div()
            .id(self.id.clone())
            .debug_selector(move || input_debug_selector.clone())
            .key_context("KitInput")
            .role(Role::TextInput)
            .when(!self.placeholder.is_empty(), |el| {
                el.aria_placeholder(self.placeholder.clone())
            })
            .track_focus(&focus_handle)
            .tab_stop(!self.disabled)
            .flex()
            .when(multiline, |el| el.items_start())
            .when(!multiline, |el| el.items_center())
            .when(has_icon || self.trailing.is_some(), |el| el.gap(px(8.)))
            .w_full()
            .h(height)
            .flex_shrink_0()
            .px(px(pad_x))
            .when(multiline, |el| el.py(px(10.)))
            .rounded(px(6.))
            .border_1()
            .border_color(chrome.border)
            .bg(chrome.bg)
            .shadow(shadows)
            .text_color(chrome.fg)
            .font_family(theme.font_family)
            .font_weight(FontWeight::NORMAL)
            .text_size(px(14.))
            .line_height(px(line_height))
            .overflow_hidden()
            .when(self.disabled, |el| el.cursor_default())
            .when(!self.disabled, |el| el.cursor(CursorStyle::IBeam))
            .on_action(window.listener_for(&state, InputState::backspace))
            .on_action(window.listener_for(&state, InputState::delete))
            .on_action(window.listener_for(&state, InputState::left))
            .on_action(window.listener_for(&state, InputState::right))
            .on_action(window.listener_for(&state, InputState::select_left))
            .on_action(window.listener_for(&state, InputState::select_right))
            .on_action(window.listener_for(&state, InputState::select_all))
            .on_action(window.listener_for(&state, InputState::home))
            .on_action(window.listener_for(&state, InputState::end))
            .on_action(window.listener_for(&state, InputState::paste))
            .on_action(window.listener_for(&state, InputState::cut))
            .on_action(window.listener_for(&state, InputState::copy))
            .on_action(window.listener_for(&state, InputState::newline))
            .on_mouse_down(
                MouseButton::Left,
                window.listener_for(&state, InputState::on_mouse_down),
            )
            .on_mouse_up(
                MouseButton::Left,
                window.listener_for(&state, InputState::on_mouse_up),
            )
            .on_mouse_up_out(
                MouseButton::Left,
                window.listener_for(&state, InputState::on_mouse_up),
            )
            .on_mouse_move(window.listener_for(&state, InputState::on_mouse_move))
            .when_some(self.leading_icon, |el, icon| {
                el.child(Icon::new(icon).px(px(16.)).color(chrome.placeholder))
            })
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.))
                    .when(multiline, |el| el.h_full())
                    .overflow_hidden()
                    .child(TextElement {
                        input: state.clone(),
                        color: chrome.fg,
                        placeholder: chrome.placeholder,
                        caret: chrome.caret,
                        show_caret,
                        fill: multiline,
                    }),
            )
            .when_some(self.trailing, |el, trailing| {
                el.child(
                    div()
                        .flex_shrink_0()
                        .font_family(theme.font_family)
                        .font_weight(FontWeight::MEDIUM)
                        .text_size(px(12.))
                        .line_height(px(16.))
                        .text_color(theme.label)
                        .child(trailing),
                )
            });

        div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .w(width)
            .flex_shrink_0()
            .refine_style(&self.style)
            .child(field)
            .when_some(helper, |el, helper| {
                el.child(
                    div()
                        .font_family(theme.font_family)
                        .font_weight(FontWeight::MEDIUM)
                        .text_size(px(13.))
                        .line_height(px(16.))
                        .text_color(helper_color)
                        .child(helper),
                )
            })
    }
}

/// Many-line field. Same chrome as [`Input`], 320×96, line-height 20.
pub fn textarea(id: impl Into<SharedString>) -> Input {
    Input::new(id).multiline(true)
}
