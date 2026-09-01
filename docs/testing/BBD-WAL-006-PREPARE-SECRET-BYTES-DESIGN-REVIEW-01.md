# BBD-WAL-006 Prepare Secret-Bytes Design Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Result: **DESIGN ACCEPTED — COMPILE-TIME TEST SOURCE AUTHORIZED FIRST**

A bounded Grok CLI review was attempted but did not return a verdict and was terminated without
repository changes. The reviewer independently inspected the complete pinned `secrecy 0.10.3`
implementation: `SecretSlice<u8>` is `SecretBox<[u8]>`, whose only field is `Box<[u8]>`; it has no
unsafe or negative auto-trait implementation. The observer field is therefore the sole current
`SecretBytes` obstacle to sound automatic `Send + Sync`.

## Accepted correction shape

After an independent compile-time red is integrated, a later production handoff may:

- keep ordinary `SecretBytes` owning only `SecretSlice<u8>` and preserve its unconditional
  zeroizing Drop, redacted Debug/Display, exposure closure, replacement wipe, and public behavior;
- introduce public `ObservedSecretBytes` solely as the return type of
  `SecretBytes::new_observed`, owning a `SecretBytes`, label, and `Box<dyn WipeObserver>`;
- implement `ObservedSecretBytes::drop` by invoking
  `SecretBytes::wipe_with(label, observer.as_mut())` before its inner empty `SecretBytes` drops;
  this must produce exactly one post-zeroize observation and remain safe if observation unwinds;
- keep the existing non-`Send` `Rc<RefCell<_>>` observer regression source unchanged and inferred
  as the new wrapper type; and
- add no `unsafe`, manual `Send`/`Sync` impl, trait-object auto-trait bound, thread, channel, global,
  dependency, feature, or alternate secret container.

The public return-type change is bounded: repository search finds `new_observed` used only by the
drop-only inferred local in `secret_hygiene.rs`. The wrapper must have redacted Debug/Display if it
implements either and must expose no accidental secret formatting or serialization.

## Test-first requirement

The first authorized change is one regression in `wallet-broker/tests/secret_hygiene.rs` that
instantiates a generic `T: Send + Sync` assertion with `SecretBytes`. Current production must fail
to compile specifically because `(dyn WipeObserver + 'static)` is neither `Send` nor `Sync`.
Production `vault.rs`, ZEC source, existing tests, and every other path remain frozen until the
reviewer accepts that expected-red evidence.
