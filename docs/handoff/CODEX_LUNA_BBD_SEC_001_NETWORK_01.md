# Codex Luna Resume — BBD-SEC-001 Network Boundary 01

Status: SUPERSEDED FOR EXECUTION by Correction 03. Preserve this file as historical
evidence; do not run its Electronegativity command. The SBOM disk paths and bounded npm
settings remain applicable when Luna reaches the unchanged manual SBOM sequence.

You are **Jr Dev — Codex Luna** (`gpt-5.6-luna`). This file supplements, and does not
replace, `CODEX_LUNA_BBD_SEC_001.md`.

## Preserved state

The prior integration turn was interrupted only after an escalated Electronegativity
download remained silent indefinitely. The earlier sandbox attempt failed with npm
`EAI_AGAIN`. The interrupted process produced no accepted scanner result and made no
repository, evidence, artifact, staging, commit, or push change.

Before that network boundary, Luna completed and restored the bounded red and
falsification edits, verified the delivered hashes, and passed:

- Electron boundary tests: 12/12;
- corrected security-policy tests: 46/46;
- repository security-policy checker; and
- `npm audit --audit-level=low`: 0 vulnerabilities.

Reviewer independently reconciled the test counts and these exact current hashes:

```text
666a4de0022a4507ebe6c9d59e0c121b861369f4a1fef47f3ad28ec17ec2602a  test/securityPolicy.node.js
7ef0b8939622d014580f47a6e3eac75314aad9a5fc1d62e125eb2d1496f905ea  test/electronSecurity.node.js
cce184efe8b4056217057a4543537dcbe3adb052b1b8c8ce3b07b5b05132804b  scripts/security-policy.js
629154349962bf2692bee868c9bb753e0f731a950920e676183e7abdf79eb71e  .github/workflows/security.yml
```

## Resume authorization

Read every original handoff and ticket instruction again. Verify the four hashes above.
Do not reconstruct or repeat already completed destructive/falsification work. Resume at
the Electronegativity acceptance command.

The network-backed scanner commands must be foreground commands with a finite wrapper so
a silent registry/download failure cannot occupy the handoff indefinitely. Create only
these explicit disk-backed directories if missing; do not clean or delete anything:

```text
/home/lars/OpenBazaar/.security-cache/bbd-sec-001-20260829/npm
/home/lars/OpenBazaar/.security-tmp/bbd-sec-001-20260829
/home/lars/OpenBazaar/.security-artifacts/bbd-sec-001-20260829
```

Run Electronegativity as this exact foreground command from the repository root:

```sh
timeout --signal=TERM 180s env \
  npm_config_cache=/home/lars/OpenBazaar/.security-cache/bbd-sec-001-20260829/npm \
  npm_config_fetch_retries=1 \
  npm_config_fetch_retry_mintimeout=1000 \
  npm_config_fetch_retry_maxtimeout=5000 \
  npm_config_fetch_timeout=30000 \
  npx --yes @doyensec/electronegativity@1.10.3 -i social-main.js
```

Exit 124, a network error, a scanner error, or any finding is a stop condition. Report
the exact safe exit/result without editing source, evidence, or Git. Never expose finding
bodies or secrets.

If and only if it passes with zero findings, continue the remaining acceptance commands
in ticket order, using the existing explicit Gitleaks binary. The local manual SBOM
exercise may use the same npm cache and the same bounded network settings for this exact
generator invocation, writing only:

```text
/home/lars/OpenBazaar/.security-artifacts/bbd-sec-001-20260829/bitbook-desktop.cdx.json
```

```sh
timeout --signal=TERM 180s env \
  npm_config_cache=/home/lars/OpenBazaar/.security-cache/bbd-sec-001-20260829/npm \
  npm_config_fetch_retries=1 \
  npm_config_fetch_retry_mintimeout=1000 \
  npm_config_fetch_retry_maxtimeout=5000 \
  npm_config_fetch_timeout=30000 \
  npx --yes @cyclonedx/cyclonedx-npm@6.0.1 --output-format JSON \
  --output-file /home/lars/OpenBazaar/.security-artifacts/bbd-sec-001-20260829/bitbook-desktop.cdx.json
```

Validate and summarize the SBOM exactly as required by the ticket. Do not dispatch any
workflow or native package build. Do not use `/tmp`, `rm`, cleanup, global installs,
unresolved targets, or source repair. On complete green, write only the originally
authorized evidence/current-task documents, commit the exact ticket change, push, and
stop for reviewer acceptance.
