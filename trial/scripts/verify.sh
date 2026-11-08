#!/usr/bin/env bash
# The deterministic gate. No model is involved in pass/fail: Verus (Z3)
# checks the committed files, and a second machine must be able to re-run
# this script on the same commit and get the same verdict.
set -euo pipefail
cd "$(dirname "$0")/.."

# Verus shells out to rustup; self-heal PATH for terminals opened before the
# Rust install (new terminals get it from ~/.zshenv).
export PATH="$HOME/.cargo/bin:$PATH"
[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"

VERUS="${VERUS:-verus-release/verus}"
if [ ! -x "$VERUS" ]; then
  echo "verus not found. Run scripts/install-verus.sh (creates verus-release/) or set VERUS=/path/to/verus" >&2
  exit 2
fi

echo "verus binary: $VERUS"
"$VERUS" --version || true

echo
echo "== [1/2] toolchain smoke (self-contained file) =="
"$VERUS" scripts/verus_smoke.rs
echo "smoke: OK"

echo
echo "== [2/2] crate verification (backend, Task 1 wired) =="
# Requires PATH to contain verus-release/cargo-verus (see install-verus.sh).
# Plain `cargo test -p backend` is the behavioral gate; this is the proof gate.
# Bodies in verified/login.rs marked #[verifier::external_body] are trusted
# placeholders; removing one turns on its real proof obligation.
PATH="$PWD/verus-release:$PATH" cargo verus verify -p backend
