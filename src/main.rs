//! Design-spec hex+alpha is grouped as `RRGGBB_AA`.
#![allow(clippy::unusual_byte_groupings)]

use glassy_ui::{
    init as init_ui, init_motion, init_theme, load_fonts, paint, rgb, textarea, ActiveTheme,
    AlertDialog, Assets, Badge, BadgeVariant, Button, ButtonGroup, ButtonSize, ButtonVariant,
    CheckState, Checkbox, CircularProgress, Command, CommandGroup, CommandItem, CommandSize,
    ContextMenu, Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DropdownMenu, DropdownMenuEntry, DropdownMenuItem, Icon, IconName, Input, Kbd, Label, Popover,
    PopoverContent, PopoverDescription, PopoverTitle, Progress, Radio, Select, SelectItem,
    Separator, Skeleton, Spinner, SpinnerSize, SpinnerTone, Switch, Theme, Tooltip,
    TooltipPlacement,
};
use gpui::{
    actions, div, point, prelude::*, px, size, App, Application, Bounds, BoxShadow, FocusHandle,
    Focusable, FontWeight, KeyBinding, Menu, MenuItem, TitlebarOptions, Window,
    WindowBackgroundAppearance, WindowBounds, WindowOptions,
};

actions!(
    glassy_gallery,
    [
        Quit,
        ToggleTheme,
        ShowButtons,
        ShowSkeletons,
        ShowTooltips,
        ShowProgress,
        ShowSpinners,
        ShowInputs,
        ShowLabels,
        ShowCheckboxes,
        ShowSwitches,
        ShowRadios,
        ShowSelects,
        ShowKbds,
        ShowSeparators,
        ShowBadges,
        ShowDialogs,
        ShowAlertDialogs,
        ShowPopovers,
        ShowDropdownMenus,
        ShowContextMenus,
        ShowCommand,
        ToggleCommand,
        ShowNextPage,
    ]
);

fn box_shadow(x: f32, y: f32, color: gpui::Hsla, blur: f32, spread: f32) -> BoxShadow {
    BoxShadow {
        color,
        offset: point(px(x), px(y)),
        blur_radius: px(blur),
        spread_radius: px(spread),
    }
}

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        init_motion(cx);
        init_theme(cx);
        init_ui(cx);
        load_fonts(cx).expect("register Inter");

        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.on_action(|_: &ToggleTheme, cx| cx.toggle_theme());
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("ctrl-q", Quit, None),
            KeyBinding::new("cmd-d", ToggleTheme, None),
            KeyBinding::new("ctrl-d", ToggleTheme, None),
            KeyBinding::new("cmd-1", ShowButtons, None),
            KeyBinding::new("ctrl-1", ShowButtons, None),
            KeyBinding::new("cmd-0", ShowSkeletons, None),
            KeyBinding::new("ctrl-0", ShowSkeletons, None),
            KeyBinding::new("cmd-shift-t", ShowTooltips, None),
            KeyBinding::new("ctrl-shift-t", ShowTooltips, None),
            KeyBinding::new("cmd-shift-0", ShowProgress, None),
            KeyBinding::new("ctrl-shift-0", ShowProgress, None),
            KeyBinding::new("cmd-2", ShowSpinners, None),
            KeyBinding::new("ctrl-2", ShowSpinners, None),
            KeyBinding::new("cmd-3", ShowInputs, None),
            KeyBinding::new("ctrl-3", ShowInputs, None),
            KeyBinding::new("cmd-4", ShowLabels, None),
            KeyBinding::new("ctrl-4", ShowLabels, None),
            KeyBinding::new("cmd-5", ShowCheckboxes, None),
            KeyBinding::new("ctrl-5", ShowCheckboxes, None),
            KeyBinding::new("cmd-6", ShowSwitches, None),
            KeyBinding::new("ctrl-6", ShowSwitches, None),
            KeyBinding::new("cmd-7", ShowRadios, None),
            KeyBinding::new("ctrl-7", ShowRadios, None),
            KeyBinding::new("cmd-8", ShowSelects, None),
            KeyBinding::new("ctrl-8", ShowSelects, None),
            KeyBinding::new("cmd-9", ShowKbds, None),
            KeyBinding::new("ctrl-9", ShowKbds, None),
            KeyBinding::new("cmd-shift-b", ShowBadges, None),
            KeyBinding::new("ctrl-shift-b", ShowBadges, None),
            KeyBinding::new("cmd-shift-d", ShowDialogs, None),
            KeyBinding::new("ctrl-shift-d", ShowDialogs, None),
            KeyBinding::new("cmd-shift-a", ShowAlertDialogs, None),
            KeyBinding::new("ctrl-shift-a", ShowAlertDialogs, None),
            KeyBinding::new("cmd-shift-p", ShowPopovers, None),
            KeyBinding::new("ctrl-shift-p", ShowPopovers, None),
            KeyBinding::new("cmd-shift-m", ShowDropdownMenus, None),
            KeyBinding::new("ctrl-shift-m", ShowDropdownMenus, None),
            KeyBinding::new("cmd-shift-c", ShowContextMenus, None),
            KeyBinding::new("ctrl-shift-c", ShowContextMenus, None),
            KeyBinding::new("cmd-shift-k", ShowCommand, None),
            KeyBinding::new("ctrl-shift-k", ShowCommand, None),
            KeyBinding::new("cmd-k", ToggleCommand, None),
            KeyBinding::new("ctrl-k", ToggleCommand, None),
            KeyBinding::new("cmd-]", ShowNextPage, None),
            KeyBinding::new("ctrl-]", ShowNextPage, None),
        ]);
        cx.set_menus(vec![Menu {
            name: "Glassy UI".into(),
            items: vec![
                MenuItem::action("Buttons", ShowButtons),
                MenuItem::action("Skeletons", ShowSkeletons),
                MenuItem::action("Tooltips", ShowTooltips),
                MenuItem::action("Progress", ShowProgress),
                MenuItem::action("Spinners", ShowSpinners),
                MenuItem::action("Inputs", ShowInputs),
                MenuItem::action("Labels", ShowLabels),
                MenuItem::action("Checkboxes", ShowCheckboxes),
                MenuItem::action("Switches", ShowSwitches),
                MenuItem::action("Radios", ShowRadios),
                MenuItem::action("Selects", ShowSelects),
                MenuItem::action("Kbd", ShowKbds),
                MenuItem::action("Separators", ShowSeparators),
                MenuItem::action("Badges", ShowBadges),
                MenuItem::action("Dialogs", ShowDialogs),
                MenuItem::action("Alert dialogs", ShowAlertDialogs),
                MenuItem::action("Popovers", ShowPopovers),
                MenuItem::action("Dropdown menus", ShowDropdownMenus),
                MenuItem::action("Context menus", ShowContextMenus),
                MenuItem::action("Command", ShowCommand),
                MenuItem::separator(),
                MenuItem::action("Toggle Light / Dark", ToggleTheme),
                MenuItem::separator(),
                MenuItem::action("Quit Glassy UI", Quit),
            ],
        }]);

        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        open_gallery(cx);
        cx.activate(true);
    });
}

fn open_gallery(cx: &mut App) {
    let bounds = Bounds::centered(None, size(px(1440.), px(900.)), cx);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            window_min_size: Some(size(px(640.), px(420.))),
            window_background: WindowBackgroundAppearance::Opaque,
            app_id: Some("dev.glassy.gallery".into()),
            titlebar: Some(TitlebarOptions {
                title: Some("Glassy UI — Inputs".into()),
                appears_transparent: true,
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| cx.new(|cx| Gallery::new(window, cx)),
    )
    .expect("open gallery window");
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GalleryPage {
    Buttons,
    Skeletons,
    Tooltips,
    Progress,
    Spinners,
    Inputs,
    Labels,
    Checkboxes,
    Switches,
    Radios,
    Selects,
    Kbds,
    Separators,
    Badges,
    Dialogs,
    AlertDialogs,
    Popovers,
    DropdownMenus,
    ContextMenus,
    Command,
}

impl GalleryPage {
    fn title(self) -> &'static str {
        match self {
            Self::Buttons => "Glassy UI — Buttons",
            Self::Skeletons => "Glassy UI — Skeletons",
            Self::Tooltips => "Glassy UI — Tooltips",
            Self::Progress => "Glassy UI — Progress",
            Self::Spinners => "Glassy UI — Spinners",
            Self::Inputs => "Glassy UI — Inputs",
            Self::Labels => "Glassy UI — Labels",
            Self::Checkboxes => "Glassy UI — Checkboxes",
            Self::Switches => "Glassy UI — Switches",
            Self::Radios => "Glassy UI — Radios",
            Self::Selects => "Glassy UI — Selects",
            Self::Kbds => "Glassy UI — Kbd",
            Self::Separators => "Glassy UI — Separators",
            Self::Badges => "Glassy UI — Badges",
            Self::Dialogs => "Glassy UI — Dialogs",
            Self::AlertDialogs => "Glassy UI — Alert dialogs",
            Self::Popovers => "Glassy UI — Popovers",
            Self::DropdownMenus => "Glassy UI — Dropdown menus",
            Self::ContextMenus => "Glassy UI — Context menus",
            Self::Command => "Glassy UI — Command",
        }
    }

    fn next_label(self) -> &'static str {
        self.next().short_name()
    }

    fn short_name(self) -> &'static str {
        match self {
            Self::Buttons => "Buttons",
            Self::Skeletons => "Skeletons",
            Self::Tooltips => "Tooltips",
            Self::Progress => "Progress",
            Self::Spinners => "Spinners",
            Self::Inputs => "Inputs",
            Self::Labels => "Labels",
            Self::Checkboxes => "Checkboxes",
            Self::Switches => "Switches",
            Self::Radios => "Radios",
            Self::Selects => "Selects",
            Self::Kbds => "Kbd",
            Self::Separators => "Separators",
            Self::Badges => "Badges",
            Self::Dialogs => "Dialogs",
            Self::AlertDialogs => "Alert dialogs",
            Self::Popovers => "Popovers",
            Self::DropdownMenus => "Dropdown menus",
            Self::ContextMenus => "Context menus",
            Self::Command => "Command",
        }
    }

    fn next(self) -> Self {
        match self {
            Self::Buttons => Self::Skeletons,
            Self::Skeletons => Self::Tooltips,
            Self::Tooltips => Self::Progress,
            Self::Progress => Self::Spinners,
            Self::Spinners => Self::Inputs,
            Self::Inputs => Self::Labels,
            Self::Labels => Self::Checkboxes,
            Self::Checkboxes => Self::Switches,
            Self::Switches => Self::Radios,
            Self::Radios => Self::Selects,
            Self::Selects => Self::Kbds,
            Self::Kbds => Self::Separators,
            Self::Separators => Self::Badges,
            Self::Badges => Self::Dialogs,
            Self::Dialogs => Self::AlertDialogs,
            Self::AlertDialogs => Self::Popovers,
            Self::Popovers => Self::DropdownMenus,
            Self::DropdownMenus => Self::ContextMenus,
            Self::ContextMenus => Self::Command,
            Self::Command => Self::Buttons,
        }
    }
}

