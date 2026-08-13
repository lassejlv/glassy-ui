use std::borrow::Cow;

use gpui::{App, AssetSource, Result, SharedString};

/// Bundled icons and Inter.
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let bytes: Option<&'static [u8]> = match path {
            "icons/plus.svg" => Some(include_bytes!("../assets/icons/plus.svg")),
            "icons/download.svg" => Some(include_bytes!("../assets/icons/download.svg")),
            "icons/chevron-right.svg" => Some(include_bytes!("../assets/icons/chevron-right.svg")),
            "icons/search.svg" => Some(include_bytes!("../assets/icons/search.svg")),
            "icons/spinner.svg" => Some(include_bytes!("../assets/icons/spinner.svg")),
            "icons/check.svg" => Some(include_bytes!("../assets/icons/check.svg")),
            "fonts/InterVariable.ttf" => Some(include_bytes!("../assets/fonts/InterVariable.ttf")),
            _ => None,
        };
        Ok(bytes.map(Cow::Borrowed))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let all = [
            "icons/plus.svg",
            "icons/download.svg",
            "icons/chevron-right.svg",
            "icons/search.svg",
            "icons/spinner.svg",
            "icons/check.svg",
            "fonts/InterVariable.ttf",
        ];
        Ok(all
            .into_iter()
            .filter(|p| p.starts_with(path))
            .map(SharedString::from)
            .collect())
    }
}

/// Register the bundled Inter variable face. Call once at startup.
pub fn load_fonts(cx: &App) -> gpui::Result<()> {
    cx.text_system().add_fonts(vec![Cow::Borrowed(
        include_bytes!("../assets/fonts/InterVariable.ttf").as_slice(),
    )])
}
