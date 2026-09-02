# BBD-WAL-007 — Broker-Managed Monero Wallet-RPC Viewing and Subaddress Adapter

Status: READY FOR AUTHORIZATION — NO SOURCE, EXECUTION, OR INTEGRATION AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Planned test and production source actor: Principal Dev — Codex Sol
(`gpt-5.6-sol`, High)

Planned integration actor: Jr Dev — Hermes

Source baseline: `c8b52ae69db8a563a0daf87af74bc09bd3ce97e2`

Architecture: `docs/architecture/BBD-WAL-001-REVIEW.md` and
`docs/architecture/BBD-WAL-007-XMR-RPC-DISTRIBUTION-DECISION.md`

Cross-repository baseline: `../bb-go` accepted BBGO-PAY-001 production at
`6bbb0629cb0dfaca9958f9cac7d57216760630ae` and final evidence at
`801f5d55d80fe02c6eb512ff35f8c09acfd679af`. This ticket does not edit that
repository. `../go-ipfs` remains deprecated.

## Security invariant

Only the Rust wallet broker may select, verify, start, authenticate, call, monitor, and
stop a pinned `monero-wallet-rpc`; it may connect that child only to the user's local,
non-bootstrap `monerod`, and it must return only account viewing state and a durably
request-bound fresh subaddress. Missing installation, process failure, remote/bootstrap
node behavior, authentication failure, persistence ambiguity, mainnet, or any spend RPC
fails closed without affecting ZEC or social behavior.

## Objective

Add the first maintained XMR adapter beneath the broker protocol. On XMR stagenet and
testnet it must:

1. let the broker-native surface select a normal or portable official Monero
   `monero-wallet-rpc` installation and persist only that explicit selection;
2. verify the executable against a reviewed release/size/SHA-256 pin before every
   launch, with no PATH search, download, replacement, or fallback;
3. probe only the user's loopback `monerod`, reject a current bootstrap/remote response,
   and preserve independent node, wallet, lock, and device states;
4. run a broker-owned full wallet RPC on a random loopback port with broker-generated
   Digest credentials that never enter argv, environment, Electron, or a renderer;
5. create, recover, open, refresh, and close broker-owned software or watch-only wallet
   files through a closed RPC method set and WAL-004 encrypted custody;
6. report exact atomic balances and non-secret sync/capability state; and
7. create a fresh XMR subaddress for `receiver.fresh`, durably bind it to `request_id`,
   and return the same binding on an exact replay.

This is a viewing, recovery, and receive adapter. It creates no transaction, signs
nothing, submits nothing, and cannot move funds. The account capability surface keeps
`can_prepare_tx=false`, `can_sign_spend=false`, and `can_broadcast=false` even for a
software wallet. BBD-WAL-009 owns XMR prepare, native confirmation, signing,
post-sign verification, submission, concurrency, cancellation races, and crash recovery.

## Fixed upstream and release pin

The first implementation is Linux x86-64 only. Other platforms return XMR `UNAVAILABLE`
until a later reviewer-approved pin is added; code may not infer a platform variant.
Normal and portable GUI installations are the same contract because verification is of
the selected executable bytes, not its parent directory.

The reviewed source is the official Monero GUI release page and canonical signed hash
list:

- <https://github.com/monero-project/monero-gui/releases/tag/v0.18.5.2>
- <https://www.getmonero.org/downloads/hashes.txt>
- <https://docs.getmonero.org/interacting/overview/>
- <https://docs.getmonero.org/interacting/monero-wallet-rpc-reference/>
- <https://docs.getmonero.org/interacting/monero-config-file/>
- <https://docs.getmonero.org/rpc-library/wallet-rpc/>
- <https://docs.getmonero.org/rpc-library/monerod-rpc/>

The exact Linux pin is:

