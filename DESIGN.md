# gpui-ui — Design

A shadcn-shaped component kit for native GPUI. The visual contract lives in Paper (`Grafik UI`). Code must match `get_jsx` / `get_computed_styles`, not screenshots.

This document describes the system we actually ship: **quiet zinc glass on a mineral canvas**. Apple’s interface language (WWDC *Designing Fluid Interfaces*, materials, type) is the feel we aim for. Paper pixels are the law.

---

## Intent

**Mood:** mineral, overcast, zinc. Limestone dust, weathered slate, a pale sky behind frosted glass. Not warm cream. Not neon SaaS. Not decorative blobs.

**Emotion:** calm, precise, slightly cool. Controls feel like thin pieces of glass sitting on stone — you can see the ground through them, but they still have an edge, a catch-light, and a little weight.

**Purpose:** one component per Paper page, light and dark, so an app can pick a button or a spinner and get the same object Apple would call *familiar*: it looks like a control, it behaves like a control, nothing extra is asking for attention.

Four human needs, applied here:

| Need | In this kit |
| --- | --- |
| Safety / predictability | Same radius, same typeface, same zinc scale everywhere. Light and dark are one system, inverted. |
| Understanding | Hierarchy is type + opacity, not extra chrome. Labels are 13/500 zinc. Actions are 14/500 ink. |
| Achievement | Primary is the only heavy material. Destructive is the only saturated color. |
| Joy | Glass catch-light and a quiet spinner — not confetti. Delight is the other three done well. |

---

## Principles

1. **Paper is the source.** Exact pixels (`px(6)`, `px(36)`, `rgba(24,24,27,0.72)`). Do not round to a Tailwind scale. Do not invent a parallel palette.
2. **Inter only.** No second family. Variable face in GPUI (`Inter Variable`); Paper artboards use Inter.
3. **Glass is the control, canvas is the ground.** The window is **opaque**. Do not put blur on the window unless asked. Translucency belongs on *controls and panels*, not the app chrome.
4. **Material weight encodes hierarchy.** Heavier / darker glass = primary action. Lighter / thinner glass = secondary, outline, ghost. Never stack a light translucent surface on another light translucent surface.
5. **One intense color.** Destructive red (`#DC2626` / `#F87171`). Everything else is zinc.
6. **Radius is 6.** Panels that are clearly a card (spinner empty state) may use 10. Circles: radio (16×16, radius 8) and avatar. Nothing else.
7. **No blobs, no gradients-as-decoration, no drop-shadow theater.** Shadows exist only to give glass thickness (inset catch-light + a soft ground shadow).
8. **Motion is a conversation, not a cutscene.** Default UI springs are critically damped. Bounce only when the user’s gesture carried momentum. Status motion (spinner) is linear and quiet.
9. **Respond on the way in.** Hover/press must change the fill immediately. Waiting for click-up to show life feels dead.
10. **Simplicity, not minimalism.** A 1px white rim and an inset highlight are not decoration — they are how glass reads as glass. Removing them makes the control look like a flat rectangle.

---

## Canvas

The page is a solid mineral ground. Content scrolls on it. The titlebar is transparent; the fill is not.

| | Light | Dark |
| --- | --- | --- |
| Canvas | `#E8EAEE` | `#0B0C0F` |
| Window | Opaque | Opaque |
| Artboard | 1440 wide, `fit-content`, min-height 900 | same |
| Page padding | 80px sides, 72px top, 80px bottom | same |
| Section gap | 56px after header, then 40px | same |
| Section label → row | 16px | same |

Gallery window: 1440×900, opaque, transparent titlebar, vertical scroll (`flex_1` + `min_h(0)` + `overflow_y_scroll`).

---

## Color

Zinc scale, plus one red. Semantic tokens live in `src/theme`. Button *chrome* (fills, rims, shadows) stays in `src/chrome.rs` — it is material, not theme text.

### Semantic (theme)