struct Gallery {
    focus_handle: FocusHandle,
    page: GalleryPage,
    dialog_open: bool,
    alert_dialog_open: bool,
    command_open: bool,
    command_focus: FocusHandle,
}

impl Gallery {
    fn new(window: &mut Window, cx: &mut gpui::Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let command_focus = cx.focus_handle().tab_stop(true);
        focus_handle.focus(window);
        Self {
            focus_handle,
            page: GalleryPage::Inputs,
            dialog_open: false,
            alert_dialog_open: false,
            command_open: false,
            command_focus,
        }
    }

    fn quit(&mut self, _: &Quit, _window: &mut Window, cx: &mut gpui::Context<Self>) {
        cx.quit();
    }

    fn toggle_theme(
        &mut self,
        _: &ToggleTheme,
        _window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        cx.toggle_theme();
    }

    fn show_buttons(&mut self, _: &ShowButtons, window: &mut Window, cx: &mut gpui::Context<Self>) {
        self.set_page(GalleryPage::Buttons, window, cx);
    }

    fn show_skeletons(
        &mut self,
        _: &ShowSkeletons,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        self.set_page(GalleryPage::Skeletons, window, cx);
    }

    fn show_tooltips(
        &mut self,
        _: &ShowTooltips,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        self.set_page(GalleryPage::Tooltips, window, cx);
    }

    fn show_progress(
        &mut self,
        _: &ShowProgress,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        self.set_page(GalleryPage::Progress, window, cx);
    }

    fn show_spinners(
        &mut self,
        _: &ShowSpinners,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        self.set_page(GalleryPage::Spinners, window, cx);
    }

    fn show_inputs(&mut self, _: &ShowInputs, window: &mut Window, cx: &mut gpui::Context<Self>) {
        self.set_page(GalleryPage::Inputs, window, cx);
    }

    fn show_labels(&mut self, _: &ShowLabels, window: &mut Window, cx: &mut gpui::Context<Self>) {
        self.set_page(GalleryPage::Labels, window, cx);
    }

    fn show_checkboxes(
        &mut self,
        _: &ShowCheckboxes,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        self.set_page(GalleryPage::Checkboxes, window, cx);
    }

    fn show_switches(
        &mut self,
        _: &ShowSwitches,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        self.set_page(GalleryPage::Switches, window, cx);
    }

    fn show_radios(&mut self, _: &ShowRadios, window: &mut Window, cx: &mut gpui::Context<Self>) {
        self.set_page(GalleryPage::Radios, window, cx);
    }

    fn show_selects(&mut self, _: &ShowSelects, window: &mut Window, cx: &mut gpui::Context<Self>) {
        self.set_page(GalleryPage::Selects, window, cx);
    }

    fn show_kbds(&mut self, _: &ShowKbds, window: &mut Window, cx: &mut gpui::Context<Self>) {
        self.set_page(GalleryPage::Kbds, window, cx);
    }

    fn show_separators(
        &mut self,
        _: &ShowSeparators,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        self.set_page(GalleryPage::Separators, window, cx);
    }

    fn show_badges(&mut self, _: &ShowBadges, window: &mut Window, cx: &mut gpui::Context<Self>) {
        self.set_page(GalleryPage::Badges, window, cx);
    }

    fn show_dialogs(&mut self, _: &ShowDialogs, window: &mut Window, cx: &mut gpui::Context<Self>) {
        self.set_page(GalleryPage::Dialogs, window, cx);
    }

    fn show_alert_dialogs(
        &mut self,
        _: &ShowAlertDialogs,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        self.set_page(GalleryPage::AlertDialogs, window, cx);
    }

    fn show_popovers(
        &mut self,
        _: &ShowPopovers,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        self.set_page(GalleryPage::Popovers, window, cx);
    }

    fn show_dropdown_menus(
        &mut self,
        _: &ShowDropdownMenus,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        self.set_page(GalleryPage::DropdownMenus, window, cx);
    }

    fn show_context_menus(
        &mut self,
        _: &ShowContextMenus,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        self.set_page(GalleryPage::ContextMenus, window, cx);
    }

    fn show_command(&mut self, _: &ShowCommand, window: &mut Window, cx: &mut gpui::Context<Self>) {
        self.set_page(GalleryPage::Command, window, cx);
    }

    fn toggle_command(
        &mut self,
        _: &ToggleCommand,
        _window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        self.command_open = !self.command_open;
        cx.notify();
    }

    fn show_next_page(
        &mut self,
        _: &ShowNextPage,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        self.set_page(self.page.next(), window, cx);
    }

    fn set_page(&mut self, page: GalleryPage, window: &mut Window, cx: &mut gpui::Context<Self>) {
        self.page = page;
        self.dialog_open = false;
        self.alert_dialog_open = false;
        self.command_open = false;
        window.set_window_title(self.page.title());
        cx.notify();
    }
}

impl Focusable for Gallery {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl gpui::Render for Gallery {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let page = self.page;
        let dialog_open = self.dialog_open;
        let alert_dialog_open = self.alert_dialog_open;
        let command_open = self.command_open;
        let command_focus = self.command_focus.clone();
        let gallery = cx.entity();
        let command_dismiss_gallery = gallery.clone();
        let command_item_gallery = gallery.clone();

        div()
            .id("gallery")
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.canvas)
            .font_family(theme.font_family)
            .text_color(theme.ink)
            .track_focus(&self.focus_handle)
            .key_context("Gallery")
            .on_action(cx.listener(Self::quit))
            .on_action(cx.listener(Self::toggle_theme))
            .on_action(cx.listener(Self::show_buttons))
            .on_action(cx.listener(Self::show_skeletons))
            .on_action(cx.listener(Self::show_tooltips))
            .on_action(cx.listener(Self::show_progress))
            .on_action(cx.listener(Self::show_spinners))
            .on_action(cx.listener(Self::show_inputs))
            .on_action(cx.listener(Self::show_labels))
            .on_action(cx.listener(Self::show_checkboxes))
            .on_action(cx.listener(Self::show_switches))
            .on_action(cx.listener(Self::show_radios))
            .on_action(cx.listener(Self::show_selects))
            .on_action(cx.listener(Self::show_kbds))
            .on_action(cx.listener(Self::show_separators))
            .on_action(cx.listener(Self::show_badges))
            .on_action(cx.listener(Self::show_dialogs))
            .on_action(cx.listener(Self::show_alert_dialogs))
            .on_action(cx.listener(Self::show_popovers))
            .on_action(cx.listener(Self::show_dropdown_menus))
            .on_action(cx.listener(Self::show_context_menus))
            .on_action(cx.listener(Self::show_command))
            .on_action(cx.listener(Self::toggle_command))
            .on_action(cx.listener(Self::show_next_page))
            .child(
                div()
                    .id("kit-page")
                    .flex_1()
                    .min_h(px(0.))
                    .w_full()
                    .overflow_y_scroll()
                    .child(match page {
                        GalleryPage::Buttons => buttons_page(theme, page).into_any_element(),
                        GalleryPage::Skeletons => skeletons_page(theme, page).into_any_element(),
                        GalleryPage::Tooltips => tooltips_page(theme, page).into_any_element(),
                        GalleryPage::Progress => progress_page(theme, page).into_any_element(),
                        GalleryPage::Spinners => spinners_page(theme, page).into_any_element(),
                        GalleryPage::Inputs => inputs_page(theme, page).into_any_element(),
                        GalleryPage::Labels => labels_page(theme, page).into_any_element(),
                        GalleryPage::Checkboxes => checkboxes_page(theme, page).into_any_element(),
                        GalleryPage::Switches => switches_page(theme, page).into_any_element(),
                        GalleryPage::Radios => radios_page(theme, page).into_any_element(),
                        GalleryPage::Selects => selects_page(theme, page).into_any_element(),
                        GalleryPage::Kbds => kbds_page(theme, page).into_any_element(),
                        GalleryPage::Separators => separators_page(theme, page).into_any_element(),
                        GalleryPage::Badges => badges_page(theme, page).into_any_element(),
                        GalleryPage::Dialogs => {
                            dialogs_page(theme, page, dialog_open, gallery.clone())
                                .into_any_element()
                        }
                        GalleryPage::AlertDialogs => {
                            alert_dialogs_page(theme, page, alert_dialog_open, gallery.clone())
                                .into_any_element()
                        }
                        GalleryPage::Popovers => popovers_page(theme, page).into_any_element(),
                        GalleryPage::DropdownMenus => {
                            dropdown_menus_page(theme, page).into_any_element()
                        }
                        GalleryPage::ContextMenus => {
                            context_menus_page(theme, page).into_any_element()
                        }
                        GalleryPage::Command => command_page(theme, page).into_any_element(),
                    }),
            )
            .child(
                Dialog::new("gallery-command-dialog")
                    .open(command_open)
                    .initial_focus(command_focus.clone())
                    .on_dismiss(move |_, cx| {
                        command_dismiss_gallery.update(cx, |gallery, cx| {
                            gallery.command_open = false;
                            cx.notify();
                        });
                    })
                    .child(gallery_command(command_item_gallery, command_focus)),
            )
    }
}

