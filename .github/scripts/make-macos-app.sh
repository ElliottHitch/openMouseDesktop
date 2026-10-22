#!/usr/bin/env bash
# Wraps a built openmouse-gpui binary into OpenMouse.app and a .dmg.
# usage: make-macos-app.sh <binary> <version> <output-dir>
set -euo pipefail

binary=$1
version=$2
out=$3
logo="$(dirname "$0")/../../gpui/assets/logo.png"

app="$out/OpenMouse.app"
rm -rf "$app" "$out/OpenMouse.dmg"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"
cp "$binary" "$app/Contents/MacOS/openmouse-gpui"

# The logo is portrait, so pad it to a square before making the icon set.
work=$(mktemp -d)
sips --padToHeightWidth 268 268 --padColor 000000 "$logo" --out "$work/square.png" >/dev/null
iconset="$work/OpenMouse.iconset"
mkdir "$iconset"
for size in 16 32 128 256 512; do
  sips -z $size $size "$work/square.png" --out "$iconset/icon_${size}x${size}.png" >/dev/null
  sips -z $((size * 2)) $((size * 2)) "$work/square.png" --out "$iconset/icon_${size}x${size}@2x.png" >/dev/null
done
iconutil -c icns "$iconset" -o "$app/Contents/Resources/OpenMouse.icns"

cat > "$app/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key><string>OpenMouse</string>
  <key>CFBundleDisplayName</key><string>OpenMouse Desktop</string>
  <key>CFBundleIdentifier</key><string>app.openmouse.desktop</string>
  <key>CFBundleExecutable</key><string>openmouse-gpui</string>
  <key>CFBundleIconFile</key><string>OpenMouse</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>$version</string>
  <key>CFBundleVersion</key><string>$version</string>
  <key>LSMinimumSystemVersion</key><string>11.0</string>
  <key>NSHighResolutionCapable</key><true/>
</dict>
</plist>
PLIST

# Ad-hoc signature: Apple Silicon refuses to run unsigned code. Not a
# substitute for a Developer ID signature and notarization.
codesign --force --deep --sign - "$app"

mkdir "$work/dmg-root"
cp -R "$app" "$work/dmg-root/"
hdiutil create -volname "OpenMouse" -srcfolder "$work/dmg-root" -ov -format UDZO "$out/OpenMouse.dmg" >/dev/null
