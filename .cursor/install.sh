#!/usr/bin/env bash
# Idempotent Cloud Agent bootstrap for grafik-ui (GPUI Kit), a native Rust
# desktop component library built on Zed's GPUI framework.
set -euo pipefail

export DEBIAN_FRONTEND=noninteractive

# System libraries required to build and link GPUI on Linux, plus a software
# Vulkan driver (Mesa lavapipe) and Xvfb so the native gallery app can run
# headless in a Cloud Agent VM that has no GPU or physical display.
# --force-confold keeps existing config files so apt never blocks on a prompt.
sudo apt-get update
sudo apt-get install -y --no-install-recommends \
  -o Dpkg::Options::=--force-confold \
  libfontconfig-dev \
  libasound2-dev \
  libglib2.0-dev \
  libssl-dev \
  libva-dev \
  libvulkan1 \
  libwayland-dev \
  libx11-xcb-dev \
  libxkbcommon-x11-dev \
  libzstd-dev \
  libsqlite3-dev \
  libgit2-dev \
  pipewire \
  xdg-desktop-portal \
  mesa-vulkan-drivers \
  vulkan-tools \
  xvfb \
  clang \
  cmake \
  build-essential \
  pkg-config

# Warm the Cargo cache (including the pinned Zed git dependency) and compile the
# whole workspace and its test targets so later agents start from a hot build.
cargo fetch --locked
cargo build --workspace --all-targets