| Artifact | Exact value |
| --- | --- |
| Official archive | `monero-gui-linux-x64-v0.18.5.2.tar.bz2` |
| Official archive SHA-256 | `294017a5aa1ee86420b0c62fe4046000f42438375a8559d9ff55e41e5c6cbbcd` |
| Wallet RPC archive member | `monero-gui-v0.18.5.2/extras/monero-wallet-rpc` |
| Wallet RPC byte length | `29026368` |
| Wallet RPC SHA-256 | `c1e3aff7c72837e6f29045c439b772a82b5cd7324c8b831fa825a6ce2019a656` |
| Verified binary version output | `Monero 'Fluorine Fermi' (v0.18.5.1-release)` |
| Gate-only `monerod` archive member | `monero-gui-v0.18.5.2/monerod` |
| Gate-only `monerod` byte length | `24112840` |
| Gate-only `monerod` SHA-256 | `9b3b2676ea7868c1a7186feea9569c2cf7683ae79d2fcc769c846a91c810a1f5` |

The different GUI archive and embedded core version numbers are expected for this pin;
the implementation must not rewrite either value or accept a prefix/range. The inner
member hashes were derived from the official archive whose outer SHA-256 matched the
canonical release hash. Product verification uses the inner wallet-RPC length and hash.
The `monerod` values are only provenance for the offline local integration gate; BitBook
does not select, verify, launch, or own the user's `monerod`.

The only new direct Rust dependency permitted by this ticket is:

```toml
md-5 = { version = "=0.11.0-pre.4", default-features = false, features = ["zeroize"] }
```

It exists solely for legacy interoperability with Monero's HTTP Digest authentication.
MD5 is not approved for password storage, signatures, integrity, key derivation, general
hashing, or any new protocol. Credentials are uniformly random and short-lived; the
security boundary remains loopback plus process ownership, with same-user malware an
explicit residual risk. No general HTTP client, async runtime, TLS stack, URL parser,
proxy library, Monero Rust wallet, C/C++ FFI, git dependency, wildcard requirement,
`[patch]`, build-time downloader, vendored binary, or new first-party `unsafe` is allowed.

The exact prerelease pin is intentional. The accepted Zcash dependency graph already
requires `digest = "=0.11.0-pre.9"`; Cargo cannot resolve that exact prerelease beside
the final `md-5 0.11.0` crate's stable `digest ^0.11` requirement. Official RustCrypto
`md-5 0.11.0-pre.4` requires exactly `digest 0.11.0-pre.9`, retains the
`digest/zeroize` feature forwarding, is `build = false`, and therefore adds no second
Digest line or build script. Upstream source metadata:
<https://docs.rs/crate/md-5/0.11.0-pre.4/source/Cargo.toml>.

## Closed installation-selection contract

The broker-native surface adds one in-process action, `SelectXmrInstallation`; it invokes
a native file picker for `monero-wallet-rpc`. Electron, preload, renderer, `bb-go`, HTTP,
and a generic broker method cannot supply a path. `NativeAction::from_method` remains
fail-closed.

The selected path must be absolute UTF-8, at most 4096 bytes, and point directly to the
wallet-RPC executable. Enrollment and every launch perform, in order:

1. `lstat` without following the final component;
2. regular-file and non-symlink validation;
3. exact byte-length validation;
4. executable validation for the effective broker user;
5. bounded streaming SHA-256 and constant-time equality with the pin; and
6. only after the hash succeeds, an exact bounded `--version` probe.

The broker stores an atomic `0600` selection record beneath its existing `0700` data
directory. It contains schema version, platform pin ID, and the selected path. The path
is never compiled into source and never appears in logs, diagnostics, evidence,
snapshots, Electron IPC, process argv, or environment. Corrupt, unknown-field,
unsupported-version, symlinked, overlong, or partially written records fail closed.

The exact selected path is re-used; there is no parent-directory scan and no PATH lookup.
Missing or unmounted portable media is `UNAVAILABLE`. A running child is stopped if the
selected file disappears or its observed file identity, size, or modification metadata
changes. The next launch repeats the full hash. Same-user replacement between a
successful hash and kernel execution is an acknowledged local-malware residual and is
not described as solved.

