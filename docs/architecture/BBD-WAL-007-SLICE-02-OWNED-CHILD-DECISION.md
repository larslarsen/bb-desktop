# BBD-WAL-007 Slice-2 Exact Owned-Child Decision

Decision: **USE THE EXACT OWNED CHILD — CORRECT THE PROCESS-GROUP TEST CLAIM**

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance baseline: `2218125d`

## Finding

Sol correctly stopped before source implementation. Rust 1.98 stable provides safe
`std::os::unix::process::CommandExt::process_group` to assign a child process group, but
stable `std::process::Child::kill` terminates only the direct child. The safe
`ChildExt::kill_process_group` API exists in the Rust 1.98 documentation only as the
nightly experimental `unix_kill_process_group` feature.

Authoritative API references:

- <https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html>
- <https://doc.rust-lang.org/std/process/struct.Child.html>

Implementing group signaling on the frozen stable toolchain would therefore require a
new process-control dependency, first-party libc/FFI/unsafe, a helper process, or nightly
Rust. Each is prohibited by BBD-WAL-007. Claiming group kill while calling only
`Child::kill` would make the test false evidence.

## Decision

The ticket's teardown wording already permits killing the exact `child/process group`.
Slice 2 will use the exact broker-owned `Child` handle, call safe stable `Child::kill`
only when the two-second graceful stop fails, and always wait/reap that exact child.
It will never signal an arbitrary PID, reused PID, caller-supplied ID, unrelated process,
or helper. The frozen test must describe and assert that exact owned-child behavior,
not process-group signaling.

The official pinned `monero-wallet-rpc` is executed directly with detach and pidfile
disabled; BBD-WAL-007 grants no descendant-process adoption. If a later maintained
binary requires child-tree teardown, that is a separate dependency/toolchain and threat
review rather than a claim made by this ticket.

No production source is authorized until the corrected 12-test source is reviewed,
integrated, and shown expected-red on the still-absent `xmr::process` module.
