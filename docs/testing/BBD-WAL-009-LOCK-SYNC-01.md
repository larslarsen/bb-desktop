# BBD-WAL-009 Lock Synchronization 01

Integration note (2026-09-09): local repository and home prefixes are rendered as <repo> and <home>. Earlier hashes/counts describe execution-time files. This is an archival evidence record; acceptance limits and reported deviations remain governed by the corresponding reviewer handoffs. No test was repeated for this source checkpoint.


Actor: Hermes Jr Dev. Repository: `<repo>`.
Governance parent: commit `7b11bc61879e1097aaf8cf1dfe453ee716098470`.

## Runtime identity

Per `docs/engineering/HERMES_JR_DEV_ROUTING.md`, the Jr Dev role is bound to the locally
installed Hermes Agent. This identity describes the ORIGINAL lock-sync run, recovered
read-only from session metadata and transcript of that run (session ID
`20260907_204336_5c6109`), not this correction run or the adoption-time routing policy:

- Agent: Hermes Agent v0.18.2 (2026.7.7.2) · upstream 2237be35 · local 10b6d1a9 (+1 carried commit)
- Provider: `nous`
- Model: `poolside/laguna-s-2.1:free`

Note: the handoff referenced the original run as session 47880, which was not recoverable
in the session DB by that identifier; the actual original lock-sync execution was located
under session `20260907_204336_5c6109`. This correction used read-only identity checks and
did not repeat `cargo update`.

## Authorized command (exact, terminal)

```text
<home>/.cargo/bin/rustup run 1.98.0 cargo update --manifest-path wallet-broker/Cargo.toml --offline --workspace
```

## Actual command and exit

Executed once, verbatim. Exit code: 0.

Command output:

```
Locking 0 packages to latest Rust 1.98.0 compatible versions
note: pass `--verbose` to see 9 unchanged dependencies behind latest
```

`--offline` was honored (no network access attempted or required).

## Lockfile hashes (SHA-256)

- Starting `wallet-broker/Cargo.lock`:
  `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420`
- Resulting `wallet-broker/Cargo.lock`:
  `b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71`

## Protected manifest (Cargo.toml)

- 122 lines, SHA-256 before:
  `73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503`
- SHA-256 after:
  `73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503`

Unchanged.

## Exact lockfile diff

```diff
diff --git a/wallet-broker/Cargo.lock b/wallet-broker/Cargo.lock
index 9dc2c287..0959f224 100644
--- a/wallet-broker/Cargo.lock
+++ b/wallet-broker/Cargo.lock
@@ -271,6 +271,7 @@ dependencies = [
  "getrandom 0.4.3",
  "hkdf",
  "md-5",
+ "orchard",
  "pczt",
  "rand_core",
  "rfd",
```

One insertion only. The sole addition is `"orchard"` to the `bitbook-wallet-broker` package's
root dependency list.

## Verification of authorized-outcome constraints

- Exit code: 0.
- Diff scope: 1 file changed, 1 insertion(+), 0 deletions.
- The only added line is `"orchard"` in the root dependency list.
- No package/version/checksum/source change.
- No new resolved package added (`orchard` and all its transitive packages were already
  present and resolved in the starting lockfile; only the `bitbook-wallet-broker` package
  entry now references it as a direct dependency).
- No manifest edit beyond the Grok-accepted `orchard` declaration (Cargo.toml hash unchanged
  before/after).
- No network access (`--offline` honored; no network error).
- No test/build executed (only `cargo update`).
- No policy run.

## Protected-file preservation (SHA-256 before == after)

The expected existing dirty files were preserved exactly:

| File | SHA-256 (before == after) |
|---|---|
| package.json | 84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780 |
| package-lock.json | 5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc |
| wallet-broker/Cargo.toml | 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503 |
| wallet-broker/src/native.rs | 992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc |
| wallet-broker/src/native_ui.rs | 04882d21ca5e4ae21f61aa101b1f9a9610baf7eb1a736effcbb44f2b103e0735 |
| wallet-broker/src/zec.rs | 045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b |
| wallet-broker/src/zec/prepare.rs | 44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07 |
| wallet-broker/src/zec/store.rs | 531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90 |
| wallet-broker/src/zec/test_support.rs | 21489e5cda159d670fbdd3b96b196bb0e40f8227fa8770a80b6bd7a3e1c23d83 |
| wallet-broker/src/zec/spend.rs (untracked) | ae665e4742925d88e1f7cf6a66172e89a268f613a5fd0efc0b3cc3581ac4cdf4 |

## Preflight state

- Repository HEAD: `7b11bc61879e1097aaf8cf1dfe453ee716098470`
- Staged changes: none (`git diff --cached --name-only` empty).
- Dirty files matched the expected set in the handoff.

## Status of pending gates

Source, policy, cryptographic, and native-UI gates remain pending. This execution is
lock synchronization only: lockfile and evidence are left uncommitted for reviewer
inspection. No CURRENT_TASK update, staging, committing, pushing, or further actor
invocation occurred.
