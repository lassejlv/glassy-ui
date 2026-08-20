# Components

All of these are re-exported from `glassy_ui`. Implementations stay private modules.

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

Separator defaults to the design spec specimen (280×1 or 1×36). Override with `Styled` (`.w_full()`). Skeleton pulse is 1600ms; reduced motion holds frame one.

## Label, Checkbox, Switch, Radio, Input, Select

See [Forms](./forms.md).

## Dialog, Alert, Popover, Tooltip, Dropdown, Context menu

See [Overlays](./overlays.md).

## Screenshots

### Buttons

| Light | Dark |
| --- | --- |
| ![Button specimens in the light theme](./images/specimens/button-light.png) | ![Button specimens in the dark theme](./images/specimens/button-dark.png) |

### Spinners

| Light | Dark |
| --- | --- |
| ![Spinner specimens in the light theme](./images/specimens/spinner-light.png) | ![Spinner specimens in the dark theme](./images/specimens/spinner-dark.png) |

### Progress

| Light | Dark |
| --- | --- |
| ![Progress specimens in the light theme](./images/specimens/progress-light.png) | ![Progress specimens in the dark theme](./images/specimens/progress-dark.png) |

### Badges

| Light | Dark |
| --- | --- |
| ![Badge specimens in the light theme](./images/specimens/badge-light.png) | ![Badge specimens in the dark theme](./images/specimens/badge-dark.png) |

### Keyboard hints

| Light | Dark |
| --- | --- |
| ![Keyboard hint specimens in the light theme](./images/specimens/kbd-light.png) | ![Keyboard hint specimens in the dark theme](./images/specimens/kbd-dark.png) |

### Separators

| Light | Dark |
| --- | --- |
| ![Separator specimens in the light theme](./images/specimens/separator-light.png) | ![Separator specimens in the dark theme](./images/specimens/separator-dark.png) |

### Skeletons

| Light | Dark |
| --- | --- |
| ![Skeleton specimens in the light theme](./images/specimens/skeleton-light.png) | ![Skeleton specimens in the dark theme](./images/specimens/skeleton-dark.png) |

### Cards

| Light | Dark |
| --- | --- |
| ![Card specimens in the light theme](./images/specimens/card-light.png) | ![Card specimens in the dark theme](./images/specimens/card-dark.png) |

### Command

| Light | Dark |
| --- | --- |
| ![Command specimens in the light theme](./images/specimens/command-light.png) | ![Command specimens in the dark theme](./images/specimens/command-dark.png) |

### Toasts

| Light | Dark |
| --- | --- |
| ![Toast specimens in the light theme](./images/specimens/toast-light.png) | ![Toast specimens in the dark theme](./images/specimens/toast-dark.png) |

### Breadcrumbs

| Light | Dark |
| --- | --- |
| ![Breadcrumb specimens in the light theme](./images/specimens/breadcrumb-light.png) | ![Breadcrumb specimens in the dark theme](./images/specimens/breadcrumb-dark.png) |

### Pagination

| Light | Dark |
| --- | --- |
| ![Pagination specimens in the light theme](./images/specimens/pagination-light.png) | ![Pagination specimens in the dark theme](./images/specimens/pagination-dark.png) |

### Tables

| Light | Dark |
| --- | --- |
| ![Table specimens in the light theme](./images/specimens/table-light.png) | ![Table specimens in the dark theme](./images/specimens/table-dark.png) |

### Sliders

| Light | Dark |
| --- | --- |
| ![Slider specimens in the light theme](./images/specimens/slider-light.png) | ![Slider specimens in the dark theme](./images/specimens/slider-dark.png) |

### Tabs

| Light | Dark |
| --- | --- |
| ![Tab specimens in the light theme](./images/specimens/tabs-light.png) | ![Tab specimens in the dark theme](./images/specimens/tabs-dark.png) |

### Accordions

| Light | Dark |
| --- | --- |
| ![Accordion specimens in the light theme](./images/specimens/accordion-light.png) | ![Accordion specimens in the dark theme](./images/specimens/accordion-dark.png) |

### Avatars

| Light | Dark |
| --- | --- |
| ![Avatar specimens in the light theme](./images/specimens/avatar-light.png) | ![Avatar specimens in the dark theme](./images/specimens/avatar-dark.png) |

## Planned

Command, Toast, Card, Tabs, Accordion, Avatar, Breadcrumb, Pagination, Table, Slider, Combobox, Menubar. Their design pages are ready, but the components are not implemented. Do not batch shadcn leftovers (calendar, sidebar, drawer, charts) unless an app needs that one page.
