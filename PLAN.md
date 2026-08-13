# Grafik — Plan

What to design in Paper and implement in GPUI next. Visual rules: [`DESIGN.md`](./DESIGN.md).

**Paper queue is empty.** Planned kit has light + dark pages. Gallery reconstruction is next. Skip the “Not unless asked” list.

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
| Select | Selects | Closed: Input chrome + chevron, 280×36. Open: secondary glass list, radius 6. Check on selected. |
| Kbd | Kbd | Ghost glass chip, 22 tall, 12/500. ⌘K / ⌘Q / Esc. Hint and menu rows. |
| Separator | Separators | 1px zinc at 12%. Horizontal 280, vertical 36. In-use stack and toolbar. |
| Skeleton | Skeletons | Secondary glass pulse. Text line 180×12, avatar 32 circle, control 280×36. Reduced motion holds frame one. |
| Tooltip | Tooltips | Inverse chip, 24 tall, radius 6. 300ms hover delay. Above / Below / Start / End. |
| Progress | Progress | Linear 280×8 outline track + primary fill. 0 / 40 / 100. Circular 24px ring. |

---

## Paper ready

Designed in Paper. Gallery reconstruction waits until this queue is finished.

| Component | Paper page | Notes |
| --- | --- | --- |
| Badge | Badges | Height 22, radius 6. Default primary, muted ghost, destructive. Counts 3 / 12 / 128. |
| Dialog | Dialogs | Radius 10 panel, dim scrim. Title, field, Ghost + Primary. Esc and overlay dismiss. |
| Alert dialog | Alert dialogs | Same panel. Title is the action. No field. Ghost + Destructive. |
| Popover | Popovers | Secondary glass, radius 6. Origin at the trigger. Page meta card. |
| Dropdown menu | Dropdown menus | Items: hover, nested, separator, disabled, destructive, Kbd. File trigger. |
| Context menu | Context menus | Same items as Dropdown. Opens at the pointer. |
| Command | Command | ⌘K palette. Grouped rows, loading spinner, empty state. |
| Toast | Toasts | 40 tall. Success, destructive, with Undo. Stacked bottom-end. |
| Card | Cards | Radius 10 panel. Header, body, Ghost + Primary footer. |
| Tabs | Tabs | Underline or ghost pill. Selected is ink. |
| Accordion | Accordions | One open. Chevron turns with the content. |
| Avatar | Avatars | Circle. 24 / 32 / 40. Image, initials, fallback. |
| Breadcrumb | Breadcrumbs | Ghost links, chevron-right, current page as ink. Home / Pages / Buttons. |
| Pagination | Pagination | Ghost 36 icon buttons + page number. Primary current. Disabled at ends. |
| Table | Tables | Header, hover ghost fill, selected outline row. Empty + spinner. Page / Size / Layers. |
| Slider | Sliders | 280×8 pill track, 16 circle thumb. 1:1 with the pointer. |
| Combobox | Comboboxes | Input chrome + filtered list. Closed Pages, open “But”, empty xyzzy. |
| Menubar | Menubars | In-window File / Edit / View. File menu: New file ⌘N, Duplicate, Delete page. |

---

## Now — Gallery

Paper is done. Reconstruct gallery pages 1:1 with Paper, in Paper-ready order.

---

## Later

Empty. The planned kit is Paper-ready. Gallery is Now.

---

## Not unless asked

These are shadcn staples that do not earn a page in Grafik yet:

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
- Page switcher stays (Buttons, Skeletons, Tooltips, Progress, Spinners, Inputs, Labels, Checkboxes, Switches, Radios, Selects, Kbd, Separator, …)

Window stays opaque. No frost unless asked.
