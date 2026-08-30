# BitBook Desktop Testing Strategy

This policy applies SQLite's reliability-oriented testing strategy, as adopted by Keel,
to BitBook's Electron client. Automated evidence, not reviewer intuition or visual
inspection alone, is the primary bug-finding layer. SQLite's scale and 100% branch
coverage are inspirations, not claims about this repository's current maturity.

## Standing Rules

1. **Tests lead implementation.** Every behavior change starts with a focused test that
   fails for the intended reason. The active ticket names the authorized red and green
   commands. Production code follows only after the red result is understood.
2. **Falsify important tests before trusting them.** Temporarily suppress or break the
   mechanism under test and prove the test fails. Do not commit the falsification change;
   report the method and result.
3. **One regression test per bug.** A bug fix is incomplete until a test reproduces the
   bug before the fix and passes after it.
4. **Assert non-vacuous outcomes.** Empty DOM selections, ignored events, swallowed
   promise rejections, or a process that merely survives startup are not proof of the
   behavior being claimed.
5. **Exercise the real boundary.** Core tests may isolate pure logic, but integration
   tests must traverse the actual renderer event, daemon API, navigation guard, security
   boundary, or packaging path named by the ticket.

## Required Layers

Use the smallest combination that proves the ticket:

- pure deterministic unit and property tests for social state, normalization, ordering,
  idempotence, escaping, and URL or API validation;
- boundary tests immediately below, at, and above defined input, storage, pagination,
  retry, and timeout limits;
- fixture-driven renderer tests for DOM events and daemon responses, including empty,
  malformed, delayed, disconnected, and hostile inputs;
- failure injection for daemon unavailability, request cancellation, reconnects,
  permission denial, navigation attempts, and corrupt persisted state;
- compound-failure tests when cleanup or recovery itself encounters another failure;
- Electron security assertions for sandboxing, context isolation, disabled Node
  integration, external-navigation policy, and denied permissions; and
- native smoke or package-structure checks only when packaging behavior changes.

Tests must be offline, deterministic, credential-free, and independent of a live public
daemon or mutable third-party service. Manual visual QA complements tests; it never
replaces an assertion.

Security-critical normalization and navigation results should have an independent oracle
where practical, such as canonical fixtures evaluated outside the production helper.
Tests must detect leaked listeners, timers, child processes, handles, and temporary state
when the touched boundary owns those resources.

## Ticket and Review Contract

Each implementation ticket must state:

- the invariant or user-visible behavior being proved;
- authorized test paths before production paths;
- the exact targeted red command and expected failure;
- the exact targeted green command and broader acceptance commands;
- how at least one high-value test will be falsified; and
- which unit, fixture, failure, security, and native layers are relevant.

If dependencies, build inputs, renderer privileges, navigation, daemon communication, or
release content can change, the ticket must also name the applicable security scans,
their exact commands, and the finding threshold that blocks acceptance.

The ticket-authorized implementation developer authors bounded test source before
production source, but does not execute tests. Jr Dev — Codex Luna integrates the test-only
drop first, records the expected red result, integrates the production drop, runs the
targeted and broader acceptance commands, and publishes the implementation evidence.
The reviewer independently inspects the tests, rejects tautological or shortcut proofs,
and accepts or rejects the integrated evidence.

## CI and Coverage

- Normal pushes run fast deterministic checks and tests.
- Windows, Linux, and macOS release packaging stays manually triggered until product
  distribution makes continuous native verification worthwhile.
- Instrumented coverage checks validate the test suite; release-critical checks must also
  exercise the packaged configuration users actually receive.
- Coverage is a ratchet, not a vanity target. Establish floors for maintained social,
  API, and security-critical modules as meaningful tests land; raise them, and never
  lower one without a ticketed rationale.

Human or model review remains necessary for architecture, security, and UX semantics,
but is not a substitute for executable tests.

## Security and Supply-Chain Evidence

- Audit the lockfile-resolved npm dependency graph, execute fail-closed security-boundary
  and dangerous-sink checks over maintained application source, and scan committed
  content for secrets before acceptance. Use a pinned third-party static-analysis tool
  only when it supports the maintained stack and findings produce a blocking result.
- Establish a reviewed baseline for inherited findings. New findings fail the ratchet;
  critical or plausibly exploitable findings require immediate review regardless of the
  baseline.
- A suppression requires a ticketed rationale, owner, affected version or path, and
  expiry or removal condition. Severity labels alone do not replace exploitability
  review.
- Generate a machine-readable SPDX or CycloneDX SBOM from each platform's exact packaged
  application inputs. Attach it to the release and scan the package or its SBOM before
  publication. Routine pushes do not need to build packages or regenerate release SBOMs.
- Pin external scanner and SBOM-generator versions. Preserve reports when a tool produces
  them as CI or release evidence; executable local security checks remain durable test
  evidence. Network access is limited to fetching tools and signed advisory data; tests
  must not depend on a mutable service response.

Lineage: [How SQLite Is Tested](https://www.sqlite.org/testing.html).