## Local-node gate

Product endpoints are constructed internally and are never parsed from caller input:

| Network | Node endpoint | Product state in this ticket |
| --- | --- | --- |
| `xmr-stagenet` | `http://127.0.0.1:38081` | enabled |
| `xmr-testnet` | `http://127.0.0.1:28081` | enabled |
| `xmr-mainnet` | `http://127.0.0.1:18081` | `NETWORK_DISABLED` before side effects |

The broker probes only `get_info` and `hard_fork_info` on the selected network's fixed
loopback node RPC. It never calls `set_daemon`, accepts a host or URL, resolves DNS,
connects through a proxy, discovers public nodes, or retries another endpoint. An
authenticated user node is out of scope for v1 and is reported as `NODE_UNAVAILABLE`;
the broker never asks Electron for node credentials.

`get_info` is accepted only when status is `OK`, `nettype` and the three network booleans
are mutually consistent with the requested network, `offline=false`, `untrusted=false`,
and `bootstrap_daemon_address` is empty. A nonempty bootstrap address or an untrusted
response is `NODE_UNAVAILABLE`, not syncing. `was_bootstrap_ever_used=true` by itself is
historical and is not a failure after bootstrap is disabled. A valid local node with
`synchronized=false` is `NODE_SYNCING`; it is not replaced with a remote node.

The wallet process may start against a valid syncing node so node and wallet progress
remain independent. Wrong network, malformed/oversized response, inconsistent heights,
HTTP authentication, connection failure, or any non-loopback redirect fails closed.

## Wallet-RPC process contract

The Rust broker owns one child per active XMR account, bounded to four. It never adopts a
pre-existing wallet-RPC. Each child has its own runtime directory, wallet directory,
random port, random login, and lifecycle state. Account IDs and network values are
validated before any directory, socket, or process side effect.

The runtime and wallet directories are `0700`; secret/config files are `0600`. The broker
chooses a port from 49152–65535 using OS entropy with at most 16 collision attempts. Port
selection is not caller-controlled. It creates a private relative
`wallet-rpc.conf`, clears the child environment, sets only the reviewed locale value,
sets the child current directory to the private runtime directory, and executes exactly:

```text
<verified selected executable> --config-file=wallet-rpc.conf
```

The OS program path is the selected executable, but Unix `argv[0]` is explicitly fixed
to `monero-wallet-rpc`; the personal selected path is not copied into the argv vector.

The config contains exactly the applicable network flag and reviewed values equivalent
to:

```text
rpc-bind-ip=127.0.0.1
rpc-bind-port=<broker random port>
rpc-login=<broker random username>:<broker random password>
rpc-ssl=disabled
daemon-address=http://127.0.0.1:<fixed network port>
daemon-ssl=disabled
untrusted-daemon=1
wallet-dir=<broker account wallet directory>
shared-ringdb-dir=<broker account private ring directory>
no-dns=1
non-interactive=1
log-file=<broker private bounded log file>
log-level=0
max-log-file-size=1048576
max-log-files=1
rpc-max-connections=4
rpc-max-connections-per-private-ip=4
rpc-max-connections-per-public-ip=1
rpc-response-soft-limit=65536
stagenet=1                 # stagenet only
testnet=1                  # testnet only
```

Exactly one of the final two network lines is present. Forbidden config/argv includes
`disable-rpc-login`, `restricted-rpc`, `confirm-external-bind`, IPv6 RPC, CORS origins,
`trusted-daemon`, `proxy`, `tx-notify`, `detach`, `pidfile`, `wallet-file`, wallet
passwords, hardware flags, mainnet flags, remote addresses, and extra options. Full
wallet RPC is intentional; watch-only authority is enforced by broker account state, not
the `--restricted-rpc` process flag.

