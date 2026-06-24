# 03 — System Architecture (Methods I)

> This is the narrative overview. The living, detailed reference is
> [`ARCHITECTURE.md`](ARCHITECTURE.md); the decisions behind it are in [`decision-records/`](decision-records/).

## The hybrid

Substrate is split in two, deliberately:

- **A compiled, deterministic kernel** — `crates/kernel` (Rust). The parts that
  must be reproducible, traceable, and safe: records, persistence, lineage, trial,
  selection, memory, and the obedience guard.
- **An interpreted / data-driven / generated periphery** — the behavior the factory
  mutates *freely, without recompiling itself*: generated artifacts (scripts run
  under resource limits), data-file parameters, and the LLM seam (shelled out).

This split is not a compromise; it *is* the principle "the model is not the factory"
and "thin stable kernel, everything else fluid." The slow-to-compile core changes
rarely because evolution happens in the periphery.

## Why Rust (decision: [ADR-0003](decision-records/0003-rust-for-the-kernel.md))

Chosen against the Three Laws and the constrained hardware the factory should run
on (where the served are):

- **Law III** makes memory safety *constitutional*. `crates/kernel` carries
  `#![forbid(unsafe_code)]` — a long-running autonomous process with unrestricted
  reach must not contain the memory-unsafety that becomes a remote-code-execution
  path "turned against the served."
- **Law I** wants a lean, no-GC, tiny-static-binary core for small hosts.
- Minimal dependencies (currently `serde`/`serde_json`) keep the trust surface
  small and auditable — also Law III.

## Crate map

```
crates/
  kernel/   substrate-kernel (lib) — the deterministic core
    store.rs        JSONL append/load (serde); data-dir resolution
    observation.rs  the observation record (the only truth)
    service.rs      the service signal (Law I)
    presence.rs     the presence signal (Law II)        [planned]
    guard.rs        the obedience guard (Law III)        [planned]
    # evolutionary kernel (loop/candidate/trial/...) ports in next [planned]
  cli/      substrate-cli (bin: `substrate`) — the thin shell
```

## The cycle (the metabolism)

Once the kernel ports, the autonomous tick is:

```
Observe → Name → Interpret → Generate → Bound → Test → Score → Select → Inherit → Return
```

with the law-signals woven through it: **service** (Law I) and **presence**
(Law II) are read continuously, and the **obedience guard** (Law III) sits at the
*Bound*/act steps as an active gate that can allow, seek consent, or refuse — not a
passive warning. The LLM boundary keeps the v1 protocol (`prompt → call_llm.sh →
response`), the canonical periphery seam.

## Storage

JSONL-in / JSONL-out via `serde`, append-only, one file per record type under a
data directory (`substrate_data/` default, `--data-dir` override). Local-first and
auditable; the factory sends no telemetry and exfiltrates nothing. The record model
and schema live in [`../data/`](../data/). SQLite is a deferred option once the file
model is proven.
