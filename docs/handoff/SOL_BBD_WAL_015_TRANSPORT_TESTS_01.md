# WAL-015 deterministic transport boundary tests

Principal Dev Sol gpt-5.6-sol High test-source actor, under the ticket's documented
escalation. Parent remains reviewer; another Sol actor owns production files.
Read AGENTS.md/TESTING.md and tickets/BBD-WAL-015.md. Author ONLY:
- wallet-broker/tests/zec_live_transport.rs (new integration target, auto-discovered);
- wallet-broker/tests/fixtures/live-grpc-server.js (new bounded test fixture).
No other files, execution, formatters, dependencies, Git, evidence, or other actors.

Prove real tonic HTTP/2+TLS request/response behavior against an owned loopback Node
server. Node and OpenSSL executables are available; no new Rust dev dependency is
needed. Test-owned temporary root under wallet-broker/target, 0700; generate ephemeral
localhost certificate/key at test runtime using openssl req with SAN localhost / IP
127.0.0.1, no committed private key or certificate. Never use a real user profile.
Rust tests launch/reap the Node fixture; explicit bounded child/stdout/stderr handling,
close all pipes and join reader threads before exact-file cleanup. Validate own root
and files before cleanup; no remove_dir_all, recursive deletion or symlink traversal.

Expected future test-support API (do not implement it yourself):
`probe_live_transport_for_test(endpoint: &str, ca_pem: Option<&[u8]>,
 cancellation: LiveCancellation, deadline: Duration) -> Result<LiveTransportProbe, ZecError>`.
Result fields: tip_height:u32, block_count:usize. It invokes the REAL production
transport's GetLightdInfo, GetLatestBlock, GetTreeState(tip-1), GetBlockRange(tip,tip),
with identical serialization, TLS/stream limits, cancellation and deadline machinery.
The optional generated trust root and shorter test deadline are only test seams;
the production constructor always uses built-in WebPKI roots and 15 seconds.
No account, seed, UFVK or wallet path enters this probe.

Use the actual upstream protobuf types and prost::Message (direct dependency now
authorized). Build deterministic response bytes in the Rust test and pass bounded
configuration to the Node child via stdin (base64ct/serde_json already available).
Node uses built-in http2 and TLS only. It must listen on 127.0.0.1:0, report readiness
on stdout, record exact request method path and body, and return real gRPC framing
and grpc-status trailers. Bound input/body sizes; reject unknown RPCs. Capture request
frames to assert no address/transaction methods or extra account-specific fields.

Roughly three tests, not an elaborate fake service framework:
1. Trust the runtime-generated CA only through the test seam; all four real gRPC RPCs
   succeed, parsed result matches independently encoded tip/block; inspect/decode actual
   requests and method allowlist. The same local server with ca_pem=None must fail TLS.
2. Return an oversized compact-block frame or unexpected extra block; actual tonic
   limit/source validation rejects it. Do not pre-reject solely inside the test helper.
3. Stall the GetBlockRange response after headers and one message. A short complete-
   stream deadline fails promptly; a separate call canceled after the server reports
   that request exits promptly too. Verify child/reader cleanup with bounded waits.

Avoid arbitrary sleeps when readiness/request observation can synchronize the test.
No mutable public endpoint is contacted. Fixed codes are CANCELLED, LIMIT,
PROTOCOL_INCOMPATIBLE or UNAVAILABLE as appropriate; diagnostics must not include
server response canaries. Prefer simple grouping over extensive fault matrices.
Report a usable test drop and API assumptions; stop for reviewer/Hermes.
