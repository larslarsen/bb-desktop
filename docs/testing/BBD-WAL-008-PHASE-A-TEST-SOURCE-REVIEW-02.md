# BBD-WAL-008 Phase-A Test-Source Review 02

Reviewer: Lead Engineer/Reviewer — Codex at High

Protected governance parent: `a5aa1ab3`

Reviewed identities:

| Path | Lines | SHA-256 | Decision |
| --- | ---: | --- | --- |
| `wallet-broker/Cargo.toml` | 117 | `7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530` | accepted and unchanged |
| `wallet-broker/tests/zec_hardware.rs` | 740 | `bd1a58fda190817c9b55cc6975e59a1234e16e7e5755b53cdd8fa47d83e51e8a` | one correction required |

Result: **TEST SOURCE NOT YET ACCEPTED**

Correction 01 properly separates spend from viewing state, completes the positive
forbidden-authority assertions, adds the durable narrowing/restoration sequence and
directory-sync fault, scans persistence as bytes, and exhaustively fixes the ASCII
fingerprint alphabet.

One non-vacuity defect remains. The redaction test asserts positive canary touch counts
only after both installation and exercised flows. An implementation could increment
the counters during `install_observable_canaries_for_test` and never carry the canaries
through the sensitive paths. Immediately after installation, assert every slot holds
its exact canary **and has touch count zero**. Retain the existing positive touch-count
assertions after decide, persistence, failure, and panic. That creates the required
zero-to-positive observation and prevents installation from satisfying its own oracle.

No execution is authorized. The correction is one test path only.
