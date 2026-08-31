# BBD-WAL-004 Lock-Graph Review

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Reviewed baseline: `fe2fe7e78fab0012a5fa77f128716bb7262aba58`

Review time: 2026-08-30T18:59:43-07:00 (PDT)

Result: **ACCEPTED FOR SOL PRODUCTION WITH THE FIXED MIT MANIFEST CORRECTION**

## Resolved graph

The committed `wallet-broker/Cargo.lock` is 3,273 lines with SHA-256
`1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5`.
Cargo locked 326 external packages; the lockfile has 327 package records including the
workspace crate. Every external source is the crates.io registry and every registry
record has a checksum. There is no git dependency, outside path dependency, wildcard,
vendored binary, wallet/coin crate, keyring, OpenSSL, or direct HTTP/general async
network runtime.

The Linux `--no-default-features` graph has 38 unique package/version pairs including
the broker. The Linux `--all-features` graph has 172. The direct exact pins and selected
features match the accepted manifest, including `secrecy = 0.10.3` with defaults off and
no optional features. The optional native graph selects only:

- eframe: `default_fonts`, `glow`, `wayland`, and `x11`;
- rfd: `xdg-portal` and `wayland`.

`wgpu`, `egui-wgpu`, `webbrowser`, `ron`, `puffin`, `accesskit_winit`, `reqwest`,
`hyper`, `tokio`, `async-std`, `zbus`, and `ashpd` are absent from the active Linux
graph. The `accesskit` data-types crate is internal to egui, but its winit exposure is
not selected. Eframe does select egui-winit clipboard infrastructure. Production must
therefore use egui's password/non-copyable edit behavior and wipe its buffer; clipboard
infrastructure gains no broker authority. Rfd uses local dynamic `libdbus` portal calls
and may fall back to the local `zenity` OS dialog. It receives path selection only and
must never receive backup bytes, plaintext, or Electron data.

## Build scripts and duplicates

Thirty-two build scripts are active in the Linux all-features graph. Inspection found
only Rust compiler/capability probes, Cargo configuration generation, system-library
`pkg-config` probes for X11/Wayland/xkbcommon, generated windowing bindings, and a small
Wayland logging C shim compiled by `cc`. No active build script contains a downloader,
HTTP client, socket client, `curl`, or `wget` invocation.

The ten duplicate package names are `calloop`, `calloop-wayland-source`, `getrandom`,
`linux-raw-sys`, `miniz_oxide`, `rustix`, `smithay-client-toolkit`, `syn`, `thiserror`,
and `thiserror-impl`. They arise from the optional UI/platform graph. No AEAD, KDF,
hash, MAC, cipher, secret-wrapper, or zeroization primitive is duplicated. `getrandom`
0.4.3 is the broker's exact direct entropy dependency; 0.3.4 is used only through
eframe/egui/winit's `ahash` UI graph.

## Licenses and advisories

All locked external packages declare permissive or selectable-permissive licenses. The
font bundle additionally requires OFL-1.1 and Ubuntu-font-1.0 notices. The only missing
license declaration is the new workspace crate itself. The repository is MIT licensed,
so production must add exactly `license = "MIT"` to `wallet-broker/Cargo.toml` and add a
fail-closed root `deny.toml` allowing only the reviewed permissive SPDX set, denying
unknown registries/git sources and wildcard dependencies, and retaining duplicate-version
visibility. No dependency or lockfile change is authorized to make the policy pass.

Official user-level `cargo-audit 0.22.2` was installed with Rust/Cargo 1.98.0, with its
build state on disk under `/home/lars/OpenBazaar/bb-desktop/target`. The exact locked
audit loaded 1,226 RustSec advisories, scanned all 327 lockfile records, exited 0, and
reported no vulnerabilities.

## Reviewer boundary

Review commands changed no repository path. `HEAD == origin/master` remained
`fe2fe7e78fab0012a5fa77f128716bb7262aba58`; only ignored `dist/`, `node_modules/`, and
`target/` are present. Production is authorized only under
`docs/handoff/CODEX_SOL_BBD_WAL_004_PRODUCTION.md`. The accepted tests, fixture, and
lockfile remain immutable during the Sol source drop.
