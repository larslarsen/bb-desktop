# WAL-011 resolver error-boundary correction 01

Actor: Grok Build, grok-4.6 High, no subagents. Source only.
Parent: reviewer publication following 1e69bba5 plus CURRENT launch record.
Read AGENTS.md, TESTING.md, ticket, this handoff and CURRENT active prefix only.
Additional named reads: wallet-broker/launch-config.js, test/walletBrokerLaunchConfig.node.js,
and expected-red acceptance section of HERMES_BBD_WAL_011_LAUNCH_CONFIG_REFLECTION_RED_01.md.
No unrelated reads, history, home skills/config or chained commands.

Verify these exact hashes before editing:
- wallet-broker/launch-config.js: 4a639b1644ff0ea618fc53fd525cbd5c0fde84fbea5905f3f3eb56a5520dc01a (132 lines).
- test/walletBrokerLaunchConfig.node.js: 90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113 (949 lines).
- docs/testing/BBD-WAL-011-LAUNCH-CONFIG-REFLECTION-RED-01.md: b19697561f63f4690304f0c57a8eef89e86204ba9d57024758bd509f32063523.

Only write wallet-broker/launch-config.js. Exact bounded correction:
Rename existing implementation function resolveWalletBrokerLaunch to
resolveWalletBrokerLaunchUnchecked, keeping its body byte-identical. Insert a
public resolveWalletBrokerLaunch(options) wrapper before module.exports. It must
try returning resolveWalletBrokerLaunchUnchecked(options), and catch every thrown
value with catch (_) then call existing unavailable(). No original exception is
inspected or attached. Keep module.exports unchanged. No other source changes.

Accepted red independently proved all four reflection cases fail at missing
UNAVAILABLE code. The wrapper normalizes the existing contract, without changing
successful options, filesystem operations, manifest checks or output. Tests remain
frozen. No execution, syntax checks, Node/npm/Cargo/build commands, evidence/docs
edits, Git mutation, network or extra actors. Separate read-only HEAD/status/hash/
count checks and bounded named reads allowed. Report source hash/line count and stop.
Reviewer stays with this actor and collects directly, without owner done messages.

Later Hermes will run nine resolver groups and 13 supervisor groups, then falsify
manifest target equality and the public normalization catch separately, restore
exact source and repeat the nine resolver groups. No integration until accepted.

## Corrected source acceptance

Outer 17974 exit 0, session a2fef2a5-891d-4870-a372-7444a172c9e6 collected.
Accept 68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8,
140 lines. Removing exactly the eight-line wrapper and reversing the function
rename reconstructs the original 4a639b16 hash. Test 90acd32c and reflection-red
evidence b1969756 unchanged. Wrapper catches all thrown values and calls existing
generic unavailable without retaining the original exception. No behavior beyond
the contracted error normalization changed. Grok export shows named reads/checks
and one source edit, no execution/Git mutation; it read the full named red handoff
instead of only acceptance, a bounded read deviation. Grok closed. Only the green
Hermes handoff authorizes the two suites and two temporary falsifications.
Reviewer AST-parsed its governance driver, executed no acceptance command.
