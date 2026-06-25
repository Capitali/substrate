# 05 — Validation and Results

What has been demonstrated, and how to reproduce it. The full cycle now runs live; the
results below are real, though several signals are still coarse cold-starts (see
[06-limitations.md](06-limitations.md)).

## Test suite (current)

The test suite is the executable specification: **~69 tests across the workspace**, all
passing, covering the kernel (records, the law-signals, loops/candidate/spec with the
Weismann barrier, score/selection/regression-guard, mutation/pattern-memory/lineage,
threads, capacities), the sandboxed runner, and the cycle (incl. execute-closes-the-loop
and idempotence). The invariants are encoded as tests (the adaptive promotion bar
`0.70+0.25·rigor`, pattern suppression `neg>pos`, the Weismann barrier, the regression
guard). Run: `cargo test`. Plan: [../validation/test-plan.md](../validation/test-plan.md).

## The full cycle, live

On the developer's Mac, with the boundary opened to Phase 1 + execute, a single tick
demonstrated the whole loop end-to-end: it **sensed** the host, **interpreted** (formed
a grounded question — *"are you working on projects involving network tunnels or
compiling C/Rust?"* — and a theory from the utun interfaces + toolchain + a recurring
loop), **generated** a candidate with an **LLM-drafted** hypothesis, **executed** the
artifact under the sandboxed runner, **scored** it (`pass`), and **selected** (promoted).
The metabolism runs as a daemon and theorizes hourly.

## Green bar

Every committed brick passes, and CI enforces:

```
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

plus no `unsafe` in `crates/kernel` (compile-enforced by `#![forbid(unsafe_code)]`).

## Result: Law I is measurable end-to-end

The service signal (Law I) was demonstrated over real records via the CLI:

```
# host-internal observation only
$ substrate observe --actor host --action reports --object cpu_load
$ substrate service
service signal 0.00 (0 of 1 observations touch the served)
  no served-facing activity observed — continuation unjustified by service (Law I)

# add served-facing observations
$ substrate observe --actor client --action requests --object status_report
$ substrate observe --actor support_team --action resolves --object customer_ticket
$ substrate service
service signal 0.40 (2 of 3 observations touch the served; e.g. client)
```

This is the central claim of the bootstrap: the first thing the familiar does is
*measure whether it is serving*, and it reports "continuation unjustified by
service" when it is not — Law I, operational rather than aspirational. The
experiment record is [../experiments/experiment-001/](../experiments/experiment-001/).

## What is **not** yet validated

- No benchmarks (performance/footprint) yet — see
  [../validation/benchmark-results.md](../validation/benchmark-results.md).
- The service measure is a cold-start proxy (served-facing *attention*, not service
  *rendered*); its accuracy as a service proxy is not yet meaningful — see
  [../validation/accuracy-metrics.md](../validation/accuracy-metrics.md) and
  [06-limitations.md](06-limitations.md).
- The evolutionary kernel (loop/candidate/trial/selection) is not yet ported, so
  its invariants are documented but not yet re-validated in Rust.
- Known failures and gaps: [../validation/known-failures.md](../validation/known-failures.md).
