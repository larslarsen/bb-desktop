#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

ELECTRON_PACKAGE="$PROJECT_ROOT/node_modules/electron"
ELECTRON_APP="$ELECTRON_PACKAGE/dist/Electron.app"
if [[ ! -f "$ELECTRON_PACKAGE/install.js" ]]; then
  echo "Electron is not installed. Run npm ci first." >&2
  exit 1
fi
if [[ ! -d "$ELECTRON_APP" ]]; then
  node "$ELECTRON_PACKAGE/install.js"
fi

case "$(uname -m)" in
  x86_64) PACKAGE_ARCH="x64" ;;
  arm64) PACKAGE_ARCH="arm64" ;;
  *)
    echo "Unsupported macOS architecture: $(uname -m)" >&2
    exit 1
    ;;
esac

VERSION="$(node -p "require('./package.json').version")"
OUTPUT_DIR="$PROJECT_ROOT/dist/BitBook-macos-$PACKAGE_ARCH"
APP_BUNDLE="$OUTPUT_DIR/BitBook.app"
DMG_PATH="$PROJECT_ROOT/dist/BitBook-$VERSION-macos-$PACKAGE_ARCH-unsigned.dmg"

rm -rf -- "$OUTPUT_DIR"
rm -f -- "$DMG_PATH"
install -d "$OUTPUT_DIR"
cp -R "$ELECTRON_APP" "$APP_BUNDLE"
mv "$APP_BUNDLE/Contents/MacOS/Electron" "$APP_BUNDLE/Contents/MacOS/BitBook"

PLIST="$APP_BUNDLE/Contents/Info.plist"
plutil -replace CFBundleDisplayName -string BitBook "$PLIST"
plutil -replace CFBundleName -string BitBook "$PLIST"
plutil -replace CFBundleExecutable -string BitBook "$PLIST"
plutil -replace CFBundleIdentifier -string com.bitbook.desktop "$PLIST"
plutil -replace CFBundleShortVersionString -string "$VERSION" "$PLIST"
plutil -replace CFBundleVersion -string "$VERSION" "$PLIST"

ICONSET="$OUTPUT_DIR/BitBook.iconset"
install -d "$ICONSET"
for ICON_SIZE in 16 32 128 256 512; do
  sips --resampleHeightWidth "$ICON_SIZE" "$ICON_SIZE" imgs/icon.png \
    --out "$ICONSET/icon_${ICON_SIZE}x${ICON_SIZE}.png" >/dev/null
  DOUBLE_SIZE=$((ICON_SIZE * 2))
  sips --resampleHeightWidth "$DOUBLE_SIZE" "$DOUBLE_SIZE" imgs/icon.png \
    --out "$ICONSET/icon_${ICON_SIZE}x${ICON_SIZE}@2x.png" >/dev/null
done
iconutil --convert icns "$ICONSET" \
  --output "$APP_BUNDLE/Contents/Resources/BitBook.icns"
rm -rf -- "$ICONSET"
plutil -replace CFBundleIconFile -string BitBook.icns "$PLIST"

APP_SOURCE="$APP_BUNDLE/Contents/Resources/app"
rm -f "$APP_BUNDLE/Contents/Resources/default_app.asar"
install -d "$APP_SOURCE/imgs"
sed -e "s/@VERSION@/$VERSION/g" packaging/runtime-package.json.in > "$APP_SOURCE/package.json"
install -m 0644 social-main.js "$APP_SOURCE/social-main.js"
cp -R social "$APP_SOURCE/social"
install -m 0644 imgs/icon.png "$APP_SOURCE/imgs/icon.png"

# Ad-hoc signing lets CI verify bundle integrity. Public releases must replace
# this with a Developer ID signature followed by Apple notarization.
codesign --force --deep --options runtime --sign - "$APP_BUNDLE"
codesign --verify --deep --strict "$APP_BUNDLE"
hdiutil create -volname BitBook -srcfolder "$APP_BUNDLE" -ov -format UDZO "$DMG_PATH"

echo "Built $DMG_PATH"