| Token | Light | Dark | Role |
| --- | --- | --- | --- |
| `canvas` | `#E8EAEE` | `#0B0C0F` | Opaque ground |
| `heading` | `#18181B` | `#FAFAFA` | Display / page title |
| `ink` | `#18181B` | `#FAFAFA` | Primary copy, default control label |
| `body` | `#71717A` | `#A1A1AA` | Supporting sentence |
| `label` | `#A1A1AA` | `#71717A` | Eyebrow, section title, caption |
| `on_solid` | `#FAFAFA` | `#FAFAFA` | Text on primary / inverse glass |
| `destructive` | `#DC2626` | `#F87171` | Destructive action + spinner |
| `destructive_soft` | `#DC2626` | `#FCA5A5` | Outline-destructive label on dark |
| `muted_fg` | `#A1A1AA` | `#71717A` | Disabled, muted ghost (Skip) |

Light muted inline copy (spinner “Fetching logs”) uses `#52525B` — one step darker than `body`, so the muted spinner and its label share a hue.

### How glass is mixed

Paper paints glass as **zinc or white at an alpha**, then a **white rim**, then an **inset 1px highlight** (light catching the top of the material), then a **soft ground shadow**. Intended CSS also uses `backdrop-filter: blur(20px) saturate(160–180%)`. GPUI has no per-control backdrop-filter; on this flat canvas that filter is nearly a no-op, so **alpha fills + rim + inset + shadow** are the native material.

Never use opaque `#18181B` for a primary button. The 72% zinc *is* the primary.

---

## Materials & depth

Apple: translucent layers structure a scene without stealing focus. Bigger surfaces read thicker. A bright top edge is light on the material.

### Control glass (radius 6)

Always, in this order:

1. Fill (zinc/white/red at alpha)
2. 1px border (rim)
3. Inset shadow `0 1px 0` (catch-light)
4. Drop shadow only if the variant has weight (ghost has none)

Hover **lifts the fill** a step (more opaque). Rest state is the Paper default. Do not scale, bounce, or glow on hover.

### Light control chrome

| Variant | Fill | Rim | Inset | Shadow | Foreground |
| --- | --- | --- | --- | --- | --- |
| Primary | `#18181B` @ 72% (`B8`) | white @ 16% | white @ 22% | `#0F172A` @ 10%, y 8 / blur 20 | `#FAFAFA` |
| Secondary | white @ 52% (`85`) | white @ 72% | white @ 90% | `#0F172A` @ 6%, y 6 / blur 16 | ink |
| Destructive | `#DC2626` @ 78% (`C7`) | white @ 28% | white @ 28% | red @ 16%, y 8 / blur 20 | `#FFFFFF` |
| Outline | white @ 28% (`47`) | white @ 62% | white @ 75% | `#0F172A` @ 4%, y 4 / blur 12 | ink |
| Outline destructive | `#FEE2E2` @ 42% | `#FECACA` @ 70% | white @ 70% | red @ 6%, y 4 / blur 12 | `#DC2626` |
| Ghost | white @ 16% (`29`) | white @ 28% | white @ 40% | none | ink |

Hover fills (same hue, higher alpha): Primary `CC`, Secondary `99`, Destructive `DB`, Outline `5C`, Outline destructive `85`, Ghost `3D`.

### Dark control chrome

On dark canvas, primary **inverts**: it becomes white glass, not black glass. That is how inverse materials work — the heavy action is the lightest translucent chip.

