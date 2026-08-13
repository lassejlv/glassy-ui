#!/usr/bin/env bash
# Per-boot runtime setup: bring up a virtual X display so the native GPUI
# gallery can be run and screenshotted headlessly. Building/testing the
# workspace does not need this; it only matters when running the GUI.
#
# To run the gallery against this display:
#   XDG_RUNTIME_DIR=/tmp/xdg-runtime DISPLAY=:99 \
#     VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json \
#     cargo run --bin gpui-kit-gallery
set -euo pipefail

export XDG_RUNTIME_DIR="${XDG_RUNTIME_DIR:-/tmp/xdg-runtime}"
mkdir -p "$XDG_RUNTIME_DIR"
chmod 700 "$XDG_RUNTIME_DIR"

# Idempotent: only launch Xvfb if display :99 is not already responding.
if ! xdpyinfo -display :99 >/dev/null 2>&1; then
  rm -f /tmp/.X99-lock
  Xvfb :99 -screen 0 1440x900x24 -ac +extension GLX +render -noreset \
    >/tmp/xvfb.log 2>&1 &
fi
