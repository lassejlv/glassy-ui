# Repository Guide

## Sources Of Truth

- This is one `glassy-ui` package, not a workspace. `src/lib.rs` is the library surface; `src/main.rs` is the `glassy-gallery` demo binary and default run target.
- For visual work, `DESIGN.md` and the `Glassy UI` design file are the contract. Use `get_jsx` / `get_computed_styles`; screenshots are only QA. Preserve exact pixels and `RRGGBB_AA` alpha values rather than approximating them.
- `PLAN.md` distinguishes implemented components from the design-ready queue. Do not implement items under "Not unless asked."

## Architecture Constraints

- Component implementations are private modules re-exported from `src/lib.rs`; add new public API there. Gallery specimens and page routing stay in `src/main.rs`.
- Semantic text/ground colors belong in `src/theme`; material fills, rims, and shadows belong in `src/chrome.rs` or the component. Do not turn control chrome into theme tokens.
- Use `src/motion`; do not introduce another animation stack. Ordinary UI motion is critically damped/interruptible, hover fills are immediate, and reduced motion must remain static or opacity-only.
- The project intentionally has no direct third-party runtime dependencies besides pinned Zed GPUI crates. Do not add a crate to replace the local theme or motion implementations.
- Asset embedding is an explicit match/list in `src/assets.rs`. Adding or renaming an icon/font requires updating both `AssetSource::load` and `AssetSource::list`.
- A consuming app must install `Assets`, then call `init_motion`, `init_theme`, `glassy_ui::init` (input keybindings), and `load_fonts`; `src/main.rs` is the executable reference.

## Commands

- Run the gallery: `cargo run` (equivalent to `cargo run --bin glassy-gallery`).
- Full verification: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, then `cargo test`.
- Library/unit tests only: `cargo test --lib`.
- One integration target: `cargo test --test select_interaction` (targets also include `dialog_interaction`, `alert_dialog_interaction`, and `popover_interaction`).
- One GPUI interaction test: `cargo test --test select_interaction popup_keeps_an_eight_pixel_trigger_gap`.

## GPUI Tests

- Interaction tests use `#[gpui::test]`, `TestAppContext`, and `VisualTestContext`; initialize required globals such as `init_theme` before opening the test window and call `run_until_parked` before querying geometry.
- Preserve `debug_selector` names used by tests. Overlay tests assert exact geometry, focus restoration, event occlusion/propagation, and keyboard behavior, not snapshots.
