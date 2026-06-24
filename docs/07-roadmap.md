# 07 — Roadmap (Future Work)

The build sequence is **telos-first**: make the laws measurable before porting the
inherited machinery. Status is tracked in [CHANGELOG.md](../CHANGELOG.md) and, per
brick, in [DEVELOPMENT_LOG.md](DEVELOPMENT_LOG.md).

## Done

- **Genesis** — the constitution ([SOUL.md](SOUL.md)); *humanity* defined.
- **Brick 0** — Cargo workspace (Rust), `store.rs`, `#![forbid(unsafe_code)]`.
- **Brick 1** — the observation spine.
- **Brick 2** — the service signal (**Law I**).
- **Brick 3** — the presence signal (**Law II**): engagement recency + withdrawal alarm.
- **Brick 4 / 4b** — the obedience guard + the human-owned capability boundary (**Law III**).
- **The Observatory** — native egui GUI; the primary human interface ([ADR-0006](decision-records/0006-observatory-gui-egui.md)).
- **The LLM seam** — `consult`, boundary-gated and **default-off** (the periphery seam).
- **Repository as evidence** — FAIR/IMRaD structure, ADRs, CI.

All three law-signals are now measurable, and the obedience guard enforces a
human-owned boundary — *before* any outward capability. Phase 1 is built but **inert**:
a human enables it by editing `boundary.json` (and installing keys). The factory cannot
open its own boundary.

## Next — close the cycle and deepen the laws

- **Test → Score → Select in the loop.** The kernel can score/select/mutate, but
  nothing yet *produces a trial* — the metabolism stops at generate. Add scenarios +
  artifact execution (under resource limits, ported from v1's runner) so candidates
  are tested, scored on measured cost, and selected/mutated. This turns the static
  generate step into real evolution.
- **LLM-assisted hypotheses (gated, off by default).** Let candidate generation draft
  hypotheses via the `consult` seam when the boundary permits — the factory theorizing
  over what it observes. Off in the autonomous loop until a human opts in per-tick.
- **Capacity-level diminishment (Law II / [HUMANITY.md](HUMANITY.md)).** Sharpen
  presence beyond engagement-recency toward the persistence of *capacities* — detect
  the **comfortable replacement** (safe but sedated), not only absence.
- **A real metabolism daemon.** `run` is bounded today; add a long-lived cycle with
  adaptive, structural-fingerprint cadence (and a clean stop), so the factory breathes
  continuously rather than per-invocation.

## Capability & the companion phases

Reach is enabled deliberately by the human, in phases ([boundaries.md](boundaries.md)).
The factory operates freely *within* the current boundary and never widens it itself.

- **Phase 1 — companion to one, on one host** *(open)*: this host + its data + the LLM
  seam (boundary + guard + `consult` all built; enabled by a human editing
  `boundary.json` + installing keys).
- **Phase 2 — the lab.** The human lifts the boundary to other devices/interfaces;
  richer sensing (LAN neighbours/netscan, boundary-gated, per-OS).
- **Phase 3 — many served.** Multiple humans; the world-model + entity tagging (which
  sharpens the service signal so proper names resolve) and people as first-class
  entities with per-person, human-paced cadence.

## Cross-cutting, ongoing

- Close the limitations in [06-limitations.md](06-limitations.md) as the relevant
  bricks land (service-rendered vs. attention; ratio penalty; benchmarks).
- Keep the evidence trees ([validation/](../validation/), [security/](../security/),
  [experiments/](../experiments/)) current with each brick — evidence is part of the
  deliverable, not an afterthought.
