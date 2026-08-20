# GPUI 0.2.2 compatibility gates

- [x] G1: The library and gallery compile against crates.io GPUI 0.2.2.
  CHECK: cargo check --all-targets
  EVIDENCE: PASS — `cargo check --all-targets` exited 0.

- [x] G2: Existing unit and GPUI interaction tests pass.
  CHECK: cargo test
  EVIDENCE: PASS — all 81 library and interaction tests passed; 4 documentation tests remain intentionally ignored.

- [x] G3: Formatting is clean.
  CHECK: cargo fmt --all -- --check
  EVIDENCE: (no output)

- [x] G4: Clippy reports no warnings across all targets.
  CHECK: cargo clippy --workspace --all-targets -- -D warnings
  EVIDENCE: PASS — Clippy exited 0 with `-D warnings`; Cargo separately reports a future-incompatibility notice for GPUI's transitive `block` 0.1.6 dependency.

- [x] G5: The gallery starts successfully with GPUI 0.2.2 and stays alive long enough to open its window.
  EVIDENCE: PASS — `cargo run --bin glassy-gallery` opened the real macOS gallery window; `/tmp/glassy-gpui-0.2.2-front.png` was visually inspected before the process was stopped.

- [x] G6: The dependency graph contains registry GPUI 0.2.2 and no Zed Git GPUI dependency.
  CHECK: cargo tree -i gpui
  EXPECT: gpui v0.2.2
  EVIDENCE: PASS — `cargo tree -i gpui` resolves `gpui v0.2.2`; `Cargo.lock` records the crates.io registry source and no Zed Git source remains.