Startup must authenticate and receive the exact pinned wallet-RPC version within 10
seconds. The config remains private while needed and is deleted during normal or failed
teardown. Stop order is closed: stop new calls, close the wallet where possible, request
`stop_wallet`, wait at most two seconds, kill the exact child/process group if needed,
wait/reap, wipe RPC/wallet credentials, close sockets, and remove runtime secrets. Lock,
broker exit, executable disappearance/change, auth failure, malformed RPC, or unexpected
child exit performs that teardown. A failure in one XMR child never terminates ZEC or
the social application.

## Closed loopback transport and authentication

Production uses a bounded synchronous `std::net::TcpStream` transport to numeric
`127.0.0.1` only. It has no URL parsing, DNS, proxy, redirect, TLS, connection pool, or
generic request method. Connect timeout is two seconds; read and write timeout is five
seconds. Request bodies are at most 16 KiB and response headers plus body are at most 64
KiB. HTTP/1.1 responses require one valid `Content-Length`, `Connection: close`, and no
`Transfer-Encoding`; duplicate/conflicting length, folding, control bytes, trailing
bytes, 3xx, or an unrecognized status is failure.

Wallet RPC requires a two-request Digest exchange. The challenge parser accepts exactly
the `Digest` scheme, bounded quoted `realm` and `nonce`, `qop=auth`, absent or `MD5`
algorithm, and optional bounded `opaque`. Duplicate keys, unknown algorithm, `auth-int`,
missing qop, bad quoting, stale loops, or more than one challenge retry fails. `cnonce` is
16 fresh random bytes encoded as lowercase hex and nonce-count begins at `00000001`.
The RFC Digest calculation is fixed to method `POST` and URI `/json_rpc`. HA1, password,
cnonce, challenge state, and authorization bytes are wipe-on-success/error/unwind and
redacted from formatting.

Node RPC uses the same bounded HTTP/JSON framing without an Authorization header.
Wallet-RPC requests require Digest on every connection. JSON is UTF-8 with no BOM,
maximum nesting 16, no trailing bytes, exact JSON-RPC `2.0`, and an exact request ID.
Closed typed response structs reject type confusion, overflow, duplicate JSON keys, and
unsupported upstream response shape. Raw upstream error text is never public or logged.

## Closed RPC authority

The adapter has no raw method-string API. The only wallet-RPC operations are phase-bound
typed calls for:

```text
get_version
create_wallet
restore_deterministic_wallet
generate_from_keys
open_wallet
close_wallet
stop_wallet
query_key                 # fresh software creation only; mnemonic only
refresh
get_height
get_balance
get_address
create_address
validate_address
```

The only node operations are `get_info` and `hard_fork_info`. A fake server fails the
test immediately on every other method. In particular, there is no transfer,
transfer-split, sweep, describe, sign, submit, relay, proof, key-image export/import,
multisig, address-book, mining, daemon-switch, raw-RPC, or generic JSON entry point.
`query_key` cannot request a spend key or view key and cannot run after creation.

## Account custody and persistence

WAL-007 supports `software` and `watch_only`. The `hardware_backed` account kind remains
unsupported until BBD-WAL-008, and every device-specific capability stays false. Account
creation/import is a typed broker-native operation; this ticket does not add an Electron
form.

Every wallet has a derived private directory and filename from its validated 32-lowercase
hex account ID; no caller path becomes a wallet filename. The directory and state DB are
network-bound. Mainnet, cross-account, cross-network, symlink, non-regular file,
unexpected owner/mode, partial wallet set, or substitution is rejected before open.

The WAL-004 encrypted vault plaintext for XMR is a closed big-endian binary
`XmrSecretV1`, maximum 2048 bytes:

```text
magic                 8 bytes: ASCII "BBXMR001"
kind                  u8: 1 software, 2 watch-only
restore_height        u64
wallet_password_len   u16: exactly 64
wallet_password       64 lowercase hex bytes from 32 OS-random bytes
primary_address_len   u16: exactly 95
primary_address       95 ASCII bytes
secret_len            u16
secret                software: bounded 25-word English mnemonic
                      watch-only: exactly 64 lowercase-hex private view key
```