| Variant | Fill | Rim | Inset | Shadow | Foreground |
| --- | --- | --- | --- | --- | --- |
| Primary | white @ 16% (`29`) | white @ 22% | white @ 28% | black @ 28%, y 8 / blur 24 | `#FAFAFA` |
| Secondary | white @ 7% (`12`) | white @ 10% | white @ 12% | black @ 18%, y 6 / blur 16 | `#FAFAFA` |
| Destructive | `#DC2626` @ 62% (`9E`) | white @ 16% | white @ 18% | red @ 18%, y 8 / blur 20 | `#FAFAFA` |
| Outline | white @ 4% (`0A`) | white @ 14% | white @ 10% | black @ 16%, y 4 / blur 12 | `#FAFAFA` |
| Outline destructive | `#7F1D1D` @ 28% | `#F87171` @ 28% | white @ 8% | none | `#FCA5A5` |
| Ghost | white @ 3% (`08`) | white @ 6% | white @ 6% | none | `#FAFAFA` |

### Panel glass (radius 10)

Used for empty / waiting surfaces (spinner “Opening workspace”): 280×148, centered content, gap 12.

| | Fill | Rim | Inset | Shadow |
| --- | --- | --- | --- | --- |
| Light | white @ 52% | white @ 72% | white @ 90% | `#0F172A` @ 6%, y 6 / blur 16 |
| Dark | white @ 7% | white @ 10% | white @ 12% | black @ 28%, y 6 / blur 16 |

Same family as secondary button, thicker because the surface is bigger.

### Inverse chip

A 40×40 primary-weight tile so a light spinner can sit on dark glass (and the reverse in dark mode). Inverse is a **context**, not a third theme.

---

## Typography

Face: **Inter** (Paper) / **Inter Variable** (GPUI). Weights 400 / 500 / 600 only.

Apple: tracking is size-specific; leading tightens as type grows; hierarchy is weight + size + leading together.

| Role | Size | Weight | Line-height | Tracking (Paper) | Color |
| --- | --- | --- | --- | --- | --- |
| Eyebrow (`BUTTON`, `SPINNER`) | 13 | 500 | 16 | `0.04em` | `label` |
| Page title | 36 | 600 | 40 | `-0.03em` | `heading` |
| Page body | 15 | 400 | 24 | 0 | `body`, max-width 460 |
| Section label | 13 | 500 | 16 | 0 | `label` |
| Button / inline label | 14 | 500 | 18 | `-0.01em` | ink / on_solid / destructive |
| Button small | 12 | 500 | 16 | `-0.01em` | same |
| Button large | 15 | 500 | 18 | `-0.01em` | same |
| Caption under a sample | 13 | 500 | 16 | 0 | `label` |

GPUI does not expose letter-spacing today. Ship the sizes, weights, and line-heights exactly; treat tracking as a known delta, not a license to pick a different face or weight.

Do not use type smaller than 12 except on `ButtonSize::Small`. Do not introduce a display face.

---

## Layout

- Flex, not absolute, unless something actually overlaps.
- Page header: column, gap 10, padding `72 80 0`. Trailing gallery controls (theme, page) sit `space-between` — they are app chrome, not Paper.
- Sample rows: `align-items: center` (or `flex-end` when sizes differ), gap 12 for buttons, 32 for spinner sizes, 24 for spinner colors, 28 for labeled spinners, 16 for in-use.
- Icon gap inside a button: 8. Icon-bearing horizontal pad: 14. Text-only pad: 12 / 16 / 20 (S/M/L).
- Icon-only control: 36×36, no extra pad.
- Grouped pair (Save draft | Publish): one outline pill, radius 6, overflow clip, children `h_full`, no per-child radius.

---

## Components

### Button

Solid, outline, and ghost. Sized from compact toolbars to primary conversion.

| Size | Height | Pad X | Type |
| --- | --- | --- | --- |
| Small | 28 | 12 | 12/16 |
| Medium | 36 | 16 (14 with icon) | 14/18 |
| Large | 44 | 20 (14 with icon) | 15/18 |
| Icon | 36×36 | 0 | — |

Variants: Primary, Secondary, Destructive, Outline, OutlineDestructive, Ghost.

States:

- **Disabled** — secondary material, `muted_fg`, no hover, default cursor.
- **Loading** — primary (or current variant) stays; label remains; leading 14px spinner in `on_solid`; not clickable.
- **Muted ghost** — Skip: ghost chrome, `muted_fg`.
- **Hover** — fill steps up. Instant. No scale.
- **Grouped** — share one outline shell.

Icons: Lucide, 16px, `currentColor` (plus, download, chevron-right, search). Loading replaces the leading icon.

### Spinner

A quiet mark for waiting. One 120° arc on a faint track. Not a Lucide “loader” glyph.

| Size | Box | Stroke | Radius |
| --- | --- | --- | --- |
| Button loading | 14 | 1.5 | 75% of half |
| Small | 16 | 1.5 | same |
| Default | 20 | 1.75 | same |
| Large | 24 | 2 | same |
| Display | 32 | 2.25 | same |

Arc caps are round. Track is the arc hue at ~18–28% alpha.

| Tone | Light arc / track | Dark arc / track |
| --- | --- | --- |
| Default | `#18181B` / 20% | `#FAFAFA` / 18% |
| Muted | `#52525B` / 28% | `#A1A1AA` / 28% |
| Inverse | `#FAFAFA` / 22% | `#18181B` / 18% |
| Destructive | `#DC2626` / 20% | `#F87171` / 28% |

Rotation: linear, 700ms per turn, repeating. This is status, not a gesture — no bounce, no ease. Honor reduced motion by holding the start frame (GPUI `reduce_motion`).

Labeled wait: 16px spinner + 8px + 14/500 label.

### Radio

One choice from a set. A circle so it cannot be mistaken for a checkbox.

| | Size | Radius | Inner |
| --- | --- | --- | --- |
| Mark | 16×16 | 8 (circle) | 6×6 dot, radius 3 |

Materials are the same as Checkbox: outline glass at rest, primary glass when selected, ghost when disabled. The selected mark is a filled `on_solid` dot, not a check. Disabled selected uses `muted_fg` for the dot.

Label: 14/400/18, gap 8, ink (muted when disabled). No mixed state.

### Select

Closed field is Input chrome, 280×36, padding-inline 14, plus a 16px chevron-down in `label`. Placeholder uses `placeholder`; value uses ink. Focus is the Input zinc ring. Disabled is Input disabled chrome.

Open list: secondary glass, radius 6, padding 4, gap 8 below the field. Rows 32 tall, padding-inline 8, 16px leading slot (check on selected, empty otherwise). Selected row is ghost fill. Disabled option is `label` and not clickable. Arrows, enter, esc.

### Kbd

A chip for a key. Ghost glass, 22 tall, radius 6, min-width 22, padding-inline 6. Type 12/500/16. Not a button: no hover, no press.

Light: ghost `#FFFFFF29` / border `#FFFFFF47` / inset `#FFFFFF66` / ink. Dark: ghost `#FFFFFF08` / border `#FFFFFF0F` / inset `#FFFFFF0F` / `#FAFAFA`.

Specimens: ⌘K, ⌘Q, Esc. Hint row gap 8. Menu row 36 tall, 360 wide, shortcut trailing.

### Separator

A quiet cut. One pixel of zinc at twelve percent — not a black rule.

| | Size | Light | Dark |
| --- | --- | --- | --- |
| Horizontal | 280×1 | `#18181B1F` | `#FAFAFA1F` |
| Vertical | 1×36 | same | same |

Toolbar ticks are the vertical rule at 16 tall. In-use stack: title, rule, body, gap 12, width 280.

### Badge

A count or status, not a button. Height 22, radius 6, 12/500, padding-inline 6.

| Variant | Light | Dark | Foreground |
| --- | --- | --- | --- |
| Default | primary glass | primary (white) glass | `on_solid` |
| Muted | ghost glass | ghost glass | `label` |
| Destructive | destructive glass | destructive glass | `#FFFFFF` / `#FAFAFA` |

### Tooltip

Inverse chip, 24 tall, radius 6, 12/500, padding-inline 8, gap 6 from the trigger. Delay ~300ms. Same path in and out. Never under the cursor.

