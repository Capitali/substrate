#!/usr/bin/env bash
# make-icns.sh — regenerate packaging/AppIcon.icns from make-icon.swift.
# Run this only when the icon art changes; the .icns is committed so build-app.sh needs no
# Swift toolchain just to assemble the bundle.
set -euo pipefail
cd "$(dirname "$0")"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

swiftc -O make-icon.swift -o "$tmp/make-icon"
"$tmp/make-icon" "$tmp/AppIcon.iconset"
iconutil -c icns "$tmp/AppIcon.iconset" -o AppIcon.icns
echo "wrote $(pwd)/AppIcon.icns"
