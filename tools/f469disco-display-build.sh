#!/usr/bin/env bash
# Build all F469-DISCO display examples, copy to dist/, optionally flash.
#
# Usage:
#   ./tools/f469disco-display-build.sh              # build + copy to dist/f469disco/
#   ./tools/f469disco-display-build.sh --size       # use release-size profile (smaller)
#   ./tools/f469disco-display-build.sh --flash      # build, copy, then flash first example
#   ./tools/f469disco-display-build.sh --flash slideshow  # flash specific example
#
# Flashing uses probe-rs (cargo install probe-rs --features cli). Alternative:
#   probe-rs run target/.../release/examples/f469disco-slideshow

set -e

FEATURES="stm32f469,stm32-fmc,framebuffer,defmt"
EXAMPLES=(f469disco-slideshow f469disco-animated-layers f469disco-paint f469disco-image-slider)
PROFILE="release"
DIST_DIR="dist/f469disco"
DO_FLASH=""
FLASH_EXAMPLE=""

while [[ $# -gt 0 ]]; do
  case $1 in
    --size)   PROFILE="release-size"; shift ;;
    --flash)  DO_FLASH=1; shift ;;
    slideshow|animated-layers|paint|image-slider)
              DO_FLASH=1; FLASH_EXAMPLE="f469disco-$1"; shift ;;
    *)        echo "Unknown option: $1"; exit 1 ;;
  esac
done

cd "$(dirname "$0")/.."
mkdir -p "$DIST_DIR"

echo "Building F469 display examples (profile=$PROFILE)..."
for ex in "${EXAMPLES[@]}"; do
  cargo build --profile "$PROFILE" --example "$ex" --features "$FEATURES"
done

TARGET_DIR="target/thumbv7em-none-eabihf/$PROFILE/examples"
echo "Copying to $DIST_DIR/ ..."
for ex in "${EXAMPLES[@]}"; do
  if [[ -f "$TARGET_DIR/$ex" ]]; then
    cp "$TARGET_DIR/$ex" "$DIST_DIR/"
    arm-none-eabi-objcopy -O binary "$TARGET_DIR/$ex" "$DIST_DIR/${ex}.bin" 2>/dev/null || true
    SIZE=$(ls -l "$DIST_DIR/$ex" | awk '{print $5}')
    echo "  $ex  $(numfmt --to=iec-i --suffix=B $SIZE 2>/dev/null || echo "${SIZE} bytes")"
  fi
done

if [[ -n "$DO_FLASH" ]]; then
  if ! command -v probe-rs &>/dev/null; then
    echo "probe-rs not found. Install with: cargo install probe-rs --features cli"
    echo "Then run: probe-rs run $DIST_DIR/f469disco-slideshow"
    exit 1
  fi
  EX="${FLASH_EXAMPLE:-f469disco-slideshow}"
  if [[ ! -f "$DIST_DIR/$EX" ]]; then
    echo "Example binary not found: $DIST_DIR/$EX"
    exit 1
  fi
  echo "Flashing $EX ..."
  probe-rs run "$DIST_DIR/$EX"
fi

echo "Done. Binaries in $DIST_DIR/"