fn buttons_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "Button",
            "A set of buttons for every action.",
            "Solid, outline, and ghost. Sized from compact toolbars to primary conversion moments.",
        ))
        .child(section(
            theme,
            "Default",
            56.0,
            false,
            div()
                .flex()
                .items_center()
                .gap(px(12.))
                .child(Button::new("primary", "Primary"))
                .child(Button::new("secondary", "Secondary").variant(ButtonVariant::Secondary))
                .child(
                    Button::new("destructive", "Destructive").variant(ButtonVariant::Destructive),
                ),
        ))
        .child(section(
            theme,
            "Outline",
            40.0,
            false,
            div()
                .flex()
                .items_center()
                .gap(px(12.))
                .child(Button::new("sign-in", "Sign In").variant(ButtonVariant::Outline))
                .child(Button::new("continue", "Continue").variant(ButtonVariant::Outline))
                .child(Button::new("delete", "Delete").variant(ButtonVariant::OutlineDestructive)),
        ))
        .child(section(
            theme,
            "Ghost",
            40.0,
            false,
            div()
                .flex()
                .items_center()
                .gap(px(12.))
                .child(Button::new("cancel", "Cancel").variant(ButtonVariant::Ghost))
                .child(Button::new("learn-more", "Learn more").variant(ButtonVariant::Ghost))
                .child(
                    Button::new("skip", "Skip")
                        .variant(ButtonVariant::Ghost)
                        .muted(true),
                ),
        ))
        .child(section(
            theme,
            "With icons",
            40.0,
            false,
            div()
                .flex()
                .items_center()
                .gap(px(12.))
                .child(Button::new("new-project", "New project").leading_icon(IconName::Plus))
                .child(
                    Button::new("export", "Export")
                        .variant(ButtonVariant::Secondary)
                        .leading_icon(IconName::Download),
                )
                .child(
                    Button::new("next", "Next")
                        .variant(ButtonVariant::Outline)
                        .trailing_icon(IconName::ChevronRight),
                ),
        ))
        .child(section(
            theme,
            "Sizes",
            40.0,
            false,
            div()
                .flex()
                .items_center()
                .gap(px(12.))
                .child(Button::new("small", "Small").size(ButtonSize::Small))
                .child(Button::new("medium", "Medium").size(ButtonSize::Medium))
                .child(Button::new("large", "Large").size(ButtonSize::Large))
                .child(Button::icon_only("search", IconName::Search)),
        ))
        .child(section(
            theme,
            "States",
            40.0,
            true,
            div()
                .flex()
                .items_center()
                .gap(px(12.))
                .child(
                    Button::new("disabled", "Disabled")
                        .variant(ButtonVariant::Secondary)
                        .disabled(true),
                )
                .child(Button::new("loading", "Loading").loading(true))
                .child(
                    ButtonGroup::new()
                        .child(
                            Button::new("save-draft", "Save draft")
                                .variant(ButtonVariant::Secondary)
                                .grouped(true),
                        )
                        .child(Button::new("publish", "Publish").grouped(true)),
                ),
        ))
}

fn skeletons_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "SKELETON",
            "A shape while we wait.",
            "Secondary glass, pulsing. A line, a face, a control. Reduced motion holds the first frame.",
        ))
        .child(section(
            theme,
            "Shapes",
            56.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(40.))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_start()
                        .gap(px(10.))
                        .child(Skeleton::text("skeleton-text"))
                        .child(caption(theme, "Text")),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .w(px(72.))
                        .flex_shrink_0()
                        .gap(px(10.))
                        .child(Skeleton::avatar("skeleton-avatar"))
                        .child(caption(theme, "Avatar")),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_start()
                        .gap(px(10.))
                        .child(Skeleton::control("skeleton-control"))
                        .child(caption(theme, "Control")),
                ),
        ))
        .child(section(
            theme,
            "In use",
            40.0,
            true,
            div()
                .flex()
                .flex_col()
                .w(px(280.))
                .gap(px(12.))
                .child(skeleton_list_row(
                    "skeleton-row-one",
                    160.0,
                    100.0,
                ))
                .child(skeleton_list_row(
                    "skeleton-row-two",
                    140.0,
                    88.0,
                )),
        ))
}

fn skeleton_list_row(id: &'static str, title_width: f32, body_width: f32) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(12.))
        .child(Skeleton::avatar(format!("{id}-avatar")))
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .gap(px(8.))
                .child(
                    Skeleton::text(format!("{id}-title"))
                        .w(px(title_width))
                        .h(px(12.)),
                )
                .child(
                    Skeleton::text(format!("{id}-body"))
                        .w(px(body_width))
                        .h(px(10.)),
                ),
        )
}

fn tooltips_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "TOOLTIP",
            "A name, not a lecture.",
            "Inverse glass, twenty-four tall. Delay about 300ms. Same path in and out. Never sit under the cursor.",
        ))
        .child(section(
            theme,
            "Default",
            56.0,
            false,
            tooltip_specimen(theme, TooltipPlacement::Above, None),
        ))
        .child(section(
            theme,
            "Placement",
            40.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(48.))
                .child(tooltip_specimen(
                    theme,
                    TooltipPlacement::Above,
                    Some("Above"),
                ))
                .child(tooltip_specimen(
                    theme,
                    TooltipPlacement::Below,
                    Some("Below"),
                ))
                .child(tooltip_specimen(
                    theme,
                    TooltipPlacement::Start,
                    Some("Start"),
                ))
                .child(tooltip_specimen(
                    theme,
                    TooltipPlacement::End,
                    Some("End"),
                )),
        ))
        .child(section(
            theme,
            "In use",
            40.0,
            true,
            div()
                .flex()
                .items_end()
                .gap(px(8.))
                .child(
                    Button::icon_only("tooltip-search", IconName::Search)
                        .variant(ButtonVariant::Ghost)
                        .tooltip(Tooltip::new("Search").placement(TooltipPlacement::Above)),
                )
                .child(
                    Button::icon_only("tooltip-new", IconName::Plus)
                        .variant(ButtonVariant::Ghost)
                        .tooltip(Tooltip::new("New page").placement(TooltipPlacement::Above)),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(6.))
                        .child(Tooltip::new("Export PNG"))
                        .child(tooltip_trigger(
                            "tooltip-export-in-use",
                            TooltipPlacement::Above,
                        )),
                ),
        ))
}

