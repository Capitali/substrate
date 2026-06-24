# Security Policy

Substrate is a long-running autonomous process with **unrestricted local and
network reach** by design. Its restraint is constitutional, not technical (it
sends no telemetry and exfiltrates nothing — see [SOUL.md](docs/SOUL.md), Law III
and "restraint is constitutional"). Security is therefore a first-class concern,
and a vulnerability here is, in the project's own terms, a path by which the
factory could "be turned against the served."

## Reporting a vulnerability

Please report privately, **not** via a public issue:

- Use GitHub's **private vulnerability reporting** ("Report a vulnerability" under
  the Security tab), or
- email **ian@river.io** with `[substrate-security]` in the subject.

Include what you found, how to reproduce it, and the impact you foresee. You will
get an acknowledgement; please allow reasonable time to remediate before any public
disclosure.

## Supported versions

Pre-1.0. Only the tip of `main` is supported; there are no maintenance branches yet.

## Design commitments that bear on security

- **Memory safety as constitution.** The kernel (`crates/kernel`) carries
  `#![forbid(unsafe_code)]` — the Law III commitment made literal. A memory-safety
  defect in an unrestricted-reach agent is exactly the kind of "turned against the
  served" failure Law III forbids.
- **Minimal trust surface.** Dependencies are kept deliberately small (currently
  `serde`/`serde_json` only). See [security/dependency-review.md](security/dependency-review.md).
- **No exfiltration.** The factory does not phone home. See
  [security/privacy-review.md](security/privacy-review.md) and
  [security/threat-model.md](security/threat-model.md).

See [security/](security/) for the full threat model, data classification, and reviews.
