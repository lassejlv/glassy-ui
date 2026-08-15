use std::rc::Rc;

use gpui::{prelude::*, App, FocusHandle, RenderOnce, SharedString, Window};

use crate::{
    Button, ButtonVariant, Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogTitle,
};

type AlertDialogHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

struct AlertDialogState {
    cancel_focus: FocusHandle,
    confirm_focus: FocusHandle,
}

/// Controlled destructive confirmation with safe focus and a locked scrim.
#[derive(IntoElement)]
pub struct AlertDialog {
    id: SharedString,
    open: bool,
    title: SharedString,
    description: SharedString,
    cancel_label: SharedString,
    confirm_label: SharedString,
    on_cancel: Option<AlertDialogHandler>,
    on_confirm: Option<AlertDialogHandler>,
}

impl AlertDialog {
    pub fn new(
        id: impl Into<SharedString>,
        title: impl Into<SharedString>,
        description: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            open: false,
            title: title.into(),
            description: description.into(),
            cancel_label: SharedString::from("Cancel"),
            confirm_label: SharedString::from("Continue"),
            on_cancel: None,
            on_confirm: None,
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn cancel_label(mut self, label: impl Into<SharedString>) -> Self {
        self.cancel_label = label.into();
        self
    }

    pub fn confirm_label(mut self, label: impl Into<SharedString>) -> Self {
        self.confirm_label = label.into();
        self
    }

    pub fn on_cancel(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_cancel = Some(Rc::new(listener));
        self
    }

    pub fn on_confirm(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_confirm = Some(Rc::new(listener));
        self
    }
}

impl RenderOnce for AlertDialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state_id = SharedString::from(format!("{}-actions", self.id));
        let state = window.use_keyed_state(state_id, cx, |_, cx| AlertDialogState {
            cancel_focus: cx.focus_handle().tab_stop(true),
            confirm_focus: cx.focus_handle().tab_stop(true),
        });
        let cancel_focus = state.read(cx).cancel_focus.clone();
        let confirm_focus = state.read(cx).confirm_focus.clone();
        let cancel_click = self.on_cancel.clone();
        let confirm_click = self.on_confirm.clone();
        let escape_cancel = self.on_cancel;
        let cancel_id = SharedString::from(format!("{}-cancel", self.id));
        let confirm_id = SharedString::from(format!("{}-confirm", self.id));

        Dialog::new(self.id)
            .open(self.open)
            .dismiss_on_scrim(false)
            .initial_focus(cancel_focus.clone())
            .focus_cycle([cancel_focus.clone(), confirm_focus.clone()])
            .on_dismiss(move |window, cx| {
                if let Some(on_cancel) = escape_cancel.as_ref() {
                    on_cancel(window, cx);
                }
            })
            .child(
                DialogContent::new()
                    .child(
                        DialogHeader::new()
                            .child(DialogTitle::new(self.title))
                            .child(DialogDescription::new(self.description)),
                    )
                    .child(
                        DialogFooter::new()
                            .child(
                                Button::new(cancel_id, self.cancel_label)
                                    .variant(ButtonVariant::Ghost)
                                    .focus_handle(cancel_focus)
                                    .on_click(move |_, window, cx| {
                                        if let Some(on_cancel) = cancel_click.as_ref() {
                                            on_cancel(window, cx);
                                        }
                                    }),
                            )
                            .child(
                                Button::new(confirm_id, self.confirm_label)
                                    .variant(ButtonVariant::Destructive)
                                    .focus_handle(confirm_focus)
                                    .on_click(move |_, window, cx| {
                                        if let Some(on_confirm) = confirm_click.as_ref() {
                                            on_confirm(window, cx);
                                        }
                                    }),
                            ),
                    ),
            )
    }
}
