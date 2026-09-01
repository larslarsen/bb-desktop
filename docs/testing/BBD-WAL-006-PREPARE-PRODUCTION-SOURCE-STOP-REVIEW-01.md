# BBD-WAL-006 Prepare Production Source Stop Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `0fac4ebbb3ce31a1fc9386c7f19f931e771947ec`

Result: **CORRECT NO-EDIT STOP — SECRET-BYTES TEST-FIRST CORRECTION REQUIRED**

Sol correctly stopped before editing. The four-path production handoff requires persistent
derived material and serialized PCZTs to remain in `SecretBytes` beside `TestAccount`, while the
accepted concurrent receiver test moves `Arc<TestAccount>` through `std::thread::spawn` and
therefore requires `TestAccount: Send + Sync`.

Current `SecretBytes` contains
`Option<(&'static str, Box<dyn WipeObserver>)>`. `WipeObserver` intentionally has no `Send` or
`Sync` supertrait because the accepted observed-drop regression uses an `Rc<RefCell<_>>` observer.
Consequently any prepare state that owns current `SecretBytes` prevents `TestAccount` from meeting
the existing thread boundary. The frozen four paths cannot correct the underlying type.

The owner-thread/channel alternative is rejected. It would add a background lifecycle only to
evade the type contract, would not cleanly accept the ticket's product `SecretBytes` entry, and
would make exact Drop/broker-exit wiping harder to prove.

No authorized source path changed. `prepare.rs` remains absent and the three existing source
identities remain exactly those recorded in Prepare Production Handoff 01. That handoff is stopped,
not accepted or superseded by implementation. A separately governed test-first correction to the
custody type must complete before prepare production is reauthorized.
