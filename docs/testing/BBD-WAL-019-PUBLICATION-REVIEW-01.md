# WAL-019 publication review 01

Decision: feature publication verified; publication evidence REQUIRES CORRECTION.
Runtime and local wallet refresh remain accepted. Final closeout is withheld.

## Verified by Codex

- Local master and live origin master both resolve to
  `f2d9c9a513b3285531352df84ada818fdac0faf6`; its parent is the required
  `2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e` baseline.
- The commit contains exactly the authorized 20 paths. All 20 working files equal
  their committed blobs; the staging index is empty. Unrelated dirty work remains.
- All ten green-handoff pins match both working and committed bytes, including
  source/test line counts. Both wallet binaries remain 552768544 bytes with the
  accepted digest; the 115-byte manifest and pinned scanner also match the green
  review and publication handoff. No reviewer tests, builds or scans were run.
- Every retained red/green command log matches its metadata digest. Runtime
  acceptance in [green review 01](BBD-WAL-019-GREEN-REVIEW-01.md) stands.
- GitHub Actions API returned no workflow runs for this exact feature commit.
  This supplies no CI pass claim.

## Findings blocking closeout

1. **P1 — Final staged scan is not substantiated.** Publication report lines 81–84
   claim two exit-0 scans and unchanged staged bytes. The entire designated capture
   directory contains only `scan1.log`: 188 bytes, SHA-256
   `b674dc3b9301edb1307805f36b4bc6f795f7dc1f0d0c2ec33d889e9eb7af54f2`.
   It reports approximately 120035 scanned bytes and no leaks. There is no retained
   argv/exit metadata, second log, or staged-tree identity. The log alone cannot
   establish final-byte coverage or the two claimed exit codes. The scanner version
   is described as embedded metadata, without the required version-command output.
   This is missing evidence, not proof that a scan failed or never ran. A fresh
   captured scan of the exact published commit can establish present coverage; it
   must not be represented as recovery of the historical second scan.
2. **P2 — Required report corrections remain incomplete.** Green report line 54
   still has the wrong mutant hash (`...1736008...` instead of `...1736e08...`).
   The actual digest in `04-mutation.json` is
   `94b7a4d173e3b422f211a785241736e08d5247117f06db884b6cda79b04780f7`.
   Command descriptions omit timeout arguments; several metadata digests remain
   placeholders and neither report links its raw artifacts. The red outcome still
   calls production the authorized next step before its later corrective paragraph.
   Publication baseline prose incorrectly describes the old HEAD as before/after
   publication, and the report omits the actual resulting commit and remote ref.

## Resolution

Owner may relay
[Hermes correction 01](../handoff/HERMES_BBD_WAL_019_PUBLICATION_CORRECTION_01.md).
It authorizes only record correction, captured published-commit/staged scans and an
exact eight-path corrective commit. Source, tests, artifacts and prior captures
remain frozen. Do not amend or rerun the old handoff. No actor was launched by Codex.
Inherited release blockers and incomplete end-to-end payments remain unchanged.