fn tooltip_specimen(
    theme: Theme,
    placement: TooltipPlacement,
    label: Option<&'static str>,
) -> impl IntoElement {
    let width = match placement {
        TooltipPlacement::Above | TooltipPlacement::Below => 120.0,
        TooltipPlacement::Start | TooltipPlacement::End => 160.0,
    };
    let trigger_id = match (placement, label) {
        (TooltipPlacement::Above, None) => "tooltip-default-trigger",
        (TooltipPlacement::Above, Some(_)) => "tooltip-above-trigger",
        (TooltipPlacement::Below, _) => "tooltip-below-trigger",
        (TooltipPlacement::Start, _) => "tooltip-start-trigger",
        (TooltipPlacement::End, _) => "tooltip-end-trigger",
    };

    div()
        .flex()
        .flex_col()
        .items_center()
        .w(px(width))
        .flex_shrink_0()
        .gap(px(if label.is_some() { 10.0 } else { 0.0 }))
        .child(
            div()
                .flex()
                .items_center()
                .when(
                    matches!(placement, TooltipPlacement::Above | TooltipPlacement::Below),
                    |el| el.flex_col(),
                )
                .gap(px(6.))
                .when(matches!(placement, TooltipPlacement::Below), |el| {
                    el.child(tooltip_trigger(trigger_id, placement))
                        .child(Tooltip::new("Export PNG"))
                })
                .when(matches!(placement, TooltipPlacement::Above), |el| {
                    el.child(Tooltip::new("Export PNG"))
                        .child(tooltip_trigger(trigger_id, placement))
                })
                .when(matches!(placement, TooltipPlacement::Start), |el| {
                    el.child(Tooltip::new("Export PNG"))
                        .child(tooltip_trigger(trigger_id, placement))
                })
                .when(matches!(placement, TooltipPlacement::End), |el| {
                    el.child(tooltip_trigger(trigger_id, placement))
                        .child(Tooltip::new("Export PNG"))
                }),
        )
        .when_some(label, |el, label| el.child(caption(theme, label)))
}

fn tooltip_trigger(id: &'static str, placement: TooltipPlacement) -> Button {
    Button::icon_only(id, IconName::Download)
        .variant(ButtonVariant::Ghost)
        .tooltip(Tooltip::new("Export PNG").placement(placement))
}

fn progress_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "PROGRESS",
            "How far, not whether.",
            "Eight tall, a pill. Outline track, primary fill. Linear. Zero, forty, a hundred. No bounce.",
        ))
        .child(section(
            theme,
            "Linear",
            56.0,
            false,
            div()
                .flex()
                .flex_col()
                .gap(px(16.))
                .child(linear_progress_sample(theme, "0", 0.0))
                .child(linear_progress_sample(theme, "40", 0.4))
                .child(linear_progress_sample(theme, "100", 1.0)),
        ))
        .child(section(
            theme,
            "Circular",
            40.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(32.))
                .child(circular_progress_sample(theme, "0", 0.0))
                .child(circular_progress_sample(theme, "40", 0.4))
                .child(circular_progress_sample(theme, "100", 1.0)),
        ))
        .child(section(
            theme,
            "In use",
            40.0,
            true,
            div()
                .flex()
                .flex_col()
                .w(px(332.))
                .gap(px(10.))
                .child(
                    div()
                        .flex()
                        .items_baseline()
                        .justify_between()
                        .child(
                            div()
                                .font_family(theme.font_family)
                                .font_weight(FontWeight::MEDIUM)
                                .text_size(px(14.))
                                .line_height(px(18.))
                                .text_color(theme.ink)
                                .child("Exporting PNG"),
                        )
                        .child(caption(theme, "40%")),
                )
                .child(Progress::new(0.4)),
        ))
}

fn linear_progress_sample(theme: Theme, label: &'static str, value: f32) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(16.))
        .child(
            div()
                .w(px(36.))
                .flex_shrink_0()
                .child(caption(theme, label)),
        )
        .child(Progress::new(value))
}

fn circular_progress_sample(theme: Theme, label: &'static str, value: f32) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .w(px(72.))
        .flex_shrink_0()
        .gap(px(10.))
        .child(CircularProgress::new(value))
        .child(caption(theme, label))
}

fn spinners_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "SPINNER",
            "A quiet mark for waiting.",
            "A single arc on a faint track. Sized for buttons, inline copy, and empty panels.",
        ))
        .child(section(
            theme,
            "Sizes",
            56.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(32.))
                .child(size_sample(theme, SpinnerSize::Small, "Small"))
                .child(size_sample(theme, SpinnerSize::Default, "Default"))
                .child(size_sample(theme, SpinnerSize::Large, "Large"))
                .child(size_sample(theme, SpinnerSize::Display, "Display")),
        ))
        .child(section(
            theme,
            "Color",
            40.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(24.))
                .child(color_sample(theme, SpinnerTone::Default, "Default"))
                .child(color_sample(theme, SpinnerTone::Muted, "Muted"))
                .child(inverse_sample(theme))
                .child(color_sample(theme, SpinnerTone::Destructive, "Destructive")),
        ))
        .child(section(
            theme,
            "With label",
            40.0,
            false,
            div()
                .flex()
                .items_center()
                .gap(px(28.))
                .child(labeled(
                    theme,
                    SpinnerTone::Default,
                    "Saving changes",
                    theme.ink,
                ))
                .child(labeled(
                    theme,
                    SpinnerTone::Muted,
                    "Fetching logs",
                    fetching_label(theme),
                ))
                .child(labeled(
                    theme,
                    SpinnerTone::Destructive,
                    "Deleting project",
                    theme.destructive,
                )),
        ))
        .child(section(
            theme,
            "In use",
            40.0,
            true,
            div()
                .flex()
                .items_center()
                .gap(px(16.))
                .child(Button::new("publish", "Publish").loading(true))
                .child(loading_panel(theme)),
        ))
}

fn size_sample(theme: Theme, size: SpinnerSize, label: &'static str) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .w(px(72.))
        .flex_shrink_0()
        .gap(px(10.))
        .child(Spinner::new().size(size))
        .child(caption(theme, label))
}

fn color_sample(theme: Theme, tone: SpinnerTone, label: &'static str) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .w(px(72.))
        .flex_shrink_0()
        .gap(px(10.))
        .child(Spinner::new().tone(tone))
        .child(caption(theme, label))
}

fn inverse_sample(theme: Theme) -> impl IntoElement {
    let (bg, border, inset, shadow) = if theme.is_dark() {
        (
            paint(0xFFFFFF_85),
            paint(0xFFFFFF_B8),
            paint(0xFFFFFF_E6),
            paint(0x000000_2E),
        )
    } else {
        (
            paint(0x18181B_B8),
            paint(0xFFFFFF_29),
            paint(0xFFFFFF_38),
            paint(0x0F172A_1A),
        )
    };

    div()
        .flex()
        .flex_col()
        .items_center()
        .w(px(72.))
        .flex_shrink_0()
        .gap(px(10.))
        .child(
            div()
                .flex()
                .items_center()
                .justify_center()
                .size(px(40.))
                .flex_shrink_0()
                .rounded(px(6.))
                .border_1()
                .border_color(border)
                .bg(bg)
                .shadow(vec![
                    box_shadow(0., 1., inset, 0., 0.),
                    box_shadow(
                        0.,
                        if theme.is_dark() { 6. } else { 8. },
                        shadow,
                        if theme.is_dark() { 16. } else { 20. },
                        0.,
                    ),
                ])
                .child(Spinner::new().tone(SpinnerTone::Inverse)),
        )
        .child(caption(theme, "Inverse"))
}

fn labeled(
    theme: Theme,
    tone: SpinnerTone,
    text: &'static str,
    color: gpui::Hsla,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .child(Spinner::new().size(SpinnerSize::Small).tone(tone))
        .child(
            div()
                .font_family(theme.font_family)
                .font_weight(FontWeight::MEDIUM)
                .text_size(px(14.))
                .line_height(px(18.))
                .text_color(color)
                .child(text),
        )
}

fn fetching_label(theme: Theme) -> gpui::Hsla {
    if theme.is_dark() {
        theme.body
    } else {
        rgb(0x52525B)
    }
}

fn loading_panel(theme: Theme) -> impl IntoElement {
    let (bg, border, inset, shadow) = if theme.is_dark() {
        (
            paint(0xFFFFFF_12),
            paint(0xFFFFFF_1A),
            paint(0xFFFFFF_1F),
            paint(0x000000_47),
        )
    } else {
        (
            paint(0xFFFFFF_85),
            paint(0xFFFFFF_B8),
            paint(0xFFFFFF_E6),
            paint(0x0F172A_0F),
        )
    };

    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .w(px(280.))
        .h(px(148.))
        .flex_shrink_0()
        .rounded(px(10.))
        .gap(px(12.))
        .border_1()
        .border_color(border)
        .bg(bg)
        .shadow(vec![
            box_shadow(0., 1., inset, 0., 0.),
            box_shadow(0., 6., shadow, 16., 0.),
        ])
        .child(Spinner::new().size(SpinnerSize::Large))
        .child(
            div()
                .font_family(theme.font_family)
                .font_weight(FontWeight::MEDIUM)
                .text_size(px(14.))
                .line_height(px(18.))
                .text_color(theme.body)
                .child("Opening workspace"),
        )
}

