# Codex Sol Handoff — BBD-WAL-007 MD5 Pin Correction 01

Status: AUTHORIZED — TWO-LINE TEST/MANIFEST CORRECTION ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Hermes stop evidence (uncommitted):
`../testing/BBD-WAL-007-EXPECTED-RED-01.md`

## Cause and objective

Hermes correctly proved that final `md-5 0.11.0` cannot resolve beside the Zcash graph's
exact `digest 0.11.0-pre.9`. Correct only the frozen MD5 pin to official RustCrypto
`md-5 0.11.0-pre.4`, whose normalized upstream manifest requires exactly that existing
Digest version and retains `zeroize = ["digest/zeroize"]`.

## Authorized paths and exact edit

Edit only:

- `wallet-broker/Cargo.toml`
- `test/securityPolicy.node.js`

In each file, replace the one exact accepted declaration:

```text
md-5 = { version = "=0.11.0", default-features = false, features = ["zeroize"] }
```

with:

```text
md-5 = { version = "=0.11.0-pre.4", default-features = false, features = ["zeroize"] }
```

Make no other insertion, deletion, reflow, rename, assertion change, or test-count
change. Every other accepted Phase-A byte stays frozen.

## Prohibited actions

Do not edit `Cargo.lock`, the Hermes stop record, Rust tests, production, governance,
fixtures, packages, workflows, or any other path. Do not run Cargo, Rust, Node, npm,
formatters, tests, builds, dependency resolution, network, Git, or GitHub. Do not stage,
commit, or push.

## Delivery

Stop after the two replacements. Report both line counts and SHA-256 hashes, confirm the
old declaration is absent and the new declaration occurs exactly once in each file, and
confirm no prohibited command or extra edit occurred.
