# BitBook Desktop

BitBook Desktop is the wallet-free client for the BitBook distributed social
network. It provides profiles, signed posts, follows, peer status, and signed
direct chat without starting any of OpenBazaar's marketplace or Bitcoin UI.

The historical OpenBazaar renderer remains in this repository as migration
reference, but the default Electron entry point is the small client in
[`social/`](social).

## Run it

First start the maintained Go daemon:

```sh
cd ../bb-go/modern
GOTOOLCHAIN=go1.27.0 go run ./cmd/bitbookd
```

For source-tree development, install and start the desktop client:

```sh
npm install
npm start
```

Development requires Node.js 22.12 or newer. The package pins Electron 44 so
fresh installs use a maintained browser runtime rather than OpenBazaar's old
Electron 6 stack. On Linux, `npm start` uses Chromium's `--no-sandbox` switch
only for local development because unpacked npm binaries cannot be installed
with the root-owned helper Chromium requires. Do not distribute that command.

## Linux package

Build the production Debian package with:

```sh
npm ci
npm run package:linux
```

The result is `dist/bitbook_<version>_<architecture>.deb`. Install it through
your desktop software center or with `apt install ./dist/bitbook_*.deb`. The
package records `chrome-sandbox` as root-owned mode `4755`, so the installed app
runs with Chromium's full process sandbox and users perform no manual setup.
BitBook itself always runs as the signed-in user; only Chromium's small sandbox
launcher briefly holds the privilege required to create the containment layer.

## Windows and macOS test packages

CI also builds native, unsigned test artifacts for Windows x64 and macOS on
both Apple Silicon and Intel:

```sh
npm run package:windows # on Windows
npm run package:macos   # on macOS
```

These artifacts prove that the native bundles start and are intentionally named
`unsigned`. They are for development and private testing, not public release.
Windows distribution still needs Microsoft Store or Artifact Signing credentials.
macOS distribution still needs an Apple Developer ID signature and notarization.
Once those accounts exist, signing can be added to the existing OS-native CI jobs
without changing the application payload.

The client connects to `http://127.0.0.1:4002` by default. Click the connection
indicator to point it at a different daemon. A non-local daemon should be
protected with transport security and authentication before it is exposed to
the internet.

## Current scope

- Persistent libp2p identity and peer discovery
- Signed, IPNS-published profiles and posts
- Signed follow and unfollow notifications
- Signed direct chat, offline queuing, typing indicators, and read receipts
- Live updates over the daemon WebSocket
- No marketplace, order, listing, wallet, exchange-rate, or Bitcoin flows

Payment support is intentionally not part of this client. Any future tipping
feature should use a narrow coin-agnostic integration so that privacy and
network choices can be made explicitly instead of inheriting OpenBazaar's
Bitcoin assumptions.

## Development checks

The maintained client is plain browser JavaScript and requires no build-time
transpilation:

```sh
npm run build
npm run test:social
npm run package:linux
```

The archived OpenBazaar renderer remains in the Git history and source tree for
migration reference, but its abandoned dependency graph is no longer installed
or loaded by the BitBook package.

## License

[MIT](LICENSE)