No trailing or unknown bytes are accepted. The existing vault envelope supplies account,
asset, network, epoch, authentication, and rollback binding. The XMR secret never has a
`Debug` representation, never crosses Electron, and is wiped after use.

Software creation calls `create_wallet` with a fresh wallet password and `language` set
to English, then calls mnemonic-only `query_key`, verifies the returned primary address,
and seals `XmrSecretV1` before reporting success. Restore height is the accepted local
`height_without_bootstrap` minus a saturating 100-block safety margin. If vault sealing
or durable account-state creation fails, the child closes, generated wallet files are
removed or quarantined, and no account is returned. A rollback cleanup failure is a
compound `INTERNAL` failure and the account remains unavailable.

Watch-only import accepts only a network-valid primary address, private view key, and
restore height no greater than the accepted local height. It uses `generate_from_keys`,
verifies that RPC reports watch-only and the same primary address, seals kind 2, and
never sets spend capability. Import values are accepted only on the broker-native
surface and are wiped on every exit.

On unlock/open, missing wallet files are recovered from the authenticated XMR vault by
`restore_deterministic_wallet` or `generate_from_keys`; existing files use `open_wallet`.
After either path, primary address, account kind, network, and restore height must match
the sealed record. Software account lock closes and stops its child so spend material is
not retained. A watch-only process may remain only after successful import/open, but a
cold restart still requires authenticated vault access to recover its wallet password.

The account state DB is a broker-owned SQLite file with `0600` mode and `FULL` sync. Its
closed schema binds schema version, account ID, network, primary address, greatest
issuance sequence, and receiver rows:

The schema version is exactly `1`.

```text
request_id, account_index, subaddress_index, subaddress, issued_at_sequence
```

`request_id`, `(account_index, subaddress_index)`, `subaddress`, and
`issued_at_sequence` are independently unique. Account index is exactly zero;
subaddress index is positive u32; sequence is positive and at most signed-64 maximum.
Schema drift, duplicate binding, rollback, corruption, failed sync, or wrong identity is
`STATE_CORRUPT`, not reconstruction from wallet output.

## Viewing and fresh-receiver behavior

Accepted node/wallet state is represented independently:

```text
node_state:   NODE_UNAVAILABLE | NODE_SYNCING | READY
wallet_state: UNAVAILABLE | LOCKED | WALLET_REFRESHING | READY
device_state: NOT_APPLICABLE
```

Node height comes only from the accepted local-node probe. Wallet height comes from the
authenticated child. A wallet behind the node is `WALLET_REFRESHING`; a node still
syncing remains `NODE_SYNCING` even if the wallet is caught up. Balance and unlocked
balance are checked u64 atomic units encoded as canonical decimal strings, with unlocked
not greater than total. Missing, stale, overflowed, negative, floating, or inconsistent
values fail closed; total is never substituted for unlocked.

The internal sanitized account view contains account ID, asset XMR, network, kind,
privacy `private`, the three states, canonical heights/balances, and the existing closed
capability object. It contains no primary address, receiver history, seed/view key,
wallet password, RPC login, endpoint, port, filesystem path, process ID, raw node value,
or upstream diagnostic. XMR sets every ZEC-only capability false/null and, in this
ticket, all prepare/sign/broadcast capabilities false.

`FreshXmrReceiverV1` input is exactly:

```text
account_id              32 lowercase hex
network                 xmr-stagenet | xmr-testnet
request_id              32 lowercase hex
```

An exact existing `(account_id, network, request_id)` returns the durably stored receiver
without an RPC call. Otherwise, only a ready eligible account may call `create_address`
with account index zero and an empty label. The adapter verifies positive returned index,
network, subaddress classification through `validate_address`, and address equality
through `get_address`, then commits the request binding and sequence before returning:

