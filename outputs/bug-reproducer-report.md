# Bug Reproducer

## ✅ FIX_PROVEN — Bug reproduced and fix proven

> The same reproducer changed from failing to passing and broader checks passed.

**Project:** Grafik  
**Bug:** Select menus reflow content and do not dismiss correctly  
**Environment:** macOS arm64, Rust 2021, GPUI 0.2.2 pinned at Zed revision 101ca00  
**Generated:** 2026-08-13

## Discovery scope

- /Users/lassevestergaard/Dev/gpui-kit/crates/ui/src/select.rs
- /Users/lassevestergaard/Dev/gpui-kit/crates/gallery/src/main.rs
- Native Grafik gallery interactions in light mode

## Ranked and tested candidates

| # | Candidate | Contract evidence | Trigger | Location | Confidence | Outcome |
|---:|---|---|---|---|---|---|
| 1 | Opening a Select menu reflows surrounding content | A Select menu is a popup and must not alter surrounding layout geometry. | Open a Select above a sentinel element. | /Users/lassevestergaard/Dev/gpui-kit/crates/ui/src/select.rs:365 | high | REPRODUCED |
| 2 | Opening another Select leaves the first menu open | Only the active Select menu should remain open. | Open two non-overlapping Select controls in sequence. | /Users/lassevestergaard/Dev/gpui-kit/crates/ui/src/select.rs:296 | high | REPRODUCED |
| 3 | Outside clicks do not dismiss an open Select | Clicking outside a transient menu dismisses it. | Open a Select and click blank window space. | /Users/lassevestergaard/Dev/gpui-kit/crates/ui/src/select.rs:296 | high | REPRODUCED |
| 4 | Escape and arrow-key selection are missing | The gallery explicitly promises arrows, Enter, and Escape. | Focus an open Select and press Escape or Down then Enter. | /Users/lassevestergaard/Dev/gpui-kit/crates/gallery/src/main.rs:1547 | high | REPRODUCED |

## Original report

The user reported that the Select menus had some bugs and asked for discovery and repair.

| Contract | Expected | Actual |
|---|---|---|
| Observed behavior | The menu is an 8 px anchored popup, only one remains open, outside click and Escape dismiss it, and arrows plus Enter select enabled items. | The list participated in flex layout, multiple menus remained open, outside click and Escape did nothing, and there was no keyboard path. |

## Minimal reproduction

A GPUI visual test window renders two Select controls and a sentinel, then dispatches real mouse and keyboard events while asserting layout bounds and selection callbacks.

**Confirming signal:** All four original interaction assertions failed; opening moved the sentinel from y=104px to y=218px.

### Reproduction files approved at Gate 1

- [select_interaction.rs](/Users/lassevestergaard/Dev/gpui-kit/crates/ui/tests/select_interaction.rs:77) — Approved GPUI interaction and layout regression tests.
- [Cargo.toml](/Users/lassevestergaard/Dev/gpui-kit/crates/ui/Cargo.toml:12) — Approved GPUI test-support development feature.

## Red to green evidence

| Evidence | Before fix | After fix |
|---|---:|---:|
| Exit code | 101 | 0 |
| Timed out | False | False |
| Duration | 18,109.143 ms | 275.644 ms |
| Same command | — | True |
| Broader suite | — | passed |

### Before — failing evidence

```text
running 4 tests
test opening_menu_does_not_reflow_surrounding_content ... FAILED
test outside_click_closes_select ... FAILED
test escape_closes_select ... FAILED
test opening_another_select_closes_the_first ... FAILED

opening_menu_does_not_reflow_surrounding_content: sentinel moved from y=104px to y=218px
outside_click_closes_select: outside click should close the menu
escape_closes_select: escape should close the menu
opening_another_select_closes_the_first: the first menu should have closed

test result: FAILED. 0 passed; 4 failed
```

### After — fixed evidence

```text
Finished ˋtestˋ profile [unoptimized + debuginfo] target(s) in 0.21s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option ˋ--future-incompat-reportˋ, or run ˋcargo report future-incompatibilities --id 2ˋ
     Running tests/select_interaction.rs (target/debug/deps/select_interaction-6b7d3a67bee8a3c5)
```

## Root cause

Select rendered its list as an ordinary flex child and stored only local open/value state. It had no deferred anchored layer, focus handle, keyboard highlight state, or outside-click dismissal handler.

## Approved fix

Rendered the list through GPUI deferred anchored positioning with an exact 8 px gap, added outside-click dismissal that naturally closes other menus, and added focus, arrow navigation, Enter/Space selection, Escape dismissal, and disabled-item skipping.

**Why this is causal:** The list no longer contributes to parent layout, and the new event paths directly own every previously missing transition.

### Production files approved at Gate 2

- [select.rs](/Users/lassevestergaard/Dev/gpui-kit/crates/ui/src/select.rs:25) — Approved anchored popup, dismissal, focus, and keyboard behavior.

## Verification

| Check | Status | Evidence |
|---|---|---|
| Select regression test | ✅ passed | Six tests pass, including exact 8 px placement and Arrow plus Enter. |
| Workspace tests | ✅ passed | All motion, theme, UI, and Select tests pass. |
| Strict clippy | ✅ passed | cargo clippy --workspace --all-targets -- -D warnings |
| Native QA | ✅ passed | Verified anchored placement, no reflow, outside click, Escape, and Down plus Enter selecting SVG. |

## Reproduce

```bash
cargo test -p grafik-ui --test select_interaction
```
```bash
cargo test --workspace
```
```bash
cargo clippy --workspace --all-targets -- -D warnings
```

## Limitations

- The native GPUI controls are not exposed individually through the macOS accessibility tree, so native QA used screenshots and coordinates.

## Residual risks

- Near-window-edge flipping relies on GPUI's pinned anchored primitive and was not separately exercised in the gallery.

## Notes

- No public Select API changed.
- Pre-existing uncommitted project work was preserved.

---

Generated by `$bug-reproducer`. A fix is proven only by the same red-to-green reproducer plus relevant broader checks.
