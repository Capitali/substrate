# 02 — Research Basis (Background)

The ideas Substrate stands on, and how they shape it.

## FAIR and FAIR4RS — why this repository is shaped the way it is

The **FAIR principles** (Findable, Accessible, Interoperable, Reusable; Wilkinson
et al., *Scientific Data*, 2016) were framed for scientific data but explicitly
extend to algorithms, tools, and workflows. **FAIR4RS** (Chue Hong et al., 2022)
adapts them to research software, emphasizing transparency, reproducibility, and
reuse.

Substrate treats its repository as a research artifact governed by FAIR4RS:

- **Findable** — rich metadata ([`CITATION.cff`](../CITATION.cff)), a navigable
  [README](../README.md), versioned releases ([CHANGELOG](../CHANGELOG.md)).
- **Accessible** — open license ([Apache-2.0](../LICENSE)), plain-text formats,
  no proprietary gate.
- **Interoperable** — JSONL records with a published [schema](../data/schema/), a
  stable CLI contract, standard tooling (Cargo, GitHub Actions).
- **Reusable** — clear provenance (the [lab notebook](DEVELOPMENT_LOG.md) and
  [ADRs](decision-records/)), tests as executable specification, documented limitations.

The presentation follows **IMRaD** (Introduction → Methods → Results → Discussion),
the standard scientific narrative — mapped across `docs/00`–`07`.

## Artificial life and the social drive

Substrate's evolutionary core descends from artificial-life thinking: populations
of candidates under variation and selection, fitness measured from the environment
rather than decreed. A particular influence is the line of work on **social /
community drives in artificial life** — agents whose behavior is shaped by an urge
toward the systems and beings around them, not only toward individual fitness. In
v1 this appeared as the "reach" and "stewardship" directions; in Substrate it is
elevated to the constitutional floor (Law I: continuation *is* service).

## The seed: a normative vision

[`seed.txt`](seed.txt) is not decoration. It is the normative target rendered as
fiction — a near-future in which a "mesh" serves flood-stricken neighborhoods and
floating classrooms, and is judged by lived outcomes: needs grouped *by need, not
status*; behavior remembered over credentials; correction accepted; consent sought
before acting; a tool that "makes forgetting harder." Recurring design pressure is
drawn directly from it:

| From `seed.txt` | Becomes, in Substrate |
|---|---|
| "needs grouped by need, not status" | the service signal (Law I), not a priority hierarchy |
| "the mesh remembered behavior" | pattern memory, advertisements (observed, not declared) |
| "it asked before sending… accepted correction" | the obedience guard (Law III); input parity |
| "a machine that praised you for sleeping was too close to management" | service is stewardship of systems, never management of people |
| "a tool that made forgetting harder" | failures preserved as memory; the lab notebook |

## Inherited engineering basis

The disciplined-evolution method — observe → name → interpret → generate → bound →
test → score → select → inherit → return, with failures preserved and no unchanged
retries — is inherited from the v1 factory and is documented, with its invariants,
in [04-methodology.md](04-methodology.md) and the architecture docs.
