# Architecture

> How Substrate is built. The *why* is `SOUL.md`; this is the *how*. Where they
> conflict, the Soul wins.

## The hybrid: compiled kernel + evolvable periphery

Substrate is split in two, deliberately:

- **A compiled, deterministic kernel** (this is `crates/kernel`, in Rust) — the
  records, persistence, lineage, trial, selection, memory, and the obedience
  guard. The parts that must be reproducible, traceable, and safe.
- **An interpreted / data-driven / generated periphery** — the behavior the
  factory mutates *freely, without recompiling itself*: generated artifacts
  (shell scripts run under resource limits), data-file parameters, and the LLM
  seam (`llm/call_llm.sh`, shelled out).

This split is not a compromise; it *is* "the LLM is not the factory" and "thin
stable kernel, everything else fluid." The slow-to-compile core changes rarely
because evolution happens in the periphery.

## Language: Rust

The kernel is **Rust**, chosen against the Three Laws and the hardware the factory
should run on (Pis, Cerbo armv7, the router — *where the served are*):

- **Law III (cannot be turned against the served)** makes memory safety
  constitutional, not a nicety. `crates/kernel` carries `#![forbid(unsafe_code)]`
  — the commitment made literal. A long-running autonomous process with
  unrestricted local + network reach must not contain the memory-unsafety that
  becomes a remote-code-execution path.
- **Law I (cheap survival)** wants a lean, no-GC, tiny-static-binary core for
  constrained hardware. Rust gives that without sacrificing safety.
- Minimal dependencies (`serde`, `serde_json` only, so far) keep the trust
  surface small and auditable — also Law III.

## Crate map

```
crates/
  kernel/   substrate-kernel (lib)  — the deterministic core
    store.rs        JSONL append/load (serde); the data-dir
    observation.rs  the observation record (the only truth)
    service.rs      the service signal (Law I)
    presence.rs     the presence signal (Law II)        [planned]
    guard.rs        the obedience guard (Law III)        [planned]
    # the evolutionary kernel (loop/candidate/trial/...) ports in next [planned]
  cli/      substrate-cli (bin: `substrate`) — the thin shell (scripting/headless)
  observatory/  substrate-observatory (bin: `observatory`) — the GUI (primary human
                interface; egui/eframe; read-only; GUI deps isolated here so the
                kernel stays serde-only and unsafe-free). See ADR-0006.
```

## Interfaces

The **Observatory** (native egui GUI) is the primary human interface — a local
window showing the Three Laws as live meters and the observation log, read-only and
with no network socket (Law III restraint). The **CLI** (`substrate`) is retained
for scripting, automation, and headless/CI use. Both are thin shells over the same
kernel.

## Storage

JSONL-in / JSONL-out via `serde`, append-only, one file per record type under a
data directory (`substrate_data/` by default, `--data-dir` to override).
Local-first and auditable; the factory sends no telemetry and exfiltrates nothing
(restraint is constitutional). SQLite remains a deferred option once the file
model is proven.

## Discipline (the green bar)

Every change must pass, with no exceptions:

- `cargo fmt --check`
- `cargo clippy -- -D warnings` (warnings are errors)
- `cargo test`
- no `unsafe` in `crates/kernel` (enforced by `#![forbid(unsafe_code)]`)
