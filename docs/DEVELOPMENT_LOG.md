# Development Log

The linear handoff trail for Substrate v2. Newest entries on top. Before making
architectural changes, read `SOUL.md` (the Three Laws) and `ARCHITECTURE.md`, then
the latest entries here.

Each entry: what changed, why, checks run, what the next developer should know.

## 2026-06-24 — Brick 2: the service signal (Law I)

### What changed

- `crates/kernel/src/service.rs` — **Law I made measurable.** `service_signal(&[Observation])`
  returns a `ServiceSignal { measure (0..1), served_facing, total, exemplar }`: zero when
  nothing observed touches the served, rising (saturating, `n/(n+3)`) with served-facing
  attention. Faithful to v1's *absolute, saturating* stewardship drive (not a ratio).
- Classifier `names_served` is a faithful port of v1's `domain_is_steward`
  (`factory/src/drive.c`) — a tight lowercase marker set.
- CLI `service` reports the signal; when zero it prints "continuation unjustified by service
  (Law I)".

### Why

Law I says continuation *is* service, so the factory must be able to see whether it is serving.
This is the cold-start sight: with only observations to read (loops/candidates/trials port
later), it measures served-facing *attention* — the honest proxy for service, the way v1's
drives started on promotion-rate before redundancy. Elevation over v1: there, stewardship was
one drive among three; here service is the first-class signal continuation is weighed against.

### Checks run

- Green bar clean. 9 kernel tests (incl. classifier markers-not-bare-names, zero-when-none,
  monotonic rise, empty-log-zero).
- Live: host-internal-only log → `service signal 0.00` + the Law I line; adding two
  served-facing observations → `0.40 (2 of 3; e.g. client)`. No real `unsafe` in the kernel.

### Next

Known cold-start limit: proper names ("betty") aren't yet served-facing — name→person
resolution waits for the world-model/entity-tagging port (as in v1, where a name became
served-facing only once a thread tagged its entity). Then Brick 3 — the presence signal (Law II).

## 2026-06-24 — Brick 1: the observation spine

### What changed

- `crates/kernel/src/observation.rs` — `Observation { id, source, actor, action, object,
  context, ts, confidence }`, a faithful port of v1's `observation_t`, as a `serde` struct over
  `store`. `record()` assigns sequential ids (`obs-NNNN`) and appends; `load()` reads oldest-first.
- CLI `observe` / `observations`, with hand-rolled, dependency-free flag parsing. The CLI stamps
  wall-clock `ts` so the kernel stays clock-free and deterministic in tests.

### Why

The thinnest possible spine — the substrate the law-signals compute over (not "machine first").
Observations are the only truth; everything else derives from them.

### Checks run

- Green bar clean. 5 tests (store round-trip/edge + sequential-id / round-trip / explicit-id).
  Live: two observes round-trip through JSONL and list back.

## 2026-06-24 — Brick 0: Cargo workspace scaffolding

### What changed

- Stood up the Rust workspace: `crates/kernel` (`substrate-kernel`, lib) and
  `crates/cli` (`substrate-cli`, bin `substrate`). Edition 2021; deps held to
  `serde` + `serde_json` only.
- `crates/kernel/src/lib.rs` carries `#![forbid(unsafe_code)]` — the Law III
  commitment made literal.
- `store.rs`: generic JSONL append/load over any `serde` record, with `--data-dir`
  resolution (default `substrate_data/`). Replaces v1's hand-rolled `json_util.c`.
  A missing file is an empty log; blank lines skip; a malformed line is a hard
  error (corruption surfaces early, never silently changes derived state).
- `docs/ARCHITECTURE.md` (Rust + hybrid + crate map) and this log.

### Why

The substrate decision (compiled core; Rust; hybrid) was made *after* the
constitution and *before* the first kernel code — the order v1 got wrong. This
brick is the thinnest possible standing repo, the spine the law-signals attach to.

### Checks run

- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` — all clean.
- `store.rs` unit tests: missing-file-is-empty, append/load round-trips in order,
  blank-skip / malformed-errors.

### Next

Brick 1 — the observation record (faithful port of v1 `observation_t`) on top of
`store.rs`, with `substrate observe`. Then Brick 2 — the service signal (Law I).