```text
account_id, network, request_id, receiver, account_index,
subaddress_index, issued_at_sequence
```

The primary address is never returned. Concurrent calls are serialized per account and
distinct request IDs receive distinct increasing indices and sequences. A crash or DB
failure after wallet-RPC creation but before the binding may consume and permanently
skip an index, but that unbound address is never returned or reused. Reopen preserves all
returned bindings. Exhaustion, lock, watch-only initialization failure, wrong network,
RPC mismatch, or persistence failure has no primary-address or stale-address fallback.

## Error and log contract

Use existing stable broker meanings:

```text
SCHEMA, UNAUTH, NETWORK_DISABLED, WRONG_NETWORK, UNAVAILABLE,
NODE_UNAVAILABLE, NODE_SYNCING, LOCKED, WATCH_ONLY,
PROTOCOL_INCOMPATIBLE, STATE_CORRUPT, LIMIT, INTERNAL
```

Syncing is normally snapshot state, not success disguised as an error. A missing portable
drive or child is `UNAVAILABLE`; a selected executable with wrong bytes/version is
`PROTOCOL_INCOMPATIBLE`; bootstrap/remote/wrong node is `NODE_UNAVAILABLE`.

Public diagnostics reveal operation, account ID, asset/network, and stable code only.
Logs and errors never contain selected/user paths, process argv/config, endpoint/port,
RPC realm/nonce/login/password/Authorization, wallet password, mnemonic, view/spend key,
primary address, fresh receiver, request ID, raw JSON/HTTP, node response, wallet files,
SQLite rows, or upstream error text. Tests use distinct canaries for every prohibited
class and require redacted `Debug`, display, error chains, panic, and teardown output.

## Required test groups

1. **Release and installation:** exact outer/inner provenance constants, normal and
   portable selection, native-only origin, atomic record, lstat/non-symlink, exact size,
   hash before execution, exact post-hash version, missing/remounted/changed file, no
   PATH/download/fallback, and no path disclosure.
2. **Process plan and lifecycle:** exact argv/config/environment, full wallet-RPC without
   `restricted-rpc`, authenticated IPv4 loopback only, random collision-bounded port,
   directory/file modes, readiness timeout, per-account/four-child cap, close/stop/kill/
   reap order, executable removal, broker exit, and independent XMR/ZEC failure.
3. **Digest and framing:** official RFC vectors plus recorded Monero challenge shape,
   random cnonce/nc, challenge/retry/timeout bounds, malformed/duplicate challenge keys,
   bad qop/algorithm/stale loops, body/header/JSON boundaries immediately below/at/above
   each limit, exact IDs, no redirect/chunking/DNS/proxy, and wipe canaries.
4. **Local node policy:** stagenet/testnet ports, mainnet pre-side-effect rejection,
   network-boolean matrix, syncing versus unavailable, bootstrap address, untrusted
   result, past-bootstrap-only acceptance, auth/redirect/malformed/oversized failure,
   and proof that no alternate endpoint is attempted.
5. **Account lifecycle and recovery:** fake RPC call order for software create, mnemonic
   capture/seal/wipe, watch-only import, open, missing-file recovery, primary/network/kind
   substitution, restore-height margin/bounds, wallet-file permissions, lock teardown,
   failed vault/state writes, cleanup failure, and no hardware capability expansion.
6. **Viewing and subaddresses:** exact u64 balances, node/wallet state independence,
   capability negatives, replay without RPC, non-primary subaddress classification,
   durable request/index/sequence binding, concurrency, reopen, exhaustion, consumed gap
   after failure, SQLite corruption/rollback/sync failure, and no fallback.
7. **Negative authority and hygiene:** phase-bound typed allowlist, fake server failure on
   every unlisted method, no raw/generic/transfer/sign/submit route, no mainnet side
   effect, secret/path/RPC canaries across success/error/unwind, child/process-group
   cleanup, credential zeroization, bounded logs, and production inventory policy.
