#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

ELECTRON_DIST="$PROJECT_ROOT/node_modules/electron/dist"
if [[ ! -f "$PROJECT_ROOT/node_modules/electron/install.js" ]]; then
  echo "Electron is not installed. Run npm ci first." >&2
  exit 1
fi
if [[ ! -x "$ELECTRON_DIST/electron" || ! -f "$ELECTRON_DIST/chrome-sandbox" ]]; then
  node "$PROJECT_ROOT/node_modules/electron/install.js"
fi

case "$(uname -m)" in
  x86_64) DEB_ARCH="amd64" ;;
  aarch64) DEB_ARCH="arm64" ;;
  *)
    echo "Unsupported Linux architecture: $(uname -m)" >&2
    exit 1
    ;;
esac

VERSION="$(node -p "require('./package.json').version")"
BUILD_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/bitbook-deb.XXXXXX")"
trap 'rm -rf -- "$BUILD_ROOT"' EXIT

PACKAGE_ROOT="$BUILD_ROOT/root"
APP_DIR="$PACKAGE_ROOT/usr/lib/bitbook"
APP_SOURCE="$APP_DIR/resources/app"

install -d \
  "$PACKAGE_ROOT/DEBIAN" \
  "$APP_SOURCE/imgs" \
  "$PACKAGE_ROOT/usr/bin" \
  "$PACKAGE_ROOT/usr/share/applications" \
  "$PACKAGE_ROOT/usr/share/doc/bitbook" \
  "$PACKAGE_ROOT/usr/share/icons/hicolor/512x512/apps"

cp -a "$ELECTRON_DIST/." "$APP_DIR/"
mv "$APP_DIR/electron" "$APP_DIR/bitbook"
rm -f "$APP_DIR/resources/default_app.asar"

sed -e "s/@VERSION@/$VERSION/g" packaging/runtime-package.json.in > "$APP_SOURCE/package.json"
install -m 0644 social-main.js "$APP_SOURCE/social-main.js"
cp -a social/. "$APP_SOURCE/social/"
install -m 0644 imgs/icon.png "$APP_SOURCE/imgs/icon.png"
install -m 0644 LICENSE "$PACKAGE_ROOT/usr/share/doc/bitbook/copyright"
install -m 0644 packaging/linux/bitbook.desktop "$PACKAGE_ROOT/usr/share/applications/bitbook.desktop"
install -m 0644 imgs/icon-512.png "$PACKAGE_ROOT/usr/share/icons/hicolor/512x512/apps/bitbook.png"
ln -s ../lib/bitbook/bitbook "$PACKAGE_ROOT/usr/bin/bitbook"

find "$PACKAGE_ROOT" -type d -exec chmod 0755 {} +
find "$APP_SOURCE" -type f -exec chmod 0644 {} +

# dpkg-deb records this mode with root ownership. The installed helper creates
# Chromium's sandbox and immediately drops privilege; BitBook itself never runs
# as root.
chmod 4755 "$APP_DIR/chrome-sandbox"

INSTALLED_SIZE="$(du -sk "$PACKAGE_ROOT/usr" | awk '{print $1}')"
sed \
  -e "s/@VERSION@/$VERSION/g" \
  -e "s/@ARCH@/$DEB_ARCH/g" \
  -e "s/@INSTALLED_SIZE@/$INSTALLED_SIZE/g" \
  packaging/linux/control.in > "$PACKAGE_ROOT/DEBIAN/control"
chmod 0644 "$PACKAGE_ROOT/DEBIAN/control"

install -d "$PROJECT_ROOT/dist"
OUTPUT="$PROJECT_ROOT/dist/bitbook_${VERSION}_${DEB_ARCH}.deb"
dpkg-deb --build --root-owner-group "$PACKAGE_ROOT" "$OUTPUT"

echo "Built $OUTPUT"
