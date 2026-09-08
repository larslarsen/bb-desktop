# WAL-009 compile prerequisites 01

Actor: Grok Build, CLI model `grok-4.6`, reasoning High. Governance parent: the
commit containing this handoff. Four mechanical edits in two source files only.

Read AGENTS.md, TESTING.md, CURRENT_TASK.md's leading active section, this handoff,
and docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01-REVIEW.md. Inspect
only the named functions and existing fault enums/helper below. The relevant
pinned API types are established by the captured compiler diagnostics; no
dependency search, home-directory search, or historical handoff reload is needed.

## Exact source baseline

Only writable paths:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 883 | c646ea835eceac69f899dea111b19ae7fb3354ecc682b73b08b598138b0d5b0d |
| wallet-broker/src/zec/test_support.rs | 4364 | 6822e458cc8475155e6c2d3bbdef592e1af2a339100f29b3e4f7359a027886ce |

Verify both hashes before edits; stop if either differs. The accepted private test
module is read-only: wallet-broker/src/zec/spend/verification_context_tests.rs,
528 lines, SHA-256
`78e7fe50c3bd213f8d0067957bf1771bd42137d46b9e0e8e7adbcb50d8330dec`.
The evidence just accepted is also read-only, SHA-256
`79b6e97f5a6a4ca4903d9db7b40233ecf0f2ca931116e0d232853f1b842d1ff2`.
All other source, tests, fixtures, native UI, dependencies, manifests, lockfiles,
package changes, evidence, and governance remain frozen.

## Four exact edits

1. In spend.rs::validate_prepared, at original line 669, change only the borrowing
   expression from recipient().map(|value| value.as_slice()) to
   recipient().as_ref().map(|value| value.as_slice()). The slice must borrow the
   stored recipient through the Option, not a copied closure-local array. Keep
   the comparison to inspection.destination_receiver_bytes.as_slice(), its
   INTENT_MISMATCH return, output selection/count, memo check, and all other code.
2. In spend.rs::inspect_final_pczt, at original line 766, replace only the
   `.cloned()` following `.zkproof()` with `.clone()`. zkproof returns a borrowed
   Option<Vec<u8>>; clone that Option into the existing single owned proof result.
   Preserve ok_or_else(INTENT_MISMATCH), nonempty proof check, and ClearEffects
   construction. Do not use into_iter, add copies/buffers, move data out of the
   borrowed PCZT, or change its return shape. This is not secret-cleanup proof.
3. In test_support.rs::wipe_exit_for_error, replace only the invalid
   `return wipe_exit_for_fault(fault);` with a call that passes this exhaustive
   conversion to the unchanged existing helper:

```rust
        return wipe_exit_for_fault(match fault {
            PipelineFault::Signer => FaultPoint::Signer,
            PipelineFault::Prover => FaultPoint::Prover,
            PipelineFault::Finalizer => FaultPoint::Finalizer,
            PipelineFault::Extractor => FaultPoint::Extractor,
            PipelineFault::Verifier => FaultPoint::Verifier,
            PipelineFault::Cleanup => FaultPoint::Cleanup,
        });
```

   Keep wipe_exit_for_fault, both enums, the corresponding six WipeExit categories,
   and the no-fault error-code match unchanged. No wildcard, generic Success/Error
   fallback, new conversion trait/helper, or cleanup-observation change.
4. In test_support.rs::SignVerifyHarness::publish, at original line 1131, remove
   only `mut` from the PipelineOutcome parameter. Preserve its ownership, all
   accesses, revalidation, publication, and error behavior.

Only ordinary formatting of these directly edited expressions is allowed. Do
not reformat either file or alter the accepted cfg(test) module/root-accessor
suffixes. The future ShieldedVerificationContext/ShieldedVerificationAuth must
remain absent. The signature_hash call and its type error are outside this slice.
No native Context::run fix, native modal implementation, recovered-effect rewrite,
proof-verification change, or secret-lifetime redesign is authorized.

## Test-first basis and reserved validation

The four accepted signature-context tests precede source repair. Hermes's exact
expected-red attempt reproduced the three targeted errors and unused-mut warning
before any test ran. This is a compiler-compatibility slice preserving the stated
semantics, not permission to add new behavior or author new tests. It does not
turn the mixed prerequisite stop into behavioral red.

The reserved targeted red/green command is unchanged:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests
```

After only these edits, the missing context and signature_hash type error remain
expected blockers. Do not run an intermediate check or claim green. The reviewer
will inspect this exact diff, then authorize a separate context-production slice
and later Hermes execution as appropriate. The original independent PCZT oracle,
transparent-rejection and message-corruption falsifications, and broader
zec_prepare/zec_sign_verify/lib regression commands remain required. The native,
security, full recovered-effect, and actual-cleanup gates are not waived.

## Stop

Read-only inspection and these four source edits only. No formatter, compiler,
test, Cargo/npm, Git, evidence, integration, network, native process, other actor,
or dependency operation. All pending work remains uncommitted and preserved.
No build artifacts are needed; the existing target remains on repository ext4.

Report both final hashes/line counts, the four exact changes, and the unchanged
accepted test-module hash. Then stop. The reviewer collects once only after the
owner reports done; do not poll or duplicate actors.
