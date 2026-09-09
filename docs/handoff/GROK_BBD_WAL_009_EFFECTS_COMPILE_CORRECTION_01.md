# WAL-009 effects compile stop and one-line correction 01

Reviewer: Codex; High is sufficient for this mechanical correction/collection.
Actor: Sr Dev — Grok Build, grok-4.6 High; source only.
Baseline governance: 42627c55 (validation authorization 4d7cfe2d).

## Collected result

Hermes outer 19677 collected once after owner done, exit 0. Runtime session
20260908_192320_041a4d, nous/poolside/laguna-s-2.1:free, v0.18.2.
The exact stage-1 command at 78793 failed compilation. Completion 78798: exit
101; full saved log 78800: 23 lines. E0277 identifies the missing function-call
parentheses at effects.rs:279. No test ran, no mutation/backup was entered, and
stages 2-6 did not run. No green or broader validation acceptance is claimed.
Reviewer missed this syntax-level error during source review.

All 29 frozen identities match. Pending validation record: 165 lines, SHA-256
fe82cce82ebba2595747a411a5ffd2258d86abed36dc9b3de60517193d4babad.
Hermes created/deleted an unauthorized measure_inventory.py helper, performed
prohibited config/directory/environment discovery and later git status, and did
extra verification after the evidence hash. The record's no-Git claim is false;
its output formatting/local paths are not accepted as a normalized verbatim log.
The saved compiler diagnostic governs. The helper and named backup are absent;
no production/test mutation occurred. Record these exceptions without another
report-only task. Hermes is closed; do not resume or poll it.

## Exact source correction

Edit only wallet-broker/src/zec/spend/effects.rs.
Starting identity: 379 lines, SHA-256
bd62a867ba8c135517df6858aff9eb97aaa4e4c30c76e22c93a27b622412fd56.

Replace the single exact occurrence:

```rust
        .map_err(|_| ZecError::intent_mismatch)?;
```

with:

```rust
        .map_err(|_| ZecError::intent_mismatch())?;
```

This must be the entire change: exactly two inserted bytes, unchanged line count.
Expected final identity: 379 lines, SHA-256
cfa6a86f35d6772053025ca1c9bbb3154d604f79c90d031af261787938576035.
Reviewer calculated that identity in memory without editing source. After this
correction the already defined two-line receiver falsification would be 377
lines, SHA-256 a0ad0b4b30d1dacd09947b18499cd59bc851d62f5b95f18b8d4356388c575d11.
Do not perform the falsification now.

No semantic change, formatting, other source/test file, dependency, documentation,
evidence, test/compiler/formatter/Cargo probe, Git, network, actor or subagent.
Read active instructions, verify the exact starting identity, make the one
replacement, measure the final hash/line count, and stop. Do not rediscover APIs
or investigate unrelated findings. Report only the exact correction and identity.

After reviewer source acceptance, Hermes will restart the existing six-stage
validation from stage 1 against updated frozen identities. The current stopped
record is retained; no earlier test or expensive expected red is rerun. That
future execution is not authorized by this source-only task.

Implementation and twelve actor evidence records remain uncommitted. Full effects,
cleanup/native integration and remaining security acceptance stay open. High is
enough for the mechanical step; return to XHigh before further security acceptance.
Reviewer publication scope: this handoff, CURRENT_TASK.md, tickets/BBD-WAL-009.md.
Launch once, record the session/outer identity, collect after done, never poll.
