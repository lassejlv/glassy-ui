# Grafik

A shadcn-shaped component kit for native [GPUI] interfaces. One component per Paper page, light and dark, so an app can pick a button or a spinner and get the same object Apple would call *familiar* — it looks like a control, it behaves like a control, nothing extra is asking for attention.

Built on [Zed]'s GPU-accelerated UI framework for Rust. No other third-party crates — motion easing, springs, and theming are implemented from scratch.

[GPUI]: https://github.com/zed-industries/zed
[Zed]: https://zed.dev

---

## Design language

**Quiet zinc glass on a mineral canvas.** Cool, overcast zinc. Translucent glass controls sitting on an opaque stone ground. One saturated color (destructive red). Radius 6. Inter only. Hierarchy is type and opacity, not extra chrome.

The visual contract lives in a Paper design file (`Grafik UI`). Code matches Paper's computed values, not screenshots. [`DESIGN.md`](./DESIGN.md) is the canonical spec; [`PLAN.md`](./PLAN.md) is the component roadmap.

### Principles

- **Glass is the control, canvas is the ground.** The window is opaque. Translucency belongs on controls and panels, not app chrome.
- **Material weight encodes hierarchy.** Heavier glass = primary action. Lighter glass = secondary, outline, ghost.
- **One intense color.** Destructive red. Everything else is zinc.
- **Motion is a conversation, not a cutscene.** Default UI springs are critically damped. Bounce only when the user's gesture carried momentum.
- **Respond on the way in.** Hover and press change the fill immediately.
- **Simplicity, not minimalism.** A 1px rim and an inset highlight are how glass reads as glass — not decoration.

---

## Components

| Component | Module | Notes |
| --- | --- | --- |
| Button | `button.rs` | Six variants (Primary, Secondary, Destructive, Outline, OutlineDestructive, Ghost), three sizes, leading/trailing icons, loading, disabled, groups |
| Input | `input.rs` | 280×36 outline-glass field, focus ring, disabled, invalid |
| Textarea | `input.rs` | Same chrome, 320×96 |
| Select | `select.rs` | Anchored popup, 8px gap, outside-click + Escape dismissal, arrow-key nav |
| Checkbox | `checkbox.rs` | 16×16, radius 6. Off / on / mixed |
| Switch | `switch.rs` | 36×20 pill, instant fill |
| Radio | `radio.rs` | 16×16 circle, 6px dot |
| Label | `label.rs` | 13/500, required `*`, optional |
| Kbd | `kbd.rs` | Ghost glass key chip, 22 tall |
| Badge | `badge.rs` | 22px status chip. Default, muted, destructive |
| Separator | `separator.rs` | 1px zinc. Horizontal 280, vertical 36 |
| Skeleton | `skeleton.rs` | Secondary-glass pulse. Text, avatar, control presets |
| Spinner | `spinner.rs` | 120° arc on faint track, linear 700ms loop. Four tones, three sizes |
| Progress | `progress.rs` | Linear 280×8 + circular 24px ring |
| Tooltip | `tooltip.rs` | Inverse chip, 24 tall, ~300ms delay. Above / Below / Start / End |
| Dialog | `dialog.rs` | Radius-10 panel, dim scrim, Escape + scrim dismissal, focus save/restore, Tab cycling |
| Alert dialog | `alert_dialog.rs` | Destructive confirmation, locked scrim, safe autofocus on cancel |
| Popover | `popover.rs` | Secondary glass, radius 6, origin at the trigger |
| Dropdown menu | `dropdown_menu.rs` | 240px anchored menu, shortcuts, disabled/destructive items, nested menus, full keyboard navigation |

### Planned

Context menu, Command palette, Toast, Card, Tabs, Accordion, Avatar, Breadcrumb, Pagination, Table, Slider, Combobox, Menubar. See [`PLAN.md`](./PLAN.md).

---

## Architecture

Three subsystems, all in `src/`:

```
src/
├── theme/          # semantic color tokens + process-wide active theme
├── motion/         # motion.dev-inspired animation (Motion, AnimatePresence, Stagger, Variants, Transition)
├── chrome.rs       # material paint helpers (fills, rims, shadows per variant)
├── *.rs            # one file per component
├── lib.rs          # module declarations + public re-exports
└── main.rs         # grafik-gallery demo binary
```

### Theme

Semantic color tokens (`canvas`, `ink`, `body`, `label`, `destructive`, …) with light and dark palettes. A process-wide `ActiveTheme` trait on `App` and `Context` provides `theme()`, `set_theme()`, `toggle_theme()`. Material chrome (button/field/dialog fills, rims, shadows) lives in `chrome.rs`, not as theme tokens.

### Motion

A [motion.dev]-inspired port providing `Motion` (div-like element with `initial` / `animate` / `exit` / `while_hover` / `while_tap`), `AnimatePresence` (keeps removed children mounted until exit animations finish), `Stagger` (delays enter animations across children), `Variants`, `Transition` (tween + spring), and shared `MotionValue`s. Built on GPUI's animation frame system.

[motion.dev]: https://motion.dev

---

## Getting started

Requires Rust (edition 2021) and a system that can build GPUI (macOS, Linux with Wayland/X11).

```sh
# Run the gallery demo (1440×900 window, 16+ component pages)
cargo run

# Run the test suite
cargo test

# Lint
cargo clippy --workspace --all-targets -- -D warnings
```

### Using the kit

Add to your `Cargo.toml`:

```toml
[dependencies]
grafik-ui = { git = "https://github.com/lassevestergaard/gpui-kit" }
gpui = { git = "https://github.com/zed-industries/zed", rev = "101ca00a1352ed71ef398f21b47836565d1998e3" }
```

Initialize at startup:

```rust
use grafik_ui::{init, init_motion, init_theme, load_fonts, Assets};

fn main() {
    let app = Application::new();
    app.run(|cx| {
        cx.set_asset_source(Assets);
        load_fonts(cx);
        init_motion(cx);
        init_theme(cx);
        init(cx); // bind input keystrokes
    });
}
```

Components read the active theme via the `ActiveTheme` trait:

```rust
use grafik_ui::{ActiveTheme, Button, ButtonVariant};

fn view(cx: &mut WindowContext) -> impl IntoElement {
    let theme = cx.theme();
    Button::new("confirm", cx)
        .variant(ButtonVariant::Primary)
        .label("Confirm")
}
```

### Gallery controls

| Action | Shortcut |
| --- | --- |
| Toggle theme | `⌘D` / `ctrl-D` |
| Quit | `⌘Q` / `ctrl-Q` |
| Next page | `⌘]` |
| Jump to page | `⌘1`–`⌘9`, `⌘0` |

---

## Testing

Integration tests in `tests/` use GPUI's `TestAppContext` to simulate real mouse and keyboard events:

- `select_interaction.rs` — popup anchoring, dismissal, keyboard nav
- `dialog_interaction.rs` — panel geometry, focus restore, scrim dismissal
- `alert_dialog_interaction.rs` — safe autofocus, locked scrim, Escape cancel
- `dropdown_menu_interaction.rs` — anchored geometry, nested pointer/keyboard actions, dismissal, focus restore

Inline unit tests assert theme tokens, component chrome, and motion easing against Paper values.

---

## License

MIT