8. **Regression and isolation:** every accepted WAL-002/003/004/005/006 and RATE-001
   Rust/Node/security test stays green; no Electron, supervisor, preload, renderer,
   `bb-go`, quote worker, workflow, package, release, or SBOM behavior changes.
9. **Real offline local gate:** after fake acceptance, use only the exact reviewed
   archive members to run a test-owned stagenet `monerod --offline` on random loopback
   ports and the real verified full wallet-RPC. Prove Digest auth, process split, network
   match, software-wallet creation, fresh non-primary subaddress, close/stop, and no
   outbound connection. The gate creates only ephemeral stagenet material, emits no
   seed/password/path, and deletes its exact scratch directory after evidence hashes.
   The scratch directory is derived only as
   `PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/wal007-local-gate")`;
   it is not accepted from an environment variable or caller.

Tests assert behavior, bytes, persisted state, call ordering, child state, and negative
capability. Source-text assertions are limited to dependency, production-inventory, and
forbidden-capability policy; they cannot substitute for runtime tests.

## Test-first phases and authorization boundary

### Phase A — test source (future; not authorized by this ticket)

After an explicit reviewer handoff, Sol may edit only:

- `wallet-broker/Cargo.toml`
- `wallet-broker/tests/native_surface.rs`
- `wallet-broker/tests/xmr_distribution.rs`
- `wallet-broker/tests/xmr_process.rs`
- `wallet-broker/tests/xmr_rpc.rs`
- `wallet-broker/tests/xmr_account.rs`
- `wallet-broker/tests/xmr_receiver.rs`
- `wallet-broker/tests/xmr_hygiene.rs`
- `wallet-broker/tests/xmr_local_gate.rs`
- `test/securityPolicy.node.js`

The manifest may add only the exact `md-5` declaration, explicit test targets, and an
empty `xmr-local-gate` feature required by the real-gate test target. That feature must
not alter production compilation or behavior. The local-gate source may launch only
reviewed binaries with fixed offline/loopback/non-mainnet flags and is not an ordinary
test dependency.

Sol does not edit `Cargo.lock`, production Rust/Node/Electron, existing non-native Rust
tests, policy implementation, fixtures, evidence, documentation, workflows, packages,
or unlisted paths. Sol does not execute tests, dependency resolution, binaries, network,
Git, or GitHub and does not create stubs, ignored tests, conditional passes, scratch
wallets, or evidence.

### Phase B — expected red (future; separate Hermes handoff)

After XHigh accepts exact Phase-A hashes, Hermes may resolve only the reviewed lock
change, inspect the dependency/source/license/build-script delta, and run the focused
fake tests. The expected red must be an absent `bitbook_wallet_broker::xmr` production
behavior or the frozen native selection behavior. It must not be a missing dependency,
bad test, missing personal installation, attempted external network, timeout, or an
unrelated regression. The real local gate does not run during expected red.

### Phase C — production source (future; separate handoff after accepted red)

The maximum production inventory is:

- `wallet-broker/src/lib.rs` — expose `pub mod xmr;` only
- `wallet-broker/src/native.rs` — add the closed native selection action/port only
- `wallet-broker/src/native_ui.rs` — implement that one native file selection only
- `wallet-broker/src/xmr.rs`
- `wallet-broker/src/xmr/account.rs`
- `wallet-broker/src/xmr/distribution.rs`
- `wallet-broker/src/xmr/model.rs`
- `wallet-broker/src/xmr/process.rs`
- `wallet-broker/src/xmr/receiver.rs`
- `wallet-broker/src/xmr/rpc.rs`
- `wallet-broker/src/xmr/store.rs`
- `wallet-broker/src/xmr/test_support.rs`

The source actor receives one bounded slice at a time: distribution/native selection,
process plan/lifecycle, RPC/node transport, account custody/recovery, then viewing/
receiver persistence. Each slice requires reviewed source and a Hermes focused-green
gate before the next. Production may not edit protocol/supervisor/Electron/quote/ZEC/
vault/session/store behavior or widen the listed inventory.

