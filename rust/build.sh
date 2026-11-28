#!/usr/bin/env bash
# toolcore wasm build + size gate (RUST-WASM-MIGRATION.md P0/P1)
#
# 1. wasm-pack build --release --target web  -> rust/toolcore/pkg-web
# 2. copy glue + wasm into assets/wasm/      (deployment whitelist: assets/)
# 3. gzip size report; exit 1 if the wasm exceeds 250KB gzipped
#
# NOTE: prepend ~/.cargo/bin (rustup) — the wasm32-unknown-unknown target is
# installed for the rustup toolchain, not the Homebrew rust.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export PATH="$HOME/.cargo/bin:$PATH"

echo "[build] wasm-pack build --release --target web"
(cd "$ROOT/rust" && wasm-pack build toolcore --release --target web --out-dir pkg-web) >/dev/null

PKG="$ROOT/rust/toolcore/pkg-web"
DEST="$ROOT/assets/wasm"
mkdir -p "$DEST"
cp "$PKG/toolcore.js" "$PKG/toolcore_bg.wasm" "$DEST/"

WASM="$DEST/toolcore_bg.wasm"
GLUE="$DEST/toolcore.js"
WASM_RAW=$(wc -c < "$WASM" | tr -d ' ')
WASM_GZ=$(gzip -c "$WASM" | wc -c | tr -d ' ')
GLUE_GZ=$(gzip -c "$GLUE" | wc -c | tr -d ' ')
LIMIT=$((250 * 1024))

echo "[build] assets/wasm/toolcore_bg.wasm : raw $WASM_RAW bytes, gzip $WASM_GZ bytes"
echo "[build] assets/wasm/toolcore.js (glue): gzip $GLUE_GZ bytes"
echo "[build] loader+glue+wasm total gzip  : $((GLUE_GZ + WASM_GZ)) bytes (loader assets/toolcore.js is tiny)"

if [ "$WASM_GZ" -gt "$LIMIT" ]; then
  echo "[build] FAIL: toolcore_bg.wasm is $WASM_GZ bytes gzipped (> $LIMIT byte gate)" >&2
  exit 1
fi
echo "[build] size gate OK (wasm gzip $WASM_GZ <= $LIMIT)"
