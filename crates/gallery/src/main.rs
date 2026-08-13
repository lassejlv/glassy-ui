//! Paper hex+alpha is grouped as `RRGGBB_AA`.
#![allow(clippy::unusual_byte_groupings)]

use gpui::{
    actions, div, prelude::*, px, size, App, Bounds, BoxShadow, FocusHandle, Focusable, FontWeight,
    KeyBinding, Menu, MenuItem, QuitMode, TitlebarOptions, Window, WindowBackgroundAppearance,
    WindowBounds, WindowOptions,
};
use gpui_kit_ui::{
    init as init_ui, init_motion, init_theme, load_fonts, paint, rgb, textarea, ActiveTheme,
    Assets, Button, ButtonGroup, ButtonSize, ButtonVariant, CheckState, Checkbox, IconName, Input,
    Kbd, Label, Radio, Select, SelectItem, Separator, Spinner, SpinnerSize, SpinnerTone, Switch,
    Theme,
};
use gpui_platform::application;

actions!(
    gpui_kit_gallery,
    [
        Quit,
        ToggleTheme,
        ShowButtons,
        ShowSpinners,
        ShowInputs,
        ShowLabels,
        ShowCheckboxes,
        ShowSwitches,
        ShowRadios,
        ShowSelects,
        ShowKbds,
        ShowSeparators,
        ShowNextPage,
    ]
);

