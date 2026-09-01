# BBD-WAL-006 Prepare Gate Targeted Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected parent: `fcae3338`

Result: **GATE STOPPED AT COMMAND 3 — CLOSED STAGE DIAGNOSTIC REQUIRED**

Jr Dev — Hermes v0.18.2 used provider `nous` and model `meituan/longcat-2.0:free`. Command 1,
Rust 1.98.0 formatting, exited 0 with no diff. Command 2, the exact locked/offline
warnings-denied Clippy gate, exited 0 with no warning or diagnostic. Command 3 exited 101 with
3 passed and 8 failed; Hermes stopped without commands 4 through 6 or any mutation, evidence,
stage, commit, or push.

All validation/binding rejection tests passed. Every valid prepare path failed after spend access;
most exposed the public `INTERNAL` code. Static review localizes that erased error to one of four
post-proposal stages: official `create_pczt_from_proposal`, PCZT serialization, secret ownership,
or parse. The public error mapping deliberately erases the upstream variant, so source inspection
cannot prove which stage failed. Do not weaken the physical read-only wallet boundary on inference.

Static review also found a later independent inspection defect: pinned IO finalization signs
protocol-padding spends and clears their dummy keys. Post-finalization `dummy_sk().is_none()` is
therefore not a real-spend discriminator, and checking signatures across all actions rejects
allowed padding signatures. The public retained spend witness distinguishes the real input, but no
production correction is authorized until the current `INTERNAL` stage is captured.

Authorize one temporary closed diagnostic marker at each erased post-proposal stage, one exact
single-test free-Hermes execution, and immediate exact source restoration. Markers may contain only
fixed stage names, never upstream error text, paths, SQL, values, or secret material.
