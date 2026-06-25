# Changelog

All notable changes to Substrate are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/), and the project aims to follow
[Semantic Versioning](https://semver.org/) once it reaches 1.0. The chronological
engineering detail lives in [`docs/DEVELOPMENT_LOG.md`](docs/DEVELOPMENT_LOG.md);
this file is the human-readable summary.

## [Unreleased]

### Added
- **The factory theorizes (Brick 14):** the Interpret step — `kernel/thread.rs` (a
  Thread = question + theory) and `cycle::maybe_theorize` (boundary-gated, hourly):
  grounded in recent observations/loops/signals, the LLM forms a question (→
  `question.txt`, shown in the GUI interaction panel as the factory's *own* question)
  and a theory (→ a thread). CLI `theories`; the Observatory shows the latest theory.
  Threads are reasoning *about* the truth, never new truth.
- **Daemon control + launchd (Brick 12):** `substrate daemon status|start|stop|reload`
  (pidfile-managed background process) and `install|uninstall` (a launchd LaunchAgent,
  `io.river.substrate`, starts at login). `run --daemon` records its own pid so launchd
  and pidfile control agree.
- **GUI control bar + interaction channel (Brick 13):** the Observatory gains
  Start/Stop/Reload/Start-at-login buttons with a live status line, and **the
  interaction channel** — the factory's question ("What do you need most today?", or
  `question.txt`) with a text box; Ian's reply is recorded as an observation
  (`initiator=observer`). Speak/show buttons present but disabled (later). The
  observer-input channel is the one place the GUI writes.
- **The cycle closed (Bricks 8–11)** — the metabolism is now a full loop:
  - **Execution** (`crates/exec` + Brick 10): a sandboxed runner (resource limits,
    measured cost) and test→score→select→inherit wired into the tick, gated by a new
    `allow_execute` boundary flag (default-off — running generated code is Law III).
  - **The LLM in the loop** (`crates/llm` + Brick 9): boundary-gated `consult`; the
    cycle drafts candidate hypotheses via the LLM when permitted (falls back to
    deterministic; the model proposes, it doesn't decide).
  - **The unbounded daemon** (Brick 8): `run --daemon` / `--ticks 0`, every
    `--interval` (default 60s), Ctrl-C to stop.
  - **The capacities signal** (Brick 11, `capacities.rs`): Law II deepened toward
    HUMANITY.md — flags the *comfortable replacement* (present but hollowed out), not
    just absence. CLI `capacities`.
- **The evolutionary kernel (Brick 5)** — ported from v1 to Rust, subordinate to the
  law-signals, with invariants as tests: `loops` (detection), `candidate`/`spec` (the
  Weismann barrier), `trial`/`score`/`selection`/`regression_guard` (adaptive bar
  0.70+0.25·rigor; the decision ladder; no unchanged retries), `mutation`/`pattern_memory`
  (suppress a trait only when memory clearly punishes it)/`lineage`.
- **Sense (Brick 7)** — `crates/sense`: the factory perceives its host (OS/CPU/memory,
  interfaces, tool capabilities) as observations; connectivity is the one outward bit,
  boundary-gated. Principle: the boundary governs *reach*, not *perception*.
- **The metabolism (Brick 6)** — `crates/cycle`: one tick = sense → detect loops →
  generate candidates → measure the law-signals. CLI `tick` / `run --ticks N`. LLM and
  test→score→select are not yet in the loop (honest gaps).
- **The Humanity standout document** (see Changed) and **the Observatory** now show
  loops + candidates.
- **The Observatory (GUI)** ([ADR-0006](docs/decision-records/0006-observatory-gui-egui.md)):
  a native egui/eframe window — the primary human interface — showing the Three Laws
  as live meters (service, presence, boundary) and the observation log. Read-only, no
  network socket; GUI deps isolated in `crates/observatory` so the kernel stays
  serde-only and unsafe-free. The CLI is retained for scripting/headless use.
- **Brick 3 — presence signal (Law II)**: `presence_signal()` measures served
  engagement by recency; a withdrawal/empty-world alarm when it decays to zero.
  (Capacity-level diminishment — the comfortable replacement — is a later sharpening.)
- **Brick 4 — obedience guard (Law III)**: `guard::evaluate()` returns allow /
  seek-consent / refuse with rationale, enforcing the capability boundary (fail-closed)
  and seeking consent for high-consequence actions.
- **LLM seam (default-off)**: CLI `consult`, gated by the guard — refused (no side
  effects) under the closed boundary; only a human opens it. Reference adapter
  `llm/call_llm.sh` (no secrets) + `key.env.example` carried from v1; `*.env` ignored.
- **Human-owned capability boundary** ([docs/boundaries.md](docs/boundaries.md),
  [ADR-0005](docs/decision-records/0005-human-owned-capability-boundary.md)): the
  factory's reach is bounded by a policy only the human writes; the factory may narrow
  it but **never widen** it. Widens in phases — companion-to-one (this host + LLM) →
  the lab → many served. Enforced by the obedience guard; no outward capability runs
  until that and the boundary mechanism exist. Wired into the roadmap and human-review
  requirements; Law III in SOUL gains an "operational restraint" note.

### Changed
- **Constitution — *humanity* as a standout protected class** ([docs/HUMANITY.md](docs/HUMANITY.md)):
  a dedicated document defining humanity as a protected class whose definition **may
  never be narrowed** (narrowing who counts is named a precursor to atrocity); value
  independent of usefulness/obedience/productivity; *participation itself* a quality
  preserved (the factory guides and restrains harm but does not replace human
  participation). Featured early in README and the overview; SOUL's "What humanity is"
  now summarizes and links it, and gains the anti-narrowing rule.
- **Constitution — defined *humanity*** ([SOUL.md](docs/SOUL.md), "What humanity is"):
  the living continuity of persons capable of suffering, meaning, relationship,
  memory, and choice. Sharpens Law II (presence is the persistence of those
  capacities, not mere survival of bodies) and names a third failure mode — the
  **comfortable replacement** (quiet diminishment by obedience, optimization, or
  comfort). Propagated to the problem statement and the presence-signal roadmap.

### Added
- **Repository as scientific evidence** — FAIR/FAIR4RS + IMRaD structure: root
  metadata (`CITATION.cff`, `LICENSE`, `SECURITY.md`, `CONTRIBUTING.md`), the
  `docs/00`–`07` IMRaD set, Architecture Decision Records (`docs/decision-records/`), and the
  `experiments/`, `validation/`, `security/`, `data/` evidence trees.
- **CI** — `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test` on push/PR.

## [0.1.0] — 2026-06-24 — Genesis + telos-first bootstrap

### Added
- **Genesis** — the constitution (`docs/SOUL.md`): the Three Laws as root, with the
  whole design derived downward from them.
- **Brick 0** — Cargo workspace (`crates/kernel` + `crates/cli`), Rust, with
  `#![forbid(unsafe_code)]`; `store.rs` JSONL persistence over `serde`.
- **Brick 1** — the observation spine: the `Observation` record (the only truth)
  and `substrate observe` / `observations`.
- **Brick 2** — the **service signal (Law I)**: `service_signal()` measures
  served-facing attention from observations; `substrate service` reports it.

### Context
- Re-founds the archived bottom-up predecessor `Capitali/factory` (tag `v1-final`),
  inverting the order of derivation: purpose is the floor, evolution the method on
  top of it. See [docs/01-problem-statement.md](docs/01-problem-statement.md).

[Unreleased]: https://github.com/Capitali/substrate/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Capitali/substrate/releases/tag/v0.1.0