fn inputs_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "INPUT",
            "A field for every line of thought.",
            "Single line or many. Outline glass, sized to the medium button.",
        ))
        .child(section(
            theme,
            "Default",
            56.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(16.))
                .child(Input::new("project-placeholder").placeholder("Project name"))
                .child(Input::new("project-filled").value("glassy-ui"))
                .child(labeled_field(
                    Label::new("Email"),
                    Input::new("email").value("hello@studio.dev"),
                )),
        ))
        .child(section(
            theme,
            "With icons",
            40.0,
            false,
            div()
                .flex()
                .items_center()
                .gap(px(16.))
                .child(
                    Input::new("search")
                        .placeholder("Search artboards")
                        .leading_icon(IconName::Search),
                )
                .child(
                    Input::new("search-clear")
                        .value("Buttons")
                        .leading_icon(IconName::Search)
                        .trailing("Clear"),
                ),
        ))
        .child(section(
            theme,
            "States",
            40.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(16.))
                .child(state_sample(
                    theme,
                    "Focus",
                    Input::new("focus").value("glassy-ui").show_focus(true),
                ))
                .child(state_sample(
                    theme,
                    "Disabled",
                    Input::new("disabled-field")
                        .value("Archived project")
                        .disabled(true),
                ))
                .child(state_sample(
                    theme,
                    "Invalid",
                    Input::new("invalid-field")
                        .placeholder("Project name")
                        .invalid(true)
                        .helper("Name is required."),
                )),
        ))
        .child(section(
            theme,
            "Textarea",
            40.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(16.))
                .child(textarea("area-placeholder").placeholder("Write a short description"))
                .child(textarea("area-filled").value(
                    "Quiet zinc glass on a mineral canvas. Keep radius 6, Inter, and no blobs.",
                ))
                .child(
                    textarea("area-invalid")
                        .value("Too short.")
                        .invalid(true)
                        .helper("Description needs at least 20 characters."),
                ),
        ))
        .child(section(
            theme,
            "In use",
            40.0,
            true,
            div()
                .flex()
                .flex_col()
                .w(px(360.))
                .gap(px(16.))
                .child(labeled_field(
                    Label::new("Name").required(true),
                    Input::new("form-name").value("glassy-ui").w(px(360.)),
                ))
                .child(labeled_field(
                    Label::new("Description").optional(true),
                    textarea("form-desc")
                        .value(
                            "Quiet zinc glass on a mineral canvas. Keep radius 6, Inter, and no blobs.",
                        )
                        .w(px(360.)),
                )),
        ))
}

fn labels_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "LABEL",
            "A name for the field.",
            "Thirteen on five hundred. Sits above the control, never inside it. A red mark only when the field is required.",
        ))
        .child(section(
            theme,
            "Default",
            56.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(32.))
                .child(label_sample(theme, Label::new("Email"), "Default", 120.0))
                .child(label_sample(
                    theme,
                    Label::new("Name").required(true),
                    "Required",
                    120.0,
                ))
                .child(label_sample(
                    theme,
                    Label::new("Description").optional(true),
                    "Optional",
                    160.0,
                )),
        ))
        .child(section(
            theme,
            "With field",
            40.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(16.))
                .child(labeled_field(
                    Label::new("Email"),
                    Input::new("label-email").value("hello@studio.dev"),
                ))
                .child(labeled_field(
                    Label::new("Name").required(true),
                    Input::new("label-name").value("glassy-ui"),
                )),
        ))
        .child(section(
            theme,
            "In use",
            40.0,
            true,
            div()
                .flex()
                .flex_col()
                .w(px(360.))
                .gap(px(16.))
                .child(labeled_field(
                    Label::new("Name").required(true),
                    Input::new("label-form-name").value("glassy-ui").w(px(360.)),
                ))
                .child(labeled_field(
                    Label::new("Description").optional(true),
                    textarea("label-form-desc")
                        .value(
                            "Quiet zinc glass on a mineral canvas. Keep radius 6, Inter, and no blobs.",
                        )
                        .w(px(360.)),
                )),
        ))
}

fn checkboxes_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "CHECKBOX",
            "A quiet mark for yes.",
            "Sixteen pixels, radius 6. Outline glass at rest, primary glass when checked. A dash for mixed.",
        ))
        .child(section(
            theme,
            "States",
            56.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(32.))
                .child(check_sample(theme, CheckState::Off, "Unchecked"))
                .child(check_sample(theme, CheckState::On, "Checked"))
                .child(check_sample(theme, CheckState::Mixed, "Mixed")),
        ))
        .child(section(
            theme,
            "Disabled",
            40.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(32.))
                .child(check_sample_disabled(theme, CheckState::Off, "Off"))
                .child(check_sample_disabled(theme, CheckState::On, "On")),
        ))
        .child(section(
            theme,
            "With label",
            40.0,
            false,
            div()
                .flex()
                .items_center()
                .gap(px(32.))
                .child(Checkbox::new("show-grid").label("Show grid"))
                .child(
                    Checkbox::new("snap")
                        .checked(true)
                        .label("Snap to pixel"),
                )
                .child(
                    Checkbox::new("lock")
                        .checked(true)
                        .disabled(true)
                        .label("Lock artboard"),
                ),
        ))
        .child(section(
            theme,
            "In use",
            40.0,
            true,
            div()
                .flex()
                .flex_col()
                .gap(px(12.))
                .child(
                    Checkbox::new("export-all")
                        .state(CheckState::Mixed)
                        .label("Export all"),
                )
                .child(
                    Checkbox::new("export-png")
                        .checked(true)
                        .label("PNG")
                        .pl(px(24.)),
                )
                .child(
                    Checkbox::new("export-svg")
                        .checked(true)
                        .label("SVG")
                        .pl(px(24.)),
                )
                .child(Checkbox::new("export-pdf").label("PDF").pl(px(24.))),
        ))
}

fn switches_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "SWITCH",
            "On, or not.",
            "A 36 by 20 pill. Outline glass off, primary glass on. The thumb moves — it does not bounce.",
        ))
        .child(section(
            theme,
            "States",
            56.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(32.))
                .child(switch_sample(theme, false, false, "Off"))
                .child(switch_sample(theme, true, false, "On")),
        ))
        .child(section(
            theme,
            "Disabled",
            40.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(32.))
                .child(switch_sample(theme, false, true, "Off"))
                .child(switch_sample(theme, true, true, "On")),
        ))
        .child(section(
            theme,
            "With label",
            40.0,
            false,
            div()
                .flex()
                .items_center()
                .gap(px(32.))
                .child(Switch::new("show-grid-sw").label("Show grid"))
                .child(Switch::new("snap-sw").on(true).label("Snap to pixel"))
                .child(
                    Switch::new("reduce-sw")
                        .on(true)
                        .disabled(true)
                        .label("Reduce motion"),
                ),
        ))
        .child(section(
            theme,
            "In use",
            40.0,
            true,
            div()
                .flex()
                .flex_col()
                .w(px(360.))
                .child(settings_row(theme, "Auto-save", false, Switch::new("auto-save").on(true)))
                .child(settings_row(theme, "Show rulers", false, Switch::new("rulers")))
                .child(settings_row(
                    theme,
                    "Reduce motion",
                    true,
                    Switch::new("reduce-settings").on(true).disabled(true),
                )),
        ))
}

fn radios_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "RADIO",
            "One of these.",
            "Sixteen pixels, a circle. Outline glass at rest, primary glass when selected. A filled dot marks the choice.",
        ))
        .child(section(
            theme,
            "States",
            56.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(32.))
                .child(radio_sample(theme, false, false, "Unselected"))
                .child(radio_sample(theme, true, false, "Selected")),
        ))
        .child(section(
            theme,
            "Disabled",
            40.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(32.))
                .child(radio_sample(theme, false, true, "Unselected"))
                .child(radio_sample(theme, true, true, "Selected")),
        ))
        .child(section(
            theme,
            "With label",
            40.0,
            false,
            div()
                .flex()
                .items_center()
                .gap(px(28.))
                .child(Radio::new("fit").group("radio-label").label("Fit"))
                .child(
                    Radio::new("fill")
                        .group("radio-label")
                        .selected(true)
                        .label("Fill"),
                )
                .child(
                    Radio::new("stretch")
                        .group("radio-label")
                        .disabled(true)
                        .label("Stretch"),
                ),
        ))
        .child(section(
            theme,
            "In use",
            40.0,
            true,
            div()
                .flex()
                .flex_col()
                .gap(px(10.))
                .child(
                    Radio::new("export-png")
                        .group("radio-export")
                        .selected(true)
                        .label("PNG"),
                )
                .child(Radio::new("export-svg").group("radio-export").label("SVG"))
                .child(
                    Radio::new("export-pdf")
                        .group("radio-export")
                        .disabled(true)
                        .label("PDF"),
                ),
        ))
}

fn selects_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    let formats = export_formats();

    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "SELECT",
            "Pick from a list.",
            "Closed field is Input chrome plus a chevron. The open list is secondary glass, radius 6. Arrows, enter, esc.",
        ))
        .child(section(
            theme,
            "Closed",
            56.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(16.))
                .child(Select::new("closed-placeholder").items(formats.clone()))
                .child(
                    Select::new("closed-filled")
                        .value("PNG")
                        .items(formats.clone()),
                )
                .child(labeled_select(
                    Label::new("Export"),
                    Select::new("closed-labeled")
                        .value("PNG")
                        .items(formats.clone()),
                )),
        ))
        .child(section(
            theme,
            "States",
            40.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(16.))
                .child(state_select(
                    theme,
                    "Focus",
                    Select::new("state-focus")
                        .value("PNG")
                        .focused(true)
                        .items(formats.clone()),
                ))
                .child(state_select(
                    theme,
                    "Disabled",
                    Select::new("state-disabled")
                        .value("Archived project")
                        .disabled(true)
                        .items(formats.clone()),
                )),
        ))
        .child(section(
            theme,
            "Open",
            40.0,
            false,
            Select::new("open")
                .value("PNG")
                .focused(true)
                .open(true)
                .items(formats.clone()),
        ))
        .child(section(
            theme,
            "In use",
            40.0,
            true,
            labeled_select(
                Label::new("Export format"),
                Select::new("in-use").value("PNG").items(formats),
            ),
        ))
}

