# 05 — Validation and Results

What has been demonstrated so far, and how to reproduce it. This is an honest
account of a project at genesis + bootstrap: the results are real but small.

## Test suite (current)

The kernel test suite is the executable specification. As of v0.1.0:

- **9 unit tests**, all passing.
  - `store`: missing-file-is-empty-log, append/load round-trips in order, blank
    lines skipped / malformed line is a hard error.
  - `observation`: sequential id assignment (`obs-NNNN`), JSONL round-trip field
    fidelity, explicit id preserved.
  - `service`: classifier matches markers (case-insensitive) but not bare proper
    names; zero when nothing serves; monotonic rise with served-facing attention;
    empty log is zero.

Run: `cargo test`. Full plan and intent: [../validation/test-plan.md](../validation/test-plan.md).

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

This is the central claim of the bootstrap: the first thing the factory does is
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
