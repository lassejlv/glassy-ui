//! Narrow compatibility helpers for the published GPUI 0.2.2 API.
//!
//! GPUI 0.2.2 predates the AccessKit-backed semantic element methods used by
//! newer Zed revisions. Keep the call sites and intent intact so the helpers
//! can be removed without redesigning every component when published GPUI
//! catches up.

use gpui::{AlignSelf, SharedString, Styled};

#[derive(Clone, Copy)]
pub(crate) enum Role {
    Button,
    CheckBox,
    ComboBox,
    Dialog,
    Label,
    ListBox,
    Menu,
    MenuItem,
    Option,
    RadioButton,
    Switch,
    TextInput,
    Tooltip,
}

#[derive(Clone, Copy)]
pub(crate) enum Toggled {
    False,
    True,
    Mixed,
}

/// Preserve newer GPUI accessibility call sites on 0.2.2, whose renderer has
/// no public accessibility-node API. These methods intentionally leave the
/// element unchanged; keyboard, focus, labels, and visual state remain owned
/// by the components themselves.
pub(crate) trait AccessibilityExt: Sized {
    fn role(self, _role: Role) -> Self {
        self
    }

    fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }

    fn aria_placeholder(self, _placeholder: impl Into<SharedString>) -> Self {
        self
    }

    fn aria_expanded(self, _expanded: bool) -> Self {
        self
    }

    fn aria_selected(self, _selected: bool) -> Self {
        self
    }

    fn aria_toggled(self, _toggled: Toggled) -> Self {
        self
    }
}

impl<T> AccessibilityExt for T {}

/// `self_start` was added after 0.2.2; express it through the underlying style
/// field that already exists in this release.
pub(crate) trait StyleCompatExt: Styled + Sized {
    fn self_start(mut self) -> Self {
        self.style().align_self = Some(AlignSelf::FlexStart);
        self
    }
}

impl<T: Styled> StyleCompatExt for T {}
