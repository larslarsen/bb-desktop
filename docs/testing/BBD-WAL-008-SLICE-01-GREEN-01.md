# BBD-WAL-008 Slice-01 Green 01 — COMPLETE

State: COMPLETE

Integration actor: Jr Dev — Hermes

## Environment identity

Hermes Agent v0.18.2 (2026.7.7.2) · upstream 63279301 · local 10b6d1a9
Provider: nous
Model: meituan/longcat-2.0:free
Node.js v22.23.1
Branch: master
HEAD: a4ae7c51ad0664564ae36abc778862d10cfcdff3
origin/master: a4ae7c51ad0664564ae36abc778862d10cfcdff3

## Filesystem fact

`wallet-broker/target` resides on `/dev/mapper/ubuntu--vg-ubuntu--lv` (ext4, disk-backed).

## Accepted source identity (Source Review 01 / Format Correction 01)

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 253 | `6dd71f9f70d7b5b8aaddd2e3d4df2b9b2b232b45182ca1db4d32078146751fa2` |
| `wallet-broker/src/zec/hardware.rs` | 864 | `48233ec9ceea26e4b8ac499c00ee5d8c00ca546f2e036dfd16d1deb59d565285` |
| `wallet-broker/src/zec/test_support.rs` | 2,385 | `7d73c8ce55073198a345a71eadf2ac47589665dc1580dc8f485ab3d0f38a5a55` |

All three frozen hashes, line counts, and the unchanged lockfile are re-verified at integration. Only these three paths are modified.

## Falsification reference (Resume 02 — already accepted)

The Resume-02 AND-to-OR falsification and its exact restoration are reviewer-accepted and need not be repeated here. The sole selected test failed because `CanView` expanded from live input, proving the intersection is non-vacuous. Source was immediately restored to the frozen AND identity.

## Fresh Resume 03 gate results

| # | Command | Result | Exit |
| --- | --- | --- | ---: |
| 1 | `cargo fmt --check` | No diff | 0 ✓ |
| 2 | `cargo test --test zec_hardware -- --skip persisted_... --skip write_file_... --skip invalid_records_... --skip verified_fields_... --skip success_error_...` | 13 passed; 0 failed; 5 filtered out | 0 ✓ |

Both commands were submitted byte-for-byte, alone, once, with no wrapper, redirection, pipeline, or repeat. Each exited 0 exactly as required.

## No-mutation proof

No source bytes were modified beyond the already-accepted Sol source drop and Spark mechanical formatter edits. The three frozen hashes and unchanged lockfile are re-confirmed at integration time.

## Prohibited-action confirmation

No persistence slice, WAL-007, real-device, transport, signing, extraction, broadcast, wallet, node, network, Electron, npm, browser, scanner, broader regression, full suite, or gate-2+ operation was run.
