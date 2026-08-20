# Components

All of these are re-exported from `gpui_ui`. Implementations stay private modules.

IDs must be stable. Most controls implement `Styled`.

## Button

```rust
Button::new("save", "Save")
    .variant(ButtonVariant::Primary)
    .size(ButtonSize::Medium)
    .leading_icon(IconName::Download)
    .loading(false)
    .disabled(false)
    .on_click(|_, _, _| {})

Button::icon_only("search", IconName::Search)
    .variant(ButtonVariant::Ghost)
    .tooltip(Tooltip::new("Search"))

ButtonGroup::new()
    .child(Button::new("draft", "Save draft").variant(ButtonVariant::Outline).grouped(true))
    .child(Button::new("publish", "Publish").grouped(true))
```

| Variant | Role |
| --- | --- |
| `Primary` | Heavy action |
| `Secondary` | Default glass |
| `Destructive` | The only saturated fill |
| `Outline` | Quiet chrome |
| `OutlineDestructive` | Danger without the heavy fill |
| `Ghost` | Toolbar, skip |

Sizes: Small 28, Medium 36, Large 44, Icon 36×36. Loading replaces the leading icon with a 14px spinner and ignores clicks. `.muted(true)` is ghost chrome with `muted_fg` (Skip).

## Spinner

120° arc on a faint track. Not a Lucide loader glyph.

```rust
Spinner::new()
    .size(SpinnerSize::Default) // Small 16, Default 20, Large 24, Display 32
    .tone(SpinnerTone::Default) // Muted, Inverse, Destructive
```

## Progress

Values clamp to `0.0..=1.0`. Linear default 280×8. Circular is 24px.

```rust
Progress::new(0.4)
CircularProgress::new(0.4)
```

## Badge, Kbd, Separator, Skeleton

```rust
Badge::new("12").variant(BadgeVariant::Muted) // Default, Muted, Destructive
Kbd::new("⌘K")
Separator::horizontal() // or Separator::vertical()
Skeleton::text("row-title")
Skeleton::avatar("owner")
Skeleton::control("field")
```

Separator defaults to the Paper specimen (280×1 or 1×36). Override with `Styled` (`.w_full()`). Skeleton pulse is 1600ms; reduced motion holds frame one.

## Label, Checkbox, Switch, Radio, Input, Select

See [Forms](./forms.md).

## Dialog, Alert, Popover, Tooltip, Dropdown, Context menu

See [Overlays](./overlays.md).

## Planned

Command, Toast, Card, Tabs, Accordion, Avatar, Breadcrumb, Pagination, Table, Slider, Combobox, Menubar. Design the Paper page first. Do not batch shadcn leftovers (calendar, sidebar, drawer, charts) unless an app needs that one page.