| | Fill | Text |
| --- | --- | --- |
| Light | primary glass (`#18181BB8`) | `#FAFAFA` |
| Dark | `#FAFAFA` | `#18181B` |

Placements: Above, Below, Start, End.

### Dropdown menu

Trigger-anchored secondary glass, 240 wide, radius 6, padding 4, row gap 2. Rows are 32 tall with radius 4 and 10px horizontal padding. The panel sits 6px from its trigger.

Items use 14/400/18 ink. Hover and keyboard highlight use `#FFFFFF47` light / `#FFFFFF12` dark. Disabled uses `label`; destructive uses `destructive`. Shortcuts reuse Kbd, separators are 1px zinc, and nested items use a 16px chevron-right.

Click or Enter/Space opens. Up/Down skip separators and disabled items. Right enters a submenu, Left returns, Escape and outside click dismiss, and focus returns to the trigger after keyboard dismissal or activation.

### Context menu

The same panel as Dropdown. No trigger. Origin is the pointer. Right-click opens, Escape and outside click dismiss, focus returns to the surface.

### Progress

Determinate. Linear fill, no bounce.

| | Size | Material |
| --- | --- | --- |
| Track | 280×8, radius 4 (pill) | outline glass |
| Fill | width = percent of 280 | primary glass |
| Circular | 24 box, stroke 2, r 9 | track at ~20% ink, arc ink |

Specimens: 0 / 40 / 100.

### Skeleton

Pulse on secondary glass. Reduced motion holds the first frame.

| Shape | Size | Radius |
| --- | --- | --- |
| Text line | 180×12 | 6 |
| Avatar | 32×32 | 16 (circle) |
| Control | 280×36 | 6 |

### Dialog

Centered question. Radius 10 panel (secondary glass), dim scrim (`#18181B47` light / `#00000073` dark) — not a second glass layer. Title 16/600, body 15/400, optional field, footer Ghost + Primary. Esc and overlay click dismiss. Spring 0.3s, damping 1.0; interruptible.

### Avatar

A face, like Radio — a circle, not a rounded-square control.

| Size | Box | Initials |
| --- | --- | --- |
| 24 | 24×24, radius 12 | 10/600 |
| 32 | 32×32, radius 16 | 12/600 |
| 40 | 40×40, radius 20 | 14/600 |

Treatments: image (clipped portrait), initials on primary glass (`on_solid`), fallback ghost glass + user icon.

### Breadcrumb

Ghost links (`label`), chevron-right 16 (`label`), current page 14/500 ink. Hover lifts a link to ink. Collapsed middle is `…`.

### Pagination

Ghost 36×36 icon buttons (chevron-left / chevron-right) and page numbers. Current page is primary glass. Disabled at the ends (ghost, opacity ~0.55, `label` icon).

### Table

Not a spreadsheet. Header 13/500 `label`, 1px zinc rule at 12%. Rows 40 tall, radius 6. Hover: ghost fill. Selected: outline glass. Empty: 280×148 secondary panel, spinner + “No pages.”

Columns: Page, Size, Layers — fixed-width slots.

### Slider

Progress track 280×8 pill, outline glass, primary fill. Thumb 16 circle, primary glass, 1:1 with the pointer while dragging. Keyboard. Disabled is muted ghost. Value is 14/500 beside the track.

### Combobox

Closed field is Select (Input chrome 280×36 + chevron). Open: same field is the filter, list is secondary glass radius 6. Selected match has a check. Empty: “No pages match.”

### Menubar

In-window File / Edit / View. Items 28 tall, 13/500, padding-inline 10. Open item is ghost fill. Menu is Dropdown (radius 6): hover, Kbd, separator, destructive. Native OS menus stay native.

---

## Motion

The motion crate is a loora/motion.dev port: tween, spring, stagger, presence. Use it for enter/exit and gesture-driven UI. Do not add a second animation stack.

