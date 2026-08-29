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

Then install and start the desktop client:

```sh
npm install
npm start
```

Development requires Node.js 22.12 or newer. The package pins Electron 44 so
fresh installs use a maintained browser runtime rather than OpenBazaar's old
Electron 6 stack.

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
```

The archived OpenBazaar renderer remains in the Git history and source tree for
migration reference, but its abandoned dependency graph is no longer installed
or loaded by the BitBook package.

## License

[MIT](LICENSE)