### Phase D — real offline gate and integration (future; Hermes only)

After all fake/source gates pass, Hermes runs the named `xmr-local-gate` with an
ephemeral, reviewer-verified official Monero root supplied outside source. It records
release/hash identities and normalized behavior only, never the personal root, wallet
path, port, password, seed, address, or raw output. No live mainnet/stagenet peer network,
funds, existing user wallet, or currently running user node is touched.

Before that run, Hermes verifies that `wallet-broker/target` is on a disk-backed
filesystem, not tmpfs. The gate refuses to start if the exact derived scratch leaf
already exists, if its parent is not the manifest directory's real `target`, or if any
validated component is a symlink. It creates that leaf itself. Cleanup may recursively
remove only that exact validated leaf after every child is reaped; it must never clean a
caller/env-selected path or a broader `target` directory.

This document authorizes none of the phases. `docs/handoff/CURRENT_TASK.md` or a later
reviewer handoff must explicitly open exact paths and stop conditions.

## Planned commands and acceptance

Sol runs none of these. Hermes handoffs will freeze exact focused red/green commands.
The eventual full gate, from repository root with Rust 1.98.0 and the existing
disk-backed target, includes:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features
/home/lars/.cargo/bin/rustup run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --all-targets --all-features -- -D warnings
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --features native-ui --test native_surface
BITBOOK_MONERO_TEST_ROOT=<reviewer-verified-root> /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --features xmr-local-gate --test xmr_local_gate
npm run build
npm test
node scripts/security-policy.js
npm audit --audit-level=low
/home/lars/.cargo/bin/cargo-audit audit --file wallet-broker/Cargo.lock
/home/lars/.cargo/bin/cargo-deny --manifest-path wallet-broker/Cargo.toml --all-features check advisories bans licenses sources
target/security-tools/gitleaks-v8.30.1/gitleaks git --redact=100 --no-banner .
target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner .
```

The real-gate command is separately scheduled and is not part of ordinary offline CI.
Its test harness must prove `monerod` received fixed `--offline`, loopback bind, random
test ports, stagenet, private data/log paths, and no bootstrap/proxy options.

At least seven isolated falsifications are required with exact restoration:

1. accept a one-byte-changed selected executable and prove the pin test fails;
2. add `restricted-rpc` or remove RPC login and prove the launch-plan test fails;
3. accept a remote/bootstrap node response and prove the local-node test fails;
4. accept malformed/replayed Digest authorization and prove the auth test fails;
5. invoke one spend/raw RPC method and prove the fake server and policy test fail;
6. return/reuse a primary or uncommitted address and prove the receiver test fails; and
7. retain a child or credential after lock/removal and prove teardown/hygiene fails.

Lock/source/license/build-script review, RustSec, deny, scanner, full-history findings,
and GitHub checks are blocking. A new finding, unexpected crate, network capability,
path/secret disclosure, unreviewed binary, flaky lifecycle, or incomplete falsification
blocks acceptance. No platform package, release artifact, Electron launch, SBOM dispatch,
mainnet node, funds, hardware device, or existing user wallet is required or allowed.

## Acceptance boundary

BBD-WAL-007 is complete only after accepted tests, expected red, bounded production
slices, fake greens, real offline local gate, falsifications, full/security regression,
exact Git state, and applicable GitHub checks. Completion proves only verified
broker-managed XMR wallet-RPC lifecycle, local-node enforcement, non-mainnet software/
watch-only viewing and recovery, and durably request-bound fresh subaddresses.

BBD-WAL-008 remains hardware capability/device trust. BBD-WAL-009 owns all XMR
transaction preparation/signing/verification/submission plus cross-coin concurrency,
cancellation, and crash recovery. BBD-WAL-010 owns visible onboarding/Pay composition,
BBD-WAL-011 owns packaged native components/SBOM release evidence, and BBD-WAL-012 owns
mainnet authorization.
