# Getting started

Requires Rust 2021 and a host that can build GPUI (macOS, or Linux with Wayland/X11). The crate pins a Zed revision. Use that same `rev` in your app.

## Run the gallery

From this repo:

```sh
cargo run                 # gpui-gallery
cargo test
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

Gallery: 1440×900, opaque window, `⌘D` theme, `⌘]` next page, `⌘Q` quit.

## Add the crate

```toml
[dependencies]
gpui-ui = { git = "https://github.com/lassejlv/gpui-ui" }
gpui = { git = "https://github.com/zed-industries/zed", rev = "101ca00a1352ed71ef398f21b47836565d1998e3" }
gpui_platform = { git = "https://github.com/zed-industries/zed", rev = "101ca00a1352ed71ef398f21b47836565d1998e3", features = [
    "font-kit",
    "wayland",
    "x11",
] }
```

## Startup

Install `Assets`, then init in this order. `src/main.rs` is the executable reference.

```rust
use gpui::App;
use gpui_platform::application;
use gpui_ui::{init, init_motion, init_theme, load_fonts, Assets};

fn main() {
    application()
        .with_assets(Assets)
        .run(|cx: &mut App| {
            init_motion(cx);
            init_theme(cx);
            init(cx); // Input / Textarea keybindings
            load_fonts(cx).expect("register Inter");
        });
}
```

`init_theme` installs light as the process default. `load_fonts` registers the bundled Inter Variable face (`FONT_FAMILY`). `init` binds field keys under the `KitInput` context.

If you skip `Assets`, kit icons will not load. If you skip `init`, text fields will not handle backspace, arrows, or clipboard.

## First control

IDs must be stable across rerenders (hover, focus, and keyed state use them).

```rust
use gpui::{div, prelude::*, px};
use gpui_ui::{ActiveTheme, Button, ButtonVariant};

fn toolbar(cx: &mut gpui::App) -> impl gpui::IntoElement {
    let theme = cx.theme();
    div()
        .bg(theme.canvas)
        .p(px(24.))
        .child(
            Button::new("save", "Save")
                .variant(ButtonVariant::Primary)
                .on_click(|_, _, _| {}),
        )
}
```

Components implement `Styled`. You can override width, padding, and similar after the kit geometry.

## Gallery

| Action | Shortcut |
| --- | --- |
| Toggle light / dark | `⌘D` / `Ctrl+D` |
| Quit | `⌘Q` / `Ctrl+Q` |
| Next page | `⌘]` / `Ctrl+]` |
| Jump | `⌘1`–`⌘9`, `⌘0`, plus shift chords for later pages |

The window stays opaque. Frost belongs on controls, not the frame.