fn kbds_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "KBD",
            "A chip for a key.",
            "Ghost glass, twelve over five hundred. For menus and hints — not a button.",
        ))
        .child(section(
            theme,
            "Keys",
            56.0,
            false,
            div()
                .flex()
                .items_center()
                .gap(px(12.))
                .child(Kbd::new("⌘K"))
                .child(Kbd::new("⌘Q"))
                .child(Kbd::new("Esc")),
        ))
        .child(section(
            theme,
            "In a hint",
            40.0,
            false,
            div()
                .flex()
                .items_center()
                .gap(px(8.))
                .child(
                    div()
                        .font_family(theme.font_family)
                        .font_weight(FontWeight::NORMAL)
                        .text_size(px(14.))
                        .line_height(px(18.))
                        .text_color(theme.body)
                        .child("Search artboards"),
                )
                .child(Kbd::new("⌘K")),
        ))
        .child(section(
            theme,
            "In a menu",
            40.0,
            true,
            div()
                .flex()
                .flex_col()
                .w(px(360.))
                .child(kbd_menu_row(theme, "New file", "⌘N"))
                .child(kbd_menu_row(theme, "Quit", "⌘Q")),
        ))
}

fn kbd_menu_row(theme: Theme, label: &'static str, keys: &'static str) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .justify_between()
        .w(px(360.))
        .h(px(36.))
        .flex_shrink_0()
        .child(
            div()
                .font_family(theme.font_family)
                .font_weight(FontWeight::NORMAL)
                .text_size(px(14.))
                .line_height(px(18.))
                .text_color(theme.ink)
                .child(label),
        )
        .child(Kbd::new(keys))
}

fn separators_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "SEPARATOR",
            "A quiet cut.",
            "One pixel of zinc at twelve percent. Horizontal or vertical. Not a black rule.",
        ))
        .child(section(
            theme,
            "Horizontal",
            56.0,
            false,
            Separator::horizontal(),
        ))
        .child(section(
            theme,
            "Vertical",
            40.0,
            false,
            Separator::vertical(),
        ))
        .child(section(
            theme,
            "In use",
            40.0,
            true,
            div()
                .flex()
                .flex_col()
                .gap(px(16.))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .w(px(280.))
                        .flex_shrink_0()
                        .child(
                            div()
                                .font_family(theme.font_family)
                                .font_weight(FontWeight::MEDIUM)
                                .text_size(px(14.))
                                .line_height(px(18.))
                                .text_color(theme.ink)
                                .child("Export"),
                        )
                        .child(Separator::horizontal())
                        .child(
                            div()
                                .font_family(theme.font_family)
                                .font_weight(FontWeight::NORMAL)
                                .text_size(px(14.))
                                .line_height(px(20.))
                                .text_color(theme.body)
                                .child("PNG, SVG, or PDF. One format at a time."),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(12.))
                        .h(px(36.))
                        .flex_shrink_0()
                        .child(separator_tool(theme, "Fit", theme.ink))
                        .child(Separator::vertical().h(px(16.)))
                        .child(separator_tool(theme, "Fill", theme.ink))
                        .child(Separator::vertical().h(px(16.)))
                        .child(separator_tool(theme, "Stretch", theme.muted_fg())),
                ),
        ))
}

fn badges_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "BADGE",
            "A count, not a click.",
            "Twenty-two tall, radius 6. Primary, ghost, or red. Status and counts — never an action.",
        ))
        .child(section(
            theme,
            "Variants",
            56.0,
            false,
            div()
                .flex()
                .items_end()
                .gap(px(32.))
                .child(badge_sample(
                    theme,
                    "Badge",
                    BadgeVariant::Default,
                    "Default",
                ))
                .child(badge_sample(
                    theme,
                    "Badge",
                    BadgeVariant::Muted,
                    "Muted",
                ))
                .child(badge_sample(
                    theme,
                    "Badge",
                    BadgeVariant::Destructive,
                    "Destructive",
                )),
        ))
        .child(section(
            theme,
            "Counts",
            40.0,
            false,
            div()
                .flex()
                .items_center()
                .gap(px(12.))
                .child(Badge::new("3"))
                .child(Badge::new("12"))
                .child(Badge::new("128")),
        ))
        .child(section(
            theme,
            "In use",
            40.0,
            true,
            div()
                .flex()
                .items_center()
                .gap(px(32.))
                .child(badge_usage(
                    theme,
                    "Artboards",
                    Badge::new("12").variant(BadgeVariant::Muted),
                ))
                .child(badge_usage(
                    theme,
                    "Failed",
                    Badge::new("3").variant(BadgeVariant::Destructive),
                )),
        ))
}

fn dialogs_page(
    theme: Theme,
    page: GalleryPage,
    dialog_open: bool,
    gallery: gpui::Entity<Gallery>,
) -> impl IntoElement {
    let open_gallery = gallery.clone();
    let dismiss_gallery = gallery.clone();
    let cancel_gallery = gallery.clone();
    let confirm_gallery = gallery;
    let scrim = if theme.is_dark() {
        paint(0x00000073)
    } else {
        paint(0x18181B47)
    };

    let panel = rename_dialog_content(
        "dialog-panel",
        Button::new("dialog-panel-cancel", "Cancel").variant(ButtonVariant::Ghost),
        Button::new("dialog-panel-confirm", "Rename").on_click(move |_, _, cx| {
            open_gallery.update(cx, |gallery, cx| {
                gallery.dialog_open = true;
                cx.notify();
            });
        }),
    );

    let overlay_scene = div()
        .flex()
        .items_center()
        .justify_center()
        .w(px(560.))
        .h(px(360.))
        .flex_shrink_0()
        .rounded(px(10.))
        .overflow_hidden()
        .bg(scrim)
        .child(rename_dialog_content(
            "dialog-scene",
            Button::new("dialog-scene-cancel", "Cancel").variant(ButtonVariant::Ghost),
            Button::new("dialog-scene-confirm", "Rename"),
        ));

    let live_dialog = Dialog::new("rename-dialog")
        .open(dialog_open)
        .on_dismiss(move |_, cx| {
            dismiss_gallery.update(cx, |gallery, cx| {
                gallery.dialog_open = false;
                cx.notify();
            });
        })
        .child(rename_dialog_content(
            "dialog-live",
            Button::new("dialog-live-cancel", "Cancel")
                .variant(ButtonVariant::Ghost)
                .on_click(move |_, _, cx| {
                    cancel_gallery.update(cx, |gallery, cx| {
                        gallery.dialog_open = false;
                        cx.notify();
                    });
                }),
            Button::new("dialog-live-confirm", "Rename").on_click(move |_, _, cx| {
                confirm_gallery.update(cx, |gallery, cx| {
                    gallery.dialog_open = false;
                    cx.notify();
                });
            }),
        ));

    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "DIALOG",
            "A question in the middle.",
            "Radius 10 panel. A dim scrim, not a second glass. Ghost cancel, primary confirm. Esc and overlay dismiss.",
        ))
        .child(section(theme, "Panel", 56.0, false, panel))
        .child(section(theme, "Overlay", 40.0, true, overlay_scene))
        .child(live_dialog)
}

fn rename_dialog_content(id: &'static str, cancel: Button, confirm: Button) -> DialogContent {
    DialogContent::new()
        .child(
            DialogHeader::new()
                .child(DialogTitle::new("Rename artboard"))
                .child(DialogDescription::new(
                    "The page keeps the same ID. Only the label changes.",
                )),
        )
        .child(Input::new(format!("{id}-field")).value("Home").w(px(352.)))
        .child(DialogFooter::new().child(cancel).child(confirm))
}

fn alert_dialogs_page(
    theme: Theme,
    page: GalleryPage,
    alert_dialog_open: bool,
    gallery: gpui::Entity<Gallery>,
) -> impl IntoElement {
    let open_gallery = gallery.clone();
    let cancel_gallery = gallery.clone();
    let confirm_gallery = gallery;
    let scrim = if theme.is_dark() {
        paint(0x00000073)
    } else {
        paint(0x18181B47)
    };

    let panel = delete_alert_content(
        Button::new("alert-panel-cancel", "Cancel").variant(ButtonVariant::Ghost),
        Button::new("alert-panel-confirm", "Delete page")
            .variant(ButtonVariant::Destructive)
            .on_click(move |_, _, cx| {
                open_gallery.update(cx, |gallery, cx| {
                    gallery.alert_dialog_open = true;
                    cx.notify();
                });
            }),
    );

    let overlay_scene = div()
        .flex()
        .items_center()
        .justify_center()
        .w(px(560.))
        .h(px(360.))
        .flex_shrink_0()
        .rounded(px(10.))
        .overflow_hidden()
        .bg(scrim)
        .child(delete_alert_content(
            Button::new("alert-scene-cancel", "Cancel").variant(ButtonVariant::Ghost),
            Button::new("alert-scene-confirm", "Delete page").variant(ButtonVariant::Destructive),
        ));

    let live_alert = AlertDialog::new(
        "delete-page-alert",
        "Delete this page?",
        "Home and everything on it are gone. This cannot be undone.",
    )
    .open(alert_dialog_open)
    .confirm_label("Delete page")
    .on_cancel(move |_, cx| {
        cancel_gallery.update(cx, |gallery, cx| {
            gallery.alert_dialog_open = false;
            cx.notify();
        });
    })
    .on_confirm(move |_, cx| {
        confirm_gallery.update(cx, |gallery, cx| {
            gallery.alert_dialog_open = false;
            cx.notify();
        });
    });

    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "ALERT DIALOG",
            "Say what it does.",
            "Destructive confirm. The title is the action. No field. Esc still dismisses.",
        ))
        .child(section(theme, "Panel", 56.0, false, panel))
        .child(section(theme, "Overlay", 40.0, true, overlay_scene))
        .child(live_alert)
}

