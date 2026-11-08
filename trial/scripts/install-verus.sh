#!/usr/bin/env bash
# Installs the prebuilt Verus binary for this machine.
# Source of truth: https://github.com/verus-lang/verus/releases
# Release used by this trial (recorded 2026-11-07): release/0.2026.11.02.8bf68ee
#
# Release layout (checked 2026-11-07): the zip unpacks to
#   verus-<VER>/verus-<arch>-<os>/{verus, cargo-verus, z3, rust_verify,
#   vstd.rlib, macos_allow_gatekeeper.sh, ...}
# This script symlinks that inner directory to ./verus-release so every
# other script and doc can use one stable path.
set -euo pipefail
cd "$(dirname "$0")/.."

# Verus shells out to rustup (rust_verify runs under a rustup toolchain), so
# rustup must be on PATH. A terminal opened before Rust was installed may lack
# it even though ~/.zshenv sets it for new shells — self-heal here.
export PATH="$HOME/.cargo/bin:$PATH"
[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"
if ! command -v rustup >/dev/null 2>&1; then
  echo "rustup not found. Install Rust first:" >&2
  echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" >&2
  exit 1
fi

VER="0.2026.11.02.8bf68ee"
case "$(uname -s)/$(uname -m)" in
  Darwin/arm64)   ASSET="verus-${VER}-arm64-macos.zip"; INNER="verus-arm64-macos" ;;
  Darwin/x86_64)  ASSET="verus-${VER}-x86-macos.zip";  INNER="verus-x86-macos" ;;
  Linux/aarch64)  ASSET="verus-${VER}-arm64-linux.zip"; INNER="verus-arm64-linux" ;;
  Linux/x86_64)   ASSET="verus-${VER}-x86-linux.zip";  INNER="verus-x86-linux" ;;
  *) echo "unsupported platform — pick the right asset from the releases page" >&2; exit 1 ;;
esac

URL="https://github.com/verus-lang/verus/releases/download/release/${VER}/${ASSET}"

if [ "${1:-}" = "--force" ] || [ ! -x verus-release/verus ]; then
  echo ">> downloading ${URL}"
  curl -fLO "$URL"
  rm -rf "verus-${VER}"
  unzip -oq "$ASSET" -d "verus-${VER}"
  rm "$ASSET"

  INNER_DIR="verus-${VER}/${INNER}"
  [ -f "$INNER_DIR/verus" ] || { echo "unexpected zip layout — no verus in $INNER_DIR" >&2; exit 1; }

  if [ -f "$INNER_DIR/macos_allow_gatekeeper.sh" ]; then
    echo ">> removing macOS Gatekeeper quarantine (documented Verus step)"
    (cd "$INNER_DIR" && bash macos_allow_gatekeeper.sh)
  fi

  rm -f verus-release
  ln -s "$INNER_DIR" verus-release
else
  echo ">> verus-release/verus already installed ('scripts/install-verus.sh --force' re-downloads)"
fi

# Verus runs its verifier under a pinned rustup toolchain, by exact name.
if ! rustup toolchain list | grep -q "^1\.99\.1-"; then
  echo ">> installing Verus's pinned toolchain (one-time)"
  rustup toolchain install 1.99.1
fi

echo ">> smoke:"
verus-release/verus --version

cat <<EOF

Done. Stable path for scripts and docs: verus-release/verus
Also shipped in this release: verus-release/cargo-verus (crate-mode
verification — this is what build-plan Task 1 will use) and verus-release/z3.

Record in intent/decisions.md (card D-toolchain): the verus version above,
the exact verify command you end up using, and the date. The deterministic
re-run depends on that record.
EOF
