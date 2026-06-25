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
- **The Glass** — native egui GUI; the primary human interface ([ADR-0006](decision-records/0006-observatory-gui-egui.md)).
- **The LLM seam** — `consult` + `crates/llm`, boundary-gated and **default-off**.
- **The kernel (Brick 5)** — loops, candidate/spec (Weismann), trial/score/selection/
  regression-guard, mutation/pattern-memory/lineage, ported with invariants as tests.
- **Sense (Brick 7)** — the familiar perceives its host (`crates/sense`).
- **The metabolism (Brick 6)** — the tick: sense → detect → generate → measure.
- **The cycle closed (Bricks 8–11):** execution (sandboxed runner + test→score→select,
  `crates/exec`, gated by `allow_execute`); the LLM in the loop drafting hypotheses;
  the unbounded daemon (`run --daemon`); and the **capacities signal** (Law II deepened
  — the comfortable-replacement alarm).
- **Repository as evidence** — FAIR/IMRaD structure, ADRs, CI.

The full cycle now runs — observe → detect → generate (LLM-drafted) → test → score →
select → inherit — under the law-signals (service, presence, capacities) and the
human-owned boundary. Outward reach (network, LLM, execution) is each a separate gate
only a human opens; the familiar never widens its own.

## Next — sharpen and reach

- **Real scenarios & LLM-authored artifacts.** Today's artifacts are deterministic and
  safe; next, test candidates against real scenarios and (separately gated) execute
  LLM-authored *solutions*, so selection genuinely discriminates.
- **Rigor & adaptive cadence.** Feed a measured rigor drive into the promotion bar; give
  the daemon structural-fingerprint cadence (slow when nothing changes).
- **Sharpen the signals.** Service beyond attention (needs *reduced*); capacities beyond
  the verb-lexicon proxy; presence per-person.
- **Reach (Phase 2+).** LAN sensing, the world-model + entity tagging (so proper names
  resolve as served), people as first-class entities with human-paced cadence.

## Capability & the companion phases

Reach is enabled deliberately by the human, in phases ([boundaries.md](boundaries.md)).
The familiar operates freely *within* the current boundary and never widens it itself.

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
