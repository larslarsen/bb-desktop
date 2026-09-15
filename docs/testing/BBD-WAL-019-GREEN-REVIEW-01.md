# WAL-019 green and local wallet refresh review 01

Decision: WAL-019 runtime behavior and local development wallet refresh accepted.
Publication remains pending the separate exact-path handoff. Codex reviewed raw
captures and current files without executing tests or rebuilding.

HEAD: `2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e`. All ten handoff inputs match
preflight, restoration, postflight and current bytes. Every command log matches its
metadata hash; the captured commands and exits match the authorized sequence.

| Gate | Verified outcome |
| --- | --- |
| Formatter | Exit 0, clean |
| Focused green | Exit 0; 6 passed, 16 filtered |
| Falsification | Exit 101; 1 failed, 21 filtered; actual start list empty |
| Restored green | Exit 0; 6 passed, 16 filtered |
| Entire account_native_ui target | Exit 0; 22 passed, none failed/ignored/filtered |
| Development wallet build/stage | Exit 0; matching debug/staged binary and manifest |

The full account UI target includes an intentional caught `CANARY_DIALOG_PANIC` in
the export-dialog test; that test passes. This is not a new unhandled failure. The
22-test result is the account UI integration target, not every native or Rust test.
No global security, full proving suite, release or end-to-end payment claim follows.

## Independently verified identities

Accepted `wallet-broker/src/account_ui.rs`: 1332 lines, SHA-256
`09d5c5cbcb7867a06574ebb99426f4331e4d789bd4ec29c1c47122f3566c1f6a`.
Frozen `wallet-broker/tests/account_native_ui.rs`: 2654 lines, SHA-256
`e7ac08d26fad3eb0302c49b8983b62cb28a6aab5bad657b142c2e84d59a11446`.

The reviewer reconstructed the exact one-block mutant from the retained original
source. Its SHA-256 is
`94b7a4d173e3b422f211a785241736e08d5247117f06db884b6cda79b04780f7`, matching
`04-mutation.json`. It replaces only the list's begin_sync call with navigation.
The test fails at line 2368 on an empty start-call list. The backup, restored source
and accepted source are byte-identical. This proves dispatch, not just painted UI.

Both `wallet-broker/target/debug/bitbook-wallet-broker` and
`wallet-broker/target/app-resources/wallet-broker/bitbook-wallet-broker` are
552768544 bytes and hash to
`6f0b2ce78a7d871bde75d1fa8f330bd27dc7a00f598853dcc60944b92d15f985`.
The manifest's platform is linux, arch x64 and its SHA-256 field matches that binary.
The 115-byte manifest itself hashes to
`dfa323db0a754e094456fbaf1651e46c2a8f9a77281055b0512bd97782109bcd`.
These are debug development artifacts, approximately 527.2 MiB, not release size.
Electron's development launch configuration selects this resource directory.
The process was not restarted; the next normal app restart loads the refreshed
wallet. The separate daemon executable remains unchanged.

## Report corrections required before publication

The raw evidence passes, but the green summary incorrectly prints mutation hash
`c90f08d321b686cb2fc199f2c1f774ebadc3b6b3c7fca07dc4c48e7e4f2c05c1` and omits
the full binary/manifest hashes behind an ellipsis. It calls the account UI target
the full native-ui suite, labels the in-process restoration snapshot an exit-0
command, omits exact command arrays/relative clickable links, and ends by saying
record edits were not authorized. The handoff explicitly authorized both reports.
Correct these descriptions from the actual metadata; do not rerun tests.

The red report now fixes the tool versions, failure stage, PATH prose and adds raw
hashes. Its residual statement that production was already the authorized next
step should explicitly date that authorization to the later reviewer decision.
Session identity remains unavailable; do not infer it from version output. The
provider/model remains executor-reported `nous` / `meituan/longcat-2.0:free`.

Next: `docs/handoff/HERMES_BBD_WAL_019_PUBLICATION_01.md` authorizes the precise
report corrections, staged-content secret scan and scoped publication. All source
is frozen. Existing unrelated WAL-009, policy work and release blockers stay open.