fn delete_alert_content(cancel: Button, confirm: Button) -> DialogContent {
    DialogContent::new()
        .child(
            DialogHeader::new()
                .child(DialogTitle::new("Delete this page?"))
                .child(DialogDescription::new(
                    "Home and everything on it are gone. This cannot be undone.",
                )),
        )
        .child(DialogFooter::new().child(cancel).child(confirm))
}

fn popovers_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "POPOVER",
            "From the source.",
            "Secondary glass, radius 6. Origin is the trigger. Same path in and out.",
        ))
        .child(section(theme, "Panel", 56.0, false, page_meta_popover()))
        .child(section(
            theme,
            "Anchored",
            40.0,
            true,
            Popover::new("page-meta-popover")
                .default_open(true)
                .trigger_label("Show page metadata")
                .trigger(popover_info_trigger(theme))
                .child(page_meta_popover()),
        ))
}

fn dropdown_menus_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "DROPDOWN",
            "A list with keys.",
            "Popover plus items. Default, destructive, disabled, shortcut, separator, nested. Arrows.",
        ))
        .child(section(
            theme,
            "Menu",
            56.0,
            false,
            DropdownMenu::new("dropdown-specimen")
                .open(true)
                .entries(file_menu_entries()),
        ))
        .child(section(
            theme,
            "In use",
            40.0,
            true,
            DropdownMenu::new("file-menu")
                .default_open(true)
                .trigger_label("Open File menu")
                .trigger(dropdown_file_trigger(theme))
                .entries(file_menu_entries()),
        ))
}

fn file_menu_entries() -> Vec<DropdownMenuEntry> {
    vec![
        DropdownMenuItem::new("New file").shortcut("⌘N").into(),
        DropdownMenuItem::new("Duplicate").into(),
        DropdownMenuItem::new("Export")
            .submenu([
                DropdownMenuEntry::item("PNG"),
                DropdownMenuEntry::item("SVG"),
            ])
            .into(),
        DropdownMenuEntry::separator(),
        DropdownMenuItem::new("Export PDF").disabled(true).into(),
        DropdownMenuItem::new("Delete page")
            .destructive(true)
            .into(),
    ]
}

fn context_menus_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "CONTEXT",
            "At the pointer.",
            "Same items as Dropdown. Opens where you click. Escape and outside click dismiss.",
        ))
        .child(section(
            theme,
            "Menu",
            56.0,
            false,
            div()
                .relative()
                .w(px(240.))
                .h(px(248.))
                .flex_shrink_0()
                .child(
                    ContextMenu::new("context-specimen")
                        .open(true)
                        .position(point(px(0.), px(0.)))
                        .entries(file_menu_entries()),
                ),
        ))
        .child(section(
            theme,
            "In use",
            40.0,
            true,
            ContextMenu::new("page-context")
                .default_open(true)
                .position(point(px(48.), px(40.)))
                .entries(file_menu_entries())
                .child(context_target(theme)),
        ))
}

fn command_page(theme: Theme, page: GalleryPage) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(header(
            theme,
            page,
            "COMMAND",
            "Jump.",
            "⌘K. Input on top, grouped rows. Spinner while loading. Empty when nothing matches.",
        ))
        .child(section(
            theme,
            "Palette",
            56.0,
            false,
            Command::new("command-specimen")
                .default_query("Home")
                .filtering(false)
                .groups(command_specimen_groups()),
        ))
        .child(section(
            theme,
            "States",
            40.0,
            true,
            div()
                .flex()
                .items_start()
                .gap(px(32.))
                .child(command_state_sample(
                    theme,
                    "Loading",
                    Command::new("command-loading")
                        .size(CommandSize::Compact)
                        .default_query("Home")
                        .loading(true)
                        .loading_label("Searching pages")
                        .show_footer(false),
                ))
                .child(command_state_sample(
                    theme,
                    "Empty",
                    Command::new("command-empty")
                        .size(CommandSize::Compact)
                        .default_query("xyzzy")
                        .empty_label("No pages match.")
                        .show_footer(false),
                )),
        ))
}

fn command_specimen_groups() -> Vec<CommandGroup> {
    vec![
        CommandGroup::new("Pages").items([
            CommandItem::new("home", "Home"),
            CommandItem::new("buttons", "Buttons"),
        ]),
        CommandGroup::new("Actions").item(CommandItem::new("new-file", "New file").shortcut("⌘N")),
    ]
}

fn gallery_command(gallery: gpui::Entity<Gallery>, focus_handle: FocusHandle) -> Command {
    let home_gallery = gallery.clone();
    let buttons_gallery = gallery.clone();
    let command_gallery = gallery.clone();
    let dismiss_gallery = gallery;

    Command::new("gallery-command")
        .placeholder("Search pages or actions…")
        .focus_handle(focus_handle)
        .groups([
            CommandGroup::new("Pages").items([
                CommandItem::new("home", "Home").on_select(move |_, window, cx| {
                    home_gallery.update(cx, |gallery, cx| {
                        gallery.set_page(GalleryPage::Inputs, window, cx)
                    });
                }),
                CommandItem::new("buttons", "Buttons").on_select(move |_, window, cx| {
                    buttons_gallery.update(cx, |gallery, cx| {
                        gallery.set_page(GalleryPage::Buttons, window, cx)
                    });
                }),
                CommandItem::new("command", "Command").on_select(move |_, window, cx| {
                    command_gallery.update(cx, |gallery, cx| {
                        gallery.set_page(GalleryPage::Command, window, cx)
                    });
                }),
            ]),
            CommandGroup::new("Actions")
                .item(CommandItem::new("new-file", "New file").shortcut("⌘N")),
        ])
        .on_dismiss(move |_, cx| {
            dismiss_gallery.update(cx, |gallery, cx| {
                gallery.command_open = false;
                cx.notify();
            });
        })
}

fn command_state_sample(theme: Theme, label: &'static str, command: Command) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w(px(280.))
        .flex_shrink_0()
        .gap(px(10.))
        .child(caption(theme, label))
        .child(command)
}

fn context_target(theme: Theme) -> impl IntoElement {
    let chrome = if theme.is_dark() {
        (paint(0xFFFFFF12), paint(0xFFFFFF1A), paint(0xFFFFFF1F))
    } else {
        (paint(0xFFFFFF85), paint(0xFFFFFFB8), paint(0xFFFFFFE6))
    };

    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(8.))
        .w(px(360.))
        .h(px(148.))
        .flex_shrink_0()
        .rounded(px(10.))
        .border_1()
        .border_color(chrome.1)
        .bg(chrome.0)
        .shadow(vec![box_shadow(0., 1., chrome.2, 0., 0.)])
        .child(
            div()
                .font_family(theme.font_family)
                .font_weight(FontWeight::SEMIBOLD)
                .text_size(px(16.))
                .line_height(px(20.))
                .text_color(theme.heading)
                .child("Home"),
        )
        .child(
            div()
                .font_family(theme.font_family)
                .font_weight(FontWeight::MEDIUM)
                .text_size(px(13.))
                .line_height(px(16.))
                .text_color(theme.label)
                .child("Right-click"),
        )
}

fn dropdown_file_trigger(theme: Theme) -> impl IntoElement {
    let (background, hover, border, inset) = if theme.is_dark() {
        (
            paint(0xFFFFFF08),
            paint(0xFFFFFF12),
            paint(0xFFFFFF0F),
            paint(0xFFFFFF0F),
        )
    } else {
        (
            paint(0xFFFFFF29),
            paint(0xFFFFFF3D),
            paint(0xFFFFFF47),
            paint(0xFFFFFF66),
        )
    };

    div()
        .flex()
        .items_center()
        .h(px(36.))
        .flex_shrink_0()
        .px(px(16.))
        .rounded(px(6.))
        .border_1()
        .border_color(border)
        .bg(background)
        .shadow(vec![box_shadow(0., 1., inset, 0., 0.)])
        .hover(move |style| style.bg(hover))
        .font_family(theme.font_family)
        .font_weight(FontWeight::MEDIUM)
        .text_size(px(14.))
        .line_height(px(18.))
        .text_color(theme.ink)
        .child("File")
}

