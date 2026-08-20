//! Process-wide light/dark theme for gpui-ui.
//!
//! ```ignore
//! use gpui_ui::{init_theme, ActiveTheme, Theme};
//!
//! fn main() {
//!     application().run(|cx| {
//!         init_theme(cx);
//!         // later, anywhere you have App / Context:
//!         let theme = cx.theme();
//!         div().bg(theme.canvas).text_color(theme.ink);
//!         cx.toggle_theme();
//!     });
//! }
//! ```

mod active;
mod tokens;

pub use active::{init, ActiveTheme};
pub use tokens::{paint, rgb, Theme, ThemeKind, FONT_FAMILY};
