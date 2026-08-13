use gpui::{App, Context, Global};

use crate::{Theme, ThemeKind};

#[derive(Clone, Copy)]
struct ActiveThemeState(Theme);

impl Global for ActiveThemeState {}

/// Install the default light theme. Call once at app startup.
pub fn init(cx: &mut App) {
    if !cx.has_global::<ActiveThemeState>() {
        cx.set_global(ActiveThemeState(Theme::light()));
    }
}

/// Read and switch the process-wide theme.
///
/// ```ignore
/// init(cx);
/// let theme = cx.theme();
/// div().bg(theme.canvas);
///
/// cx.toggle_theme();
/// cx.set_theme(Theme::dark());
/// cx.set_theme_name("light");
/// ```
pub trait ActiveTheme {
    fn theme(&self) -> Theme;
    fn set_theme(&mut self, theme: Theme);
    fn set_theme_kind(&mut self, kind: ThemeKind);
    fn set_theme_name(&mut self, name: &str) -> bool;
    fn toggle_theme(&mut self);
}

impl ActiveTheme for App {
    fn theme(&self) -> Theme {
        if self.has_global::<ActiveThemeState>() {
            self.global::<ActiveThemeState>().0
        } else {
            Theme::light()
        }
    }

    fn set_theme(&mut self, theme: Theme) {
        self.set_global(ActiveThemeState(theme));
        self.refresh_windows();
    }

    fn set_theme_kind(&mut self, kind: ThemeKind) {
        self.set_theme(Theme::for_kind(kind));
    }

    fn set_theme_name(&mut self, name: &str) -> bool {
        if let Some(theme) = Theme::named(name) {
            self.set_theme(theme);
            true
        } else {
            false
        }
    }

    fn toggle_theme(&mut self) {
        let next = self.theme().toggle();
        self.set_theme(next);
    }
}

impl<T> ActiveTheme for Context<'_, T> {
    fn theme(&self) -> Theme {
        App::theme(self)
    }

    fn set_theme(&mut self, theme: Theme) {
        App::set_theme(self, theme);
    }

    fn set_theme_kind(&mut self, kind: ThemeKind) {
        App::set_theme_kind(self, kind);
    }

    fn set_theme_name(&mut self, name: &str) -> bool {
        App::set_theme_name(self, name)
    }

    fn toggle_theme(&mut self) {
        App::toggle_theme(self);
    }
}