fn page_meta_popover() -> PopoverContent {
    PopoverContent::new()
        .child(PopoverTitle::new("Home"))
        .child(PopoverDescription::new("1440 × 900 · 3 layers"))
}

fn popover_info_trigger(theme: Theme) -> impl IntoElement {
    let (background, hover, border, inset) = if theme.is_dark() {
        (
            paint(0xFFFFFF08),
            paint(0xFFFFFF12),
            paint(0xFFFFFF0F),
            paint(0xFFFFFF0F),
        )
    } else {
        (
            paint(0xFFFFFF29),
            paint(0xFFFFFF3D),
            paint(0xFFFFFF47),
            paint(0xFFFFFF66),
        )
    };

    div()
        .flex()
        .items_center()
        .justify_center()
        .size(px(36.))
        .flex_shrink_0()
        .rounded(px(6.))
        .border_1()
        .border_color(border)
        .bg(background)
        .shadow(vec![box_shadow(0., 1., inset, 0., 0.)])
        .hover(move |style| style.bg(hover))
        .child(Icon::new(IconName::Info).px(px(16.)).color(theme.ink))
}

fn badge_sample(
    theme: Theme,
    label: &'static str,
    variant: BadgeVariant,
    caption_text: &'static str,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .w(px(72.))
        .flex_shrink_0()
        .gap(px(10.))
        .child(Badge::new(label).variant(variant))
        .child(caption(theme, caption_text))
}

fn badge_usage(theme: Theme, label: &'static str, badge: Badge) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .child(
            div()
                .font_family(theme.font_family)
                .font_weight(FontWeight::NORMAL)
                .text_size(px(14.))
                .line_height(px(18.))
                .text_color(theme.ink)
                .child(label),
        )
        .child(badge)
}

fn separator_tool(theme: Theme, label: &'static str, color: gpui::Hsla) -> impl IntoElement {
    div()
        .font_family(theme.font_family)
        .font_weight(FontWeight::MEDIUM)
        .text_size(px(14.))
        .line_height(px(18.))
        .text_color(color)
        .child(label)
}

fn labeled_field(label: Label, field: Input) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(8.))
        .flex_shrink_0()
        .child(label)
        .child(field)
}

fn labeled_select(label: Label, field: Select) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(8.))
        .w(px(280.))
        .flex_shrink_0()
        .child(label)
        .child(field)
}

fn state_select(theme: Theme, caption: &'static str, field: Select) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(10.))
        .w(px(280.))
        .flex_shrink_0()
        .child(field)
        .child(self::caption(theme, caption))
}

fn export_formats() -> Vec<SelectItem> {
    vec![
        SelectItem::new("PNG"),
        SelectItem::new("SVG"),
        SelectItem::new("PDF").disabled(true),
    ]
}

fn state_sample(theme: Theme, caption: &'static str, field: Input) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(10.))
        .w(px(280.))
        .flex_shrink_0()
        .child(field)
        .child(self::caption(theme, caption))
}

fn label_sample(theme: Theme, label: Label, caption: &'static str, width: f32) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .w(px(width))
        .flex_shrink_0()
        .gap(px(10.))
        .child(label)
        .child(self::caption(theme, caption))
}

fn check_sample(theme: Theme, state: CheckState, caption: &'static str) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .w(px(72.))
        .flex_shrink_0()
        .gap(px(10.))
        .child(Checkbox::new(caption).state(state))
        .child(self::caption(theme, caption))
}

fn check_sample_disabled(
    theme: Theme,
    state: CheckState,
    caption: &'static str,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .w(px(72.))
        .flex_shrink_0()
        .gap(px(10.))
        .child(
            Checkbox::new(format!("disabled-{caption}"))
                .state(state)
                .disabled(true),
        )
        .child(self::caption(theme, caption))
}

fn switch_sample(
    theme: Theme,
    on: bool,
    disabled: bool,
    caption: &'static str,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .w(px(72.))
        .flex_shrink_0()
        .gap(px(10.))
        .child(
            Switch::new(format!("sw-{caption}-{on}-{disabled}"))
                .on(on)
                .disabled(disabled),
        )
        .child(self::caption(theme, caption))
}

fn settings_row(
    theme: Theme,
    label: &'static str,
    muted: bool,
    control: Switch,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .justify_between()
        .w(px(360.))
        .h(px(36.))
        .flex_shrink_0()
        .child(
            div()
                .font_family(theme.font_family)
                .font_weight(FontWeight::NORMAL)
                .text_size(px(14.))
                .line_height(px(20.))
                .text_color(if muted { theme.muted_fg() } else { theme.ink })
                .child(label),
        )
        .child(control)
}

fn radio_sample(
    theme: Theme,
    selected: bool,
    disabled: bool,
    caption: &'static str,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .w(px(72.))
        .flex_shrink_0()
        .gap(px(10.))
        .child(
            Radio::new(format!("radio-{caption}-{selected}-{disabled}"))
                .selected(selected)
                .disabled(disabled),
        )
        .child(self::caption(theme, caption))
}

fn caption(theme: Theme, text: &'static str) -> impl IntoElement {
    div()
        .font_family(theme.font_family)
        .font_weight(FontWeight::MEDIUM)
        .text_size(px(13.))
        .line_height(px(16.))
        .text_color(theme.label)
        .child(text)
}

fn header(
    theme: Theme,
    page: GalleryPage,
    eyebrow: &'static str,
    title: &'static str,
    body: &'static str,
) -> impl IntoElement {
    let next_theme = if theme.is_dark() { "Light" } else { "Dark" };
    let next_page = page.next_label();

    div()
        .flex()
        .items_start()
        .justify_between()
        .pt(px(72.))
        .px(px(80.))
        .gap(px(24.))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(10.))
                .min_w(px(0.))
                .flex_1()
                .child(
                    div()
                        .font_family(theme.font_family)
                        .font_weight(FontWeight::MEDIUM)
                        .text_size(px(13.))
                        .line_height(px(16.))
                        .text_color(theme.label)
                        .child(eyebrow),
                )
                .child(
                    div()
                        .font_family(theme.font_family)
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_size(px(36.))
                        .line_height(px(40.))
                        .text_color(theme.heading)
                        .child(title),
                )
                .child(
                    div()
                        .font_family(theme.font_family)
                        .font_weight(FontWeight::NORMAL)
                        .text_size(px(15.))
                        .line_height(px(24.))
                        .text_color(theme.body)
                        .max_w(px(460.))
                        .child(body),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(8.))
                .child(
                    Button::new("switch-page", next_page)
                        .variant(ButtonVariant::Ghost)
                        .on_click(move |_, window, cx| {
                            window.dispatch_action(Box::new(ShowNextPage), cx)
                        }),
                )
                .child(
                    Button::new("toggle-theme", next_theme)
                        .variant(ButtonVariant::Secondary)
                        .on_click(|_, _, cx| cx.toggle_theme()),
                ),
        )
}

fn section(
    theme: Theme,
    title: &'static str,
    pad_top: f32,
    last: bool,
    row: impl IntoElement,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .pt(px(pad_top))
        .when(last, |el| el.pb(px(80.)))
        .px(px(80.))
        .gap(px(16.))
        .child(
            div()
                .font_family(theme.font_family)
                .font_weight(FontWeight::MEDIUM)
                .text_size(px(13.))
                .line_height(px(16.))
                .text_color(theme.label)
                .child(title),
        )
        .child(row)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{size, TestAppContext, VisualTestContext};

    fn setup(cx: &mut TestAppContext) -> VisualTestContext {
        cx.update(|cx| {
            init_motion(cx);
            init_theme(cx);
            init_ui(cx);
            cx.bind_keys([
                KeyBinding::new("cmd-k", ToggleCommand, None),
                KeyBinding::new("ctrl-k", ToggleCommand, None),
                KeyBinding::new("cmd-]", ShowNextPage, None),
                KeyBinding::new("ctrl-]", ShowNextPage, None),
            ]);
        });
        let window = cx.add_window(Gallery::new);
        cx.simulate_window_resize(window.into(), size(px(800.), px(700.)));
        cx.run_until_parked();
        VisualTestContext::from_window(window.into(), cx)
    }

    #[gpui::test]
    fn command_shortcut_filters_selects_and_closes(cx: &mut TestAppContext) {
        let mut cx = setup(cx);
        assert!(cx.debug_bounds("gallery-command-dialog-overlay").is_none());

        cx.simulate_keystrokes("cmd-k");
        assert!(cx.debug_bounds("gallery-command-dialog-overlay").is_some());
        cx.simulate_input("but");
        cx.simulate_keystrokes("enter");

        assert_eq!(cx.window_title().as_deref(), Some("Glassy UI — Buttons"));
    }

    #[gpui::test]
    fn escape_closes_command_and_restores_gallery_focus(cx: &mut TestAppContext) {
        let mut cx = setup(cx);
        cx.simulate_keystrokes("cmd-k escape");
        assert!(cx.debug_bounds("gallery-command-dialog-overlay").is_none());

        cx.simulate_keystrokes("cmd-]");
        assert_eq!(cx.window_title().as_deref(), Some("Glassy UI — Labels"));
    }
}
