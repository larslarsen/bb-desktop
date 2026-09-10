# WAL-013 native window source correction — after actor41323 completes

Sol gpt-5.6-sol High, exact wallet-broker/src/account_ui.rs only. No tests, sourceother
paths, execution/formatters/Git/evidence/delegation. Recorded UI escalation continues.
Read target and relevant installed APIs only; baseline actor41323 finalhash to be
recorded at dispatch. Reviewer source findings, correct ones still present in finaldrop:

1. SharedAccountPort::confirm_restore currently invokes private
AccountError::unavailable from sibling module (compile blocker). Instead return the
service Result<Option<summary>> through with_manager then .ok_or(UNAVAILABLE) at the
public-code layer. Never change AccountError visibility or service source.
2. MaskedInput::take_secret moves a capacity1024 String into SecretBytes::new, whose
Vec::into_boxed_slice may shrink/reallocate secret allocation. Transfer via a fresh
exact-length vec/boxed allocation, copy only at explicit submission, immediately wipe
original preallocated buffer fullcapacity before constructing SecretBytes. Keep
original buffer capacity1024 for reuse. Do not move secret through a shrinking Vec.
No secretgetter or plaintext egui/TextEdit/context history.
3. MaskedInput allocates a clickable widget then another interactive ID for same rect,
creating an unused focusable stop that breaks Tab. Allocate geometry without focusable
interaction (e.g. allocate_exact_size Sense::hover), then one stable interactive ID.
Remove unused response/modifiers bindings, preserve real pointer/Tab/IME handling.
4. Close/quit can return before UI widget consumes queued Text/Paste/IME strings.
Explicitly scrub BOTH ctx.input events/raw.events on close/quit/hidden pass, with no
password target. Do not scrub valid focused input before active widget handles it.
Ensure any remaining text events on an active UI pass are wiped after widgets so
unfocused/out-of-form strings never remain in egui event buffers.

Review production for these exact defects; do not redesign or expand scope. No tests
executed here; accepted9widget tests remain frozen. Stop with final hash/count.

## Collected baseline and remaining work

Actor41323 collectedexit0. account_ui.rs final2b3e57f48b72d5312059b48902a8ae263fe5b3ab510eb18c127c0b5384a742ad (931lines), lib9b39a0f773260a1445b87908ba341ca414728a19906227279a5b202247e5f770.
Actor self-review already corrected findings1and3; preserve those accepted changes.
Only remaining findings2and4 require edits: exactallocation transfer and eventcleanup.
Use a zero-filled boxed slice of exact length BEFORE copying secret, then into_vec
(capacity==len), wipe original fullcapacity, hand to SecretBytes (no shrinkneeded).
Do not repeat broad selfreview or fullfileoutput. Two small corrections only.


## One-line finish authorization

Sol47428 collectedexit0, account_ui.rs7970abf1be88127fb8eae31aec8f396ca6869acc77cfccec7427612698cb462c,949lines.
Transfer/scrubbing accepted except a new hidden-branch earlyreturn violates original
hidden tick/repaint contract. Sol High source-only may REMOVE ONLY the `return;`
immediately after `scrub_secret_events(context);` in the `if !self.control.visible()`
branch inside service (around539). Keep scrubbing and fall through to refresh_accounts
and request_repaint_after, including hidden passes. All other returns/source unchanged.
No broader reads, tests, formatter or Git. Reporthash/count and stop.
