#!/usr/bin/env bash
# rust-tool-gate — RUST-WASM-MIGRATION.md differential-verification gate.
#
# 1. cargo test (native unit tests: JS numeric semantics + tool goldens + ABI)
# 2. parity harness: legacy tools/<t>/calculator.js (CJS, node) vs toolcore
#    wasm (wasm-pack --target nodejs) over test-derived fixtures + boundaries
#    + 200 seeded-random inputs per tool; any diff -> exit 1.
#
# Run from the site root (or anywhere): bash scripts/rust-tool-gate.sh
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export PATH="$HOME/.cargo/bin:$PATH" # rustup toolchain owns wasm32 target

echo "[gate] 1/2 cargo test"
(cd "$ROOT/rust" && cargo test --quiet)

NODE_PKG="$ROOT/rust/toolcore/pkg-node/toolcore.js"
if [ ! -f "$NODE_PKG" ]; then
  echo "[gate] building nodejs-target wasm for the parity harness"
  (cd "$ROOT/rust" && wasm-pack build toolcore --release --target nodejs --out-dir pkg-node) >/dev/null
fi

echo "[gate] 2/2 parity harness (legacy JS vs wasm)"
node "$ROOT/rust/parity/run.mjs"
