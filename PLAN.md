# GPUI Kit — Plan

What to design in Paper and implement in GPUI next. Visual rules: [`DESIGN.md`](./DESIGN.md).

**Paper first.** Finish remaining Paper pages (light + dark) for the planned kit before more GPUI. Gallery reconstruction comes after that queue. Skip the “Not unless asked” list.

---

## Done

Paper page **and** gallery reconstruction.

| Component | Paper page | Notes |
| --- | --- | --- |
| Button | Buttons | Six variants, three sizes, icon, disabled, loading, group |
| Spinner | Spinners | Track + 120° arc, sizes, tones, labeled, in-use |
| Input | Inputs | 280×36, outline glass. Focus zinc ring, disabled, invalid. Shared page with Textarea. |
| Textarea | Inputs | Same chrome, 320×96, line-height 20. Wrap is display; caret is single-line only. |
| Label | Labels | 13/500 above the control. Required `*`, Optional 12/500. |
| Checkbox | Checkboxes | 16×16, radius 6. Off / on / mixed. Disabled, labeled, nested Export all. |
| Switch | Switches | 36×20 pill, 16px thumb, 2px inset. Instant fill, no bounce. |
| Radio | Radios | 16×16 circle, radius 8. Outline glass off, primary glass on. 6px dot. |

---

## Paper ready

Designed in Paper. Gallery reconstruction waits until this queue is finished.

| Component | Paper page | Notes |
| --- | --- | --- |
| Select | Selects | Closed: Input chrome + chevron, 280×36. Open: secondary glass list, radius 6. Check on selected. |
| Kbd | Kbd | Ghost glass chip, 22 tall, 12/500. ⌘K / ⌘Q / Esc. Hint and menu rows. |
| Separator | Separators | 1px zinc at 12%. Horizontal 280, vertical 36. In-use stack and toolbar. |

---

## Now — Paper queue

Still foundation. Next Paper page:

1. **Badge** — Default, muted, destructive. Height ~20–22, radius 6. For counts and status, not buttons.

---

## Next

Overlay and feedback. Needs Button, Input, Spinner, Kbd.

11. **Tooltip** — Delay ~300ms, critically damped 0.3s, same path in and out. Inverse chip (dark on light, light on dark). Never clip under the cursor.
12. **Progress** — Determinate bar (and optional circular using Spinner geometry). 0 / 40 / 100. Linear fill, no bounce.
13. **Skeleton** — Pulse on secondary glass. Text line, avatar circle, control-sized block. Reduced motion: static.
14. **Dialog** — Centered panel, radius 10, dim scrim (not a second glass layer). Title, body, footer with Ghost + Primary. Esc and overlay click dismiss. Spring 0.3s, damping 1.0; interruptible.
15. **Alert dialog** — Dialog with Destructive confirm. Title states the irreversible action. No cute copy.
16. **Popover** — Anchored to the trigger, origin at the source. Secondary glass, radius 6. Same enter/exit path.
17. **Dropdown menu** — Popover + items: default, destructive, disabled, shortcut (`Kbd`), separator, nested. Arrow keys.
18. **Context menu** — Dropdown that opens at the pointer. Same items.
19. **Command** — `⌘K` palette. Input on top, grouped rows, spinner while loading, empty state. This is the native-app moment.
20. **Toast** — Quiet, bottom or top-end. Success (ink), destructive, with optional action. Stack, do not bounce. Auto-dismiss; hover pauses.

---

## Later

Layout and data, once forms and overlays exist.

21. **Card** — Radius 10 panel (same as spinner empty state). Header, body, footer. No extra shadow language.
22. **Tabs** — Underline or ghost-pill. Selected is ink; the rest is `label`. Keyboard left/right.
23. **Accordion** — One open, interruptible height. Chevron rotates with the content, not as a separate gag.
24. **Avatar** — Image, initials, fallback. Sizes 24 / 32 / 40. Radius full (circle) — a face, like Radio, not a rounded-square control.
25. **Breadcrumb** — Ghost links, chevron-right, current page as ink.
26. **Pagination** — Ghost icon buttons + page number. Disabled at ends.
27. **Table** — Header, row hover (ghost fill), selected row, empty + spinner. Not a spreadsheet.
28. **Slider** — 1:1 with the pointer while dragging; spring only on release if we snap. Keyboard.
29. **Combobox** — Input + filtered list (Select + Command patterns).
30. **Menubar** — App-level File / Edit. Native menu is already used in the gallery; this is in-window if an app needs it.

---

## Not unless asked

These are shadcn staples that do not earn a page in a native GPUI kit yet:

- Carousel, chart, calendar, date picker, OTP, sidebar, drawer/sheet, resizable panels, hover card, toggle group, sonner-as-a-brand.

If an app needs one, design that one page. Do not batch them in.

---

## Theme gaps (add only when a component needs them)

Do not pre-create tokens. Input already landed these:

- `placeholder` — same as `label` (`#A1A1AA` light / `#71717A` dark)
- focus ring — material in `field_chrome`, not a theme token. Light: extra `#18181B24` 0 0 0 3px. Dark: `#FFFFFF24` 3px.
- `invalid` — light `#FEE2E26B` / `#FECACAB3`; dark `#7F1D1D47` / `#F8717147`. Helper uses `destructive`.

Focus must be visible. Do not rely on hover alone.

---

## Gallery

Each new component:

- One gallery page, 1:1 with Paper
- Keep scroll, theme toggle (`⌘D`), quit (`⌘Q`)
- Page switcher stays (Buttons, Spinners, Inputs, Labels, Checkboxes, Switches, Radios, …)

Window stays opaque. No frost unless asked.