fn main() {
    application()
        .with_assets(Assets)
        .with_quit_mode(QuitMode::LastWindowClosed)
        .run(|cx: &mut App| {
            init_motion(cx);
            init_theme(cx);
            init_ui(cx);
            load_fonts(cx).expect("register Inter");

            cx.set_app_identity("dev.gpui-kit.gallery", "GPUI Kit");
            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.on_action(|_: &ToggleTheme, cx| cx.toggle_theme());
            cx.bind_keys([
                KeyBinding::new("cmd-q", Quit, None),
                KeyBinding::new("ctrl-q", Quit, None),
                KeyBinding::new("cmd-d", ToggleTheme, None),
                KeyBinding::new("ctrl-d", ToggleTheme, None),
                KeyBinding::new("cmd-1", ShowButtons, None),
                KeyBinding::new("ctrl-1", ShowButtons, None),
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
                KeyBinding::new("cmd-]", ShowNextPage, None),
                KeyBinding::new("ctrl-]", ShowNextPage, None),
            ]);
            cx.set_menus([Menu::new("GPUI Kit").items([
                MenuItem::action("Buttons", ShowButtons),
                MenuItem::action("Spinners", ShowSpinners),
                MenuItem::action("Inputs", ShowInputs),
                MenuItem::action("Labels", ShowLabels),
                MenuItem::action("Checkboxes", ShowCheckboxes),
                MenuItem::action("Switches", ShowSwitches),
                MenuItem::action("Radios", ShowRadios),
                MenuItem::action("Selects", ShowSelects),
                MenuItem::action("Kbd", ShowKbds),
                MenuItem::action("Separators", ShowSeparators),
                MenuItem::separator(),
                MenuItem::action("Toggle Light / Dark", ToggleTheme),
                MenuItem::separator(),
                MenuItem::action("Quit GPUI Kit", Quit),
            ])]);

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
            app_id: Some("dev.gpui-kit.gallery".into()),
            titlebar: Some(TitlebarOptions {
                title: Some("GPUI Kit — Inputs".into()),
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
    Spinners,
    Inputs,
    Labels,
    Checkboxes,
    Switches,
    Radios,
    Selects,
    Kbds,
    Separators,
}

impl GalleryPage {
    fn title(self) -> &'static str {
        match self {
            Self::Buttons => "GPUI Kit — Buttons",
            Self::Spinners => "GPUI Kit — Spinners",
            Self::Inputs => "GPUI Kit — Inputs",
            Self::Labels => "GPUI Kit — Labels",
            Self::Checkboxes => "GPUI Kit — Checkboxes",
            Self::Switches => "GPUI Kit — Switches",
            Self::Radios => "GPUI Kit — Radios",
            Self::Selects => "GPUI Kit — Selects",
            Self::Kbds => "GPUI Kit — Kbd",
            Self::Separators => "GPUI Kit — Separators",
        }
    }

    fn next_label(self) -> &'static str {
        self.next().short_name()
    }

    fn short_name(self) -> &'static str {
        match self {
            Self::Buttons => "Buttons",
            Self::Spinners => "Spinners",
            Self::Inputs => "Inputs",
            Self::Labels => "Labels",
            Self::Checkboxes => "Checkboxes",
            Self::Switches => "Switches",
            Self::Radios => "Radios",
            Self::Selects => "Selects",
            Self::Kbds => "Kbd",
            Self::Separators => "Separators",
        }
    }

    fn next(self) -> Self {
        match self {
            Self::Buttons => Self::Spinners,
            Self::Spinners => Self::Inputs,
            Self::Inputs => Self::Labels,
            Self::Labels => Self::Checkboxes,
            Self::Checkboxes => Self::Switches,
            Self::Switches => Self::Radios,
            Self::Radios => Self::Selects,
            Self::Selects => Self::Kbds,
            Self::Kbds => Self::Separators,
            Self::Separators => Self::Buttons,
        }
    }
}

struct Gallery {
    focus_handle: FocusHandle,
    page: GalleryPage,
}

impl Gallery {
    fn new(window: &mut Window, cx: &mut gpui::Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);
        Self {
            focus_handle,
            page: GalleryPage::Inputs,
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
            .on_action(cx.listener(Self::show_spinners))
            .on_action(cx.listener(Self::show_inputs))
            .on_action(cx.listener(Self::show_labels))
            .on_action(cx.listener(Self::show_checkboxes))
            .on_action(cx.listener(Self::show_switches))
            .on_action(cx.listener(Self::show_radios))
            .on_action(cx.listener(Self::show_selects))
            .on_action(cx.listener(Self::show_kbds))
            .on_action(cx.listener(Self::show_separators))
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
                        GalleryPage::Spinners => spinners_page(theme, page).into_any_element(),
                        GalleryPage::Inputs => inputs_page(theme, page).into_any_element(),
                        GalleryPage::Labels => labels_page(theme, page).into_any_element(),
                        GalleryPage::Checkboxes => checkboxes_page(theme, page).into_any_element(),
                        GalleryPage::Switches => switches_page(theme, page).into_any_element(),
                        GalleryPage::Radios => radios_page(theme, page).into_any_element(),
                        GalleryPage::Selects => selects_page(theme, page).into_any_element(),
                        GalleryPage::Kbds => kbds_page(theme, page).into_any_element(),
                        GalleryPage::Separators => separators_page(theme, page).into_any_element(),
                    }),
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
                    BoxShadow::new(px(0.), px(1.), inset).inset(),
                    BoxShadow::new(px(0.), px(if theme.is_dark() { 6. } else { 8. }), shadow)
                        .blur_radius(px(if theme.is_dark() { 16. } else { 20. })),
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
            BoxShadow::new(px(0.), px(1.), inset).inset(),
            BoxShadow::new(px(0.), px(6.), shadow).blur_radius(px(16.)),
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
                .child(Input::new("project-filled").value("GPUI Kit"))
                .child(labeled_field(
                    Label::new("Email"),
                    Input::new("email").value("hello@paper.design"),
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
                    Input::new("focus").value("GPUI Kit").show_focus(true),
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
                    Input::new("form-name").value("GPUI Kit").w(px(360.)),
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
                    Input::new("label-email").value("hello@paper.design"),
                ))
                .child(labeled_field(
                    Label::new("Name").required(true),
                    Input::new("label-name").value("GPUI Kit"),
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
                    Input::new("label-form-name").value("GPUI Kit").w(px(360.)),
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
