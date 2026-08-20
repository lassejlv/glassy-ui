# gpui-ui

A shadcn-shaped component kit for native [GPUI](https://github.com/zed-industries/zed). One component per Paper page, light and dark. It should look like a control and behave like a control.

Quiet zinc glass on a mineral canvas. Opaque window. Translucency on controls, not the frame. One saturated color (destructive red). Radius 6. Inter only.

Paper (`Grafik UI`) is the visual contract. Code matches computed values, not screenshots.

## Docs

- [Getting started](docs/getting-started.md) — pin, init, first button
- [Components](docs/components.md) — public API and examples
- [Forms](docs/forms.md) — controlled state, Input, Select
- [Overlays](docs/overlays.md) — dialog, menus, tooltip
- [Theme and chrome](docs/theme.md) — tokens, glass, motion
- [`DESIGN.md`](DESIGN.md) — pixels and materials
- [`PLAN.md`](PLAN.md) — shipped vs next

## Run

```sh
cargo run          # gallery (⌘D theme, ⌘] next page, ⌘Q quit)
cargo test
```

GPUI is pinned to Zed `101ca00`. Use that revision in consuming apps. Full setup is in [getting started](docs/getting-started.md).

## Components

Button, Input, Textarea, Select, Checkbox, Switch, Radio, Label, Kbd, Badge, Separator, Skeleton, Spinner, Progress, Tooltip, Dialog, Alert dialog, Popover, Dropdown menu, Context menu.

Next in Paper order: Command, then Toast. See [`PLAN.md`](PLAN.md).

## License

MIT
