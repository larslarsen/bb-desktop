# BBD-WAL-007 Slice-5 Green Resume 02 Rejection 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance parent: `67e7877d`

Result: **EXECUTION REJECTED — ONE-LINE TEST-ORACLE CORRECTION REQUIRED**

Hermes verified the protected parent, clean index, exact accepted/frozen worktree,
disk-backed target, and Hermes Agent v0.18.2 (2026.7.7.2) using provider `nous` and
model `meituan/longcat-2.0:free`. The Rust 1.98 formatter completed successfully. The
temporary durable-replay falsification then produced the exact expected selected-test
failure: exit 101, 0 passed, 1 failed, 14 filtered out, and no warning or compiler
diagnostic. Hermes restored the accepted 871-line receiver identity exactly.

The first full `xmr_receiver` green command compiled and ran all 15 tests, with 14
passing and only
`sanitized_xmr_view_has_exact_fields_and_all_spend_zec_hardware_caps_negative`
failing at `wallet-broker/tests/xmr_receiver.rs:234`. The loop tests each forbidden
field name with a raw substring search. Its forbidden value `"port"` is necessarily a
substring of the legitimate and required capability key
`"can_export_viewing_material"`; the assertion therefore rejects the intended
sanitized schema even when no exact `port` field exists. The narrow oracle repair is to
search for an exact serialized JSON key token, not an arbitrary substring.

The execution is independently rejected for command-scope violations. Hermes appended
`; echo "EXIT_CODE=$?"` to the formatter and falsification commands, and appended both
`2>&1` and the exit-code command to the first green command, despite the handoff's
explicit no-wrapper, no-redirection, no-alteration rule. No later green, Clippy, native,
policy, evidence, staging, commit, or push step ran.

Hermes restored the temporary falsification and confirmed all eight accepted source
identities and the frozen stop draft remain exact. Reviewer inspection confirms
`HEAD == origin/master == 67e7877d`, a clean index, the restored receiver SHA-256
`4a1fd4ddd8025ea3c64c443d2746f319e91e609526d00ef912548fbd20d833f0`,
the corrected store SHA-256
`3a7f4d5b8cc7b33e3596910ce0b9b10d2f760f24c3ccff98fd2941c410ee2df4`,
the frozen 59-line draft SHA-256
`20f07f0b44dae0f006e192d91b717b541487a857d4d920047bbfd68818705637`,
and a clean `git diff --check`.

Because Grok's weekly usage remains exhausted, Codex Sol at High alone may make the
linked one-line test-oracle correction. Production source is frozen. Hermes execution
and integration remain unauthorized pending XHigh source review; broader/final
acceptance and the real offline local-Monero gate remain unauthorized.
