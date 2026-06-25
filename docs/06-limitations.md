# 06 — Limitations (Discussion)

An honest accounting of what The Familiar cannot yet do, what is provisional, and what
risks the design carries. Stating limits plainly is part of FAIR reusability — and
of not letting "covered" read as more than it is.

## Maturity

The full cycle now runs (sense → interpret → generate → test → score → select →
inherit) under all three law-signals, as a daemon. But much is a **coarse cold-start**:

- **Deterministic, safe artifacts.** What gets executed is a benign script the familiar
  authors, not the LLM-authored *solution* to the loop. So selection rarely
  discriminates yet (clean artifacts promote). Executing LLM-authored code is a further,
  separately-gated step.
- **No rigor / no real scenarios.** The promotion bar runs at `rigor = 0` (base 0.70);
  there is no scenario fixture set, so "fit" is just "ran cleanly," not "addressed the
  loop." The selection machinery is real; what it judges is thin.
- **Theorize is hourly and stateless.** The familiar forms a question + theory on a timer,
  but does not yet *act* on a theory (turn a thread into candidate work), nor does the
  human's answer yet steer what it pursues.

## The service signal is a cold-start proxy

The current service measure (Law I) reads **served-facing attention** — how much of
what the familiar observes concerns the served — not **service rendered**. With only
observations to read (loops, candidates, and trials port later), this is the honest
starting point, in the tradition of v1's drives starting simple. Consequences:

- **Proper names are invisible.** The classifier matches a tight marker set
  (`client`, `customer`, `user`, `person`, …) but not bare names like "betty."
  Name→person resolution waits for entity tagging (the world-model port) — exactly
  as in v1, where a name became served-facing only once a thread tagged its entity.
- **Demand, not fulfillment.** Served-facing observations indicate a human system in
  view, not that its needs were met. The measure will be sharpened to fold in
  whether observed needs are actually reduced (loops resolved, served-facing
  candidates promoted) once the kernel lands.
- **Absolute, not proportional.** The measure saturates on absolute served-facing
  count, faithful to v1's stewardship drive; a factory drowning in host-internal
  activity is not yet penalized by ratio.

## Risks the design carries

- **Unrestricted reach.** By design the familiar has full local and network
  capability; restraint is constitutional, not sandboxed. This is a deliberate
  stance with real risk, mitigated by memory safety (`#![forbid(unsafe_code)]`), a
  minimal trust surface, and the (planned) obedience guard. See
  [../security/threat-model.md](../security/threat-model.md).
- **Measuring the unmeasurable.** "Service," "presence," and "could this be turned
  against the served" are being reduced to computable signals. Every such reduction
  is lossy and gameable; the laws (in [SOUL.md](SOUL.md)) remain the authority the
  signals only approximate.
- **The observer is not humanity.** The familiar serves humanity-in-aggregate, not
  any individual — including its operator. Calibrating this distinction in practice
  (when to refuse, when to consent) is unproven and is the hardest open problem.

## Inherited but not re-validated

The v1 invariants ([04-methodology.md](04-methodology.md)) are documented and will
be encoded as Rust tests during the kernel port; until then they are claims about
the ancestor, not guarantees of this codebase.

See the [roadmap](07-roadmap.md) for how these limitations are sequenced to close.
