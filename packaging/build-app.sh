#!/usr/bin/env bash
# build-app.sh — assemble (and sign) Familiar.app from a release build.
#
# Produces dist/Familiar.app, a menu-bar accessory bundle whose entry point is the marble.
# All four binaries live in Contents/MacOS together, so the marble's sibling-resolution finds
# the Glass, the familiar daemon, and the familiar-eye camera helper inside the bundle — and,
# crucially, the macOS camera grant (TCC) attaches to *this bundle's* identity rather than to
# whatever terminal launched a build.
#
# Signing: pass a Developer ID / Apple Development identity as $SIGN_IDENTITY for a stable,
# distributable signature; otherwise it falls back to an ad-hoc signature ("-"), which is fine
# for running locally but gives the bundle a new identity on each rebuild (so the camera grant
# must be re-approved after a rebuild — see the installer notes).
set -euo pipefail

cd "$(dirname "$0")/.."
ROOT="$(pwd)"
APP="$ROOT/dist/Familiar.app"
MACOS="$APP/Contents/MacOS"
BINS=(marble glass familiar familiar-eye)
SIGN_IDENTITY="${SIGN_IDENTITY:--}"  # default: ad-hoc

echo "==> building release binaries"
cargo build --release

# familiar-eye is compiled by the vision crate's build.rs during the release build; verify it.
if [[ ! -x "$ROOT/target/release/familiar-eye" ]]; then
  echo "!! target/release/familiar-eye missing — is the Swift toolchain (swiftc) installed?" >&2
  echo "   The app will build without camera capture." >&2
fi

echo "==> assembling $APP"
rm -rf "$APP"
mkdir -p "$MACOS"
cp "$ROOT/packaging/Info.plist" "$APP/Contents/Info.plist"
for b in "${BINS[@]}"; do
  if [[ -x "$ROOT/target/release/$b" ]]; then
    cp "$ROOT/target/release/$b" "$MACOS/$b"
  else
    echo "   (skipping $b — not built)" >&2
  fi
done

echo "==> signing (identity: $SIGN_IDENTITY)"
# --deep so the helper executables beside the main one in Contents/MacOS (glass, familiar,
# familiar-eye) are each signed and sealed into the bundle. (--deep is fine for ad-hoc/local
# signing; a notarized distribution build would sign each component explicitly instead.)
codesign --force --deep --sign "$SIGN_IDENTITY" "$APP"

echo "==> verifying"
codesign --verify --deep --strict --verbose=2 "$APP"
echo
echo "Built: $APP"
echo "Run:   open '$APP'    (or: '$MACOS/marble' install  to set up the login item)"
