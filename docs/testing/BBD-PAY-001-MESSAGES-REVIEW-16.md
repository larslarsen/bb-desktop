# Messages UI evidence review 16

Reviewer: Codex. Decision: accept retained UI assertion-completion and visual evidence;
execution-record compliance and final security/publication acceptance remain incomplete.
No tests, syntax checks, scans, builds or Electron executed by reviewer.
HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`; index empty.

## Functional outcome

[Hermes UI06 report](BBD-PAY-001-HERMES-UI-VERIFY-06.md) and the actual ignored
artifacts in dist/pay001-ui-verify06/ are present. All 21 current input hashes and line
counts match review-05 pins with review-15's 1599-line driver override:
`f09c2dcc214b4fb97699d77eecf7407391cdcff5d1785cf850a1d59455abaf54`.
The retained driver copy matches exactly; before-manifest counts now match actual lines.

The retained electron-smoke.json is written only after the complete journey's assertions.
It contains no fixture failures or unexpected blocked requests. Its presence on the
reviewed driver, successful bootstrap/sandbox records and four matching screenshots
support accepting assertion completion for this UI slice. This is not proof of compliant
execution capture or an independently verified process exit; those limits are below.

The reached assertions cover empty and request/mixed conversations, native pointer and
Enter details toggling, long-history geometry, focus/open/draft/node/anchor preservation
during automatic status/text updates, bottom following, cancellation, in-place expiry,
identity mismatch/recovery, and unavailable/recovery on the same payment endpoint.
Renderer proof records Seccomp 2, NoNewPrivs 1 and NSpid `754714 4`.

Visual inspection confirms the narrow request image shows Payment request, 1.00000000 ZEC,
zec-testnet/requested, long memo and open details in Messages. The mixed image shows two
peer rows with the request/cancellation preview. Unavailable image shows the payment
notice while retaining text history. These are isolated fixture data, not owner payments.
The existing Feed Refresh button is outside payment/message automatic-update controls.
No request creation, approval, broadcast or portable-account/WebRTC implementation is
claimed. The read-only v1 scope and architecture direction remain unchanged.

## Corrections to the actor report

- Enter toggles details; the journey does not prove sending a chat message with Enter.
  Its /ob/chat POST is the typing path, not proof of successful message sending.
- electron-smoke.json retains only the first 12 social/payment hits and first 8 expected
  blocks. It contains 12 social samples (11 GET, one POST), 12 payment samples, and
  8 expected-block samples, not the claimed totals 11/12/7. Samples do not establish
  complete traffic counts. No response status is retained in these sampled hit objects,
  so “all 200” is not established by that artifact.
- The UI report's before/after-manifest claim lacks source-after.json and runtime-after.json.
  Current reviewer hashing establishes current identities only.

## Execution evidence gaps and gate contradiction

Metadata files exist under syntax-metadata.json/ui-metadata.json (hyphens rather than
specified dots; spelling is not the material issue). Both report exit 0 with hashes that
match their empty raw logs. Their argv omits the authorized timeout wrapper; the timeout
field is a duration string, not an observed timeout outcome. No executor wrapper is
retained. Exact wrapper status, child status and capture timing are not independently
corroborated by process records.

prelaunch-gate.json claims all checks passed at 01:45:00Z and syntax exit 0, while syntax
metadata says the command ended at 01:45:01Z. As written, the gate claims a completed
result before its recorded completion. It also accepts a 177-byte count-only runtime
summary, contrary to the required complete manifests, omits the initialized UI metadata
check and lacks most file hashes. Therefore the declared prelaunch gate is rejected.
Do not treat its booleans as proof of checks performed before launch.

Raw preflight shows disk space 154G while JSON/report say 160G. Raw helper/electron hashes
are present, but full original/copied runtime inventories, after manifests and the
executor source are missing. Recover actual contemporaneous records if available;
otherwise explicitly mark their provenance/timing unavailable. No reconstruction,
backdating, replacement of original artifacts or unchanged UI rerun is authorized.
Current observations must be recorded as current, not renamed execution-time evidence.

The accepted functional artifacts remain useful despite these process-record gaps.
This review does not retroactively declare the mandatory execution gate satisfied.
Final acceptance/publication is pending the remaining checks and reviewer decision.

## Verified artifact identities

Paths relative to dist/pay001-ui-verify06/.

| Artifact | SHA-256 |
| --- | --- |
| driver-source.js | f09c2dcc214b4fb97699d77eecf7407391cdcff5d1785cf850a1d59455abaf54 |
| source-before.json | 1bd6b6d1e0a0cb5f81410edf30c79b3d24943a232b296d9d38dcc2893591eb4e |
| syntax-metadata.json | 722dda05ad6e95ef4d1dbea372c6de0d992d331208aa076bce32fd7d3ce31306 |
| ui-metadata.json | aac1b64f5a13ff74fc1f2ad289ef4d5f7c103c12fcaff201c5338a922b6f0537 |
| prelaunch-gate.json | c5b0c3a5907023da4b2160222a64412addd09d78d24c5274d6744e98e50be648 |
| ui-01/bootstrap-diagnostics.json | cfca21dd7bae3a90a5a57ebcf6970c43925c5033383a5f05a1840b7d7b4e6064 |
| ui-01/sandbox-status.json | 36df1f36056b548360741f21d251b4de7f4f6a763a80e4b56bfb8ba3140901cd |
| ui-01/electron-smoke.json | 8a9a68ac084b8a09d9cba44419bb54915873b9e6019b8d95c0744eef9cf50644 |
| ui-01/fixture-empty-messages-1440x1000.png | 823a825e90b8f6553857416fac68ae7c1394db3f125768229d44daecbf1c25ee |
| ui-01/fixture-request-card-980x720.png | b95be9287698d5f609ad04bab89518d13cffe80e45e615fd4db94ea2e1c3d8b4 |
| ui-01/fixture-mixed-1440x1000.png | 24be369581e766e022eb267061643a6ee42e7b4c00fd18099c536c20c4be0efc |
| ui-01/fixture-unavailable-980x720.png | ca8dddea98e7b9ab16e4a1fedfd0f3cb080383dd8b949e9fa3b8fe13dd89f3c7 |

All screenshot hashes/dimensions match electron-smoke.json. Wide images are 1440x934;
narrow images are 980x654. No passing suite is rerun for clerical completeness.

## Next authority

[Hermes evidence closeout and scans 01](../handoff/HERMES_BBD_PAY_001_MESSAGES_SCANS_01.md)
is sole execution authority. Close UI06, retain its original artifacts, document corrections
and provenance gaps, then execute the remaining bounded policy/audit/secret checks.
The prospective scan input is explicitly enumerated in the handoff's path list; it is
not Git/publication authority. Prior Node and UI assertion proof stands. Inherited policy
failures remain release blockers. No source changes, actor launch or user-process restart.

Governance paths: this review, scans handoff and exact scan-path list, closed UI06 handoff,
CURRENT_TASK, PAY-001 ticket and end-to-end status map.