**Defaults (Apple-shaped, mapped onto what we have):**

| Interaction | Tool | Values |
| --- | --- | --- |
| Default UI (menus, toggles, layout) | Spring, critically damped | damping `1.0`, response `0.3–0.4s` — or tween 300ms ease-out |
| Hover fill | Instant | no duration |
| Press | Instant scale toward `0.97` if we add it; never wait for mouse-up | — |
| Flick / drag release | Spring with bounce | damping `~0.8`, response `0.3–0.4s`, hand off release velocity |
| Spinner | Linear loop | 700ms |
| Theme change | Cross-fade / instant retoken | no full-viewport flash |

Rules:

- Animate from the **presentation** value, never jump to a logical target.
- Every transition must be interruptible. New input retargets the spring; it does not queue.
- Enter and exit along the same path.
- Bounce only when the gesture had momentum. A button that merely appeared must not overshoot.
- Reduced motion: opacity cross-fade, no elastic, spinner static.

Implemented component motion uses `src/motion`: selection marks fade in over 140ms, anchored/modal surfaces fade in over 180ms, and switch thumbs slide over 180ms ease-out without bounce. Hover fills remain instant.

Crate defaults today: tween 300ms ease-out; `Transition::spring()` is 500ms with bounce `0.25` (use that for momentum, not for chrome). Prefer a critically damped spring for ordinary UI when wiring new motion.

---

## Light and dark

One system. Dark is not a new personality.

- Ground goes near-black (`#0B0C0F`), ink goes near-white (`#FAFAFA`).
- Labels and body **swap** (`#A1A1AA` ↔ `#71717A`) so captions stay quieter than sentences.
- Primary glass **inverts** (black glass → white glass).
- Destructive stays red, lightened on dark (`#F87171`) so it still reads on `#0B0C0F`.
- Inverse spinner is the opposite chip: dark tile on light, light tile on dark.

Toggle is global (`cx.theme()`, `cx.toggle_theme()`). Components read the active theme; `.theme(Theme::dark())` is an override, not the normal path.

---

## Page language (Paper)

One page per component. Two artboards: `{Name} / Light` and `{Name} / Dark`.

Shared skeleton:

1. Header — eyebrow, title, one sentence (max 460).
2. Sections — 13px label, then a row of specimens.
3. Last section — 80px bottom pad.

Buttons: Default → Outline → Ghost → With icons → Sizes → States.

Spinners: Sizes → Color → With label → In use.

Checkboxes / Switches / Radios: States → Disabled → With label → In use.

Overlays: Panel (or Default) → Overlay / Placement → In use.

Copy stays dry. No marketing flourish beyond the one-line promise.

---

## Platform deltas (do not “fix” with fakes)

| Paper | GPUI |
| --- | --- |
| `backdrop-filter: blur(20px) saturate(160–180%)` | No API. Alpha fills stand in. |
| Letter-spacing `0.04em` / `-0.03em` / `-0.01em` | Not exposed. |
| Static Inter | Vendored Inter Variable. |
| Window / artboard as designed | Window is opaque canvas; no frost unless asked. |

Do not rasterize text or controls to chase screenshot fidelity. Do not restore window blur to fake materials.

---

## Adding a component

1. Design it in Paper: one page, light + dark, this document’s type, radius, zinc, and red.
2. Read `get_jsx` (inline styles) and `get_computed_styles`. Screenshots are QA, not source.
3. Put semantic color in `theme` only if it is text/ground. Put material (fill, rim, shadow) next to the component, like `chrome.rs`.
4. Use exact Paper pixels. Reuse Button / Spinner / theme. Do not add a second radius or a second family.
5. Motion: critically damped by default; spinner-like status stays linear.
6. Gallery: reconstruct the Paper page 1:1, keep scroll, theme toggle, ⌘Q.

If a choice is not in this file and not in Paper, it does not belong in the kit.
