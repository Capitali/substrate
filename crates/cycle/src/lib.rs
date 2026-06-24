//! The metabolism — one tick of the factory cycle.
//!
//! `Observe → Name → Generate → … → Return`, in the honest form available today:
//!
//! 1. **Sense** the host (perception; deduped by triple so static facts don't spam
//!    the log — only genuinely new facts are recorded). Connectivity is the one
//!    outward bit, gated by the caller.
//! 2. **Detect** loops over all observations.
//! 3. **Generate** a gen-0 candidate for any loop not yet covered.
//! 4. **Measure** the law-signals (service, presence).
//! 5. **Return** a report of what changed.
//!
//! Not yet in the loop (honest gaps): *test → score → select* await scenarios and
//! artifact execution; *LLM-assisted* hypothesis drafting awaits a gated, off-by-
//! default consult. Candidate generation is deterministic for now. The cycle never
//! reaches outward except the (gated) connectivity probe.

use std::collections::HashSet;
use std::io;
use std::path::Path;

use substrate_kernel::candidate::{self, Candidate};
use substrate_kernel::loops;
use substrate_kernel::observation;
use substrate_kernel::presence;
use substrate_kernel::service;
use substrate_sense as sense;

/// What one tick changed.
#[derive(Debug, Clone, PartialEq)]
pub struct TickReport {
    /// New observations recorded this tick (deduped against the existing log).
    pub sensed: usize,
    /// Loops detected (total, after this tick).
    pub loops: usize,
    /// Candidates generated this tick (one per newly-covered loop).
    pub new_candidates: usize,
    /// Of those candidates, how many got an LLM-drafted hypothesis.
    pub llm_hypotheses: usize,
    /// Service signal (Law I), 0..1.
    pub service: f64,
    /// Presence signal (Law II), 0..1.
    pub presence: f64,
    /// True when the served have withdrawn (Law II alarm).
    pub presence_withdrawn: bool,
}

/// Ask the LLM (boundary-gated) for a one-line hypothesis addressing a loop.
/// Returns None on refusal, error, or unparseable output (caller falls back to the
/// deterministic hypothesis). The model proposes; it does not decide.
fn draft_hypothesis(dir: &Path, lp: &loops::Loop) -> Option<String> {
    let triple = lp
        .description
        .strip_prefix("Repeated: ")
        .unwrap_or(&lp.description);
    let prompt = format!(
        "A recurring pattern (loop) was observed in the environment: \"{triple}\" \
         (actor|action|object). In ONE sentence, propose a hypothesis for how to serve \
         the people involved by reducing this loop's friction — honoring that humanity \
         is served, not managed, obeyed, or optimized away. \
         Reply ONLY as compact JSON: {{\"hypothesis\":\"...\"}}."
    );
    match substrate_llm::consult(dir, &prompt) {
        Ok(substrate_llm::Outcome::Response(json)) => {
            serde_json::from_str::<serde_json::Value>(&json)
                .ok()
                .and_then(|v| {
                    v.get("hypothesis")
                        .and_then(|h| h.as_str())
                        .map(str::to_string)
                })
                .filter(|s| !s.trim().is_empty())
        }
        _ => None,
    }
}

fn triple(o: &observation::Observation) -> (String, String, String) {
    (o.actor.clone(), o.action.clone(), o.object.clone())
}

/// Run one tick over the data dir. `allow_connectivity` and `allow_llm` must reflect
/// the obedience guard's verdicts (the caller computes them from the boundary; see
/// [`tick_gated`]); all other steps are local perception and internal work. When
/// `allow_llm` is false the cycle never reaches the LLM — candidate hypotheses are
/// deterministic, and tests stay offline.
pub fn tick(
    dir: &Path,
    now: i64,
    allow_connectivity: bool,
    allow_llm: bool,
) -> io::Result<TickReport> {
    // 1. Sense — record only triples not already present (structural dedup).
    let mut seen: HashSet<(String, String, String)> =
        observation::load(dir)?.iter().map(triple).collect();
    let mut perceived = Vec::new();
    perceived.extend(sense::census(now));
    perceived.extend(sense::interfaces(now));
    perceived.extend(sense::capabilities(now, sense::DEFAULT_TOOLS));
    if allow_connectivity {
        perceived.push(sense::connectivity(now));
    }
    let mut sensed = 0;
    for o in perceived {
        if seen.insert(triple(&o)) {
            observation::record(dir, o)?;
            sensed += 1;
        }
    }

    // 2. Detect loops (a pure rewrite).
    let obs = observation::load(dir)?;
    let detected = loops::detect(&obs);
    loops::save_all(dir, &detected)?;

    // 3. Generate a candidate for each uncovered loop.
    let cands = candidate::load(dir)?;
    let covered: HashSet<String> = cands.iter().map(|c| c.loop_id.clone()).collect();
    let mut seq = cands.len();
    let mut new_candidates = 0;
    let mut llm_hypotheses = 0;
    for lp in &detected {
        if !covered.contains(&lp.id) {
            seq += 1;
            let mut c = Candidate::from_loop(lp, format!("candidate-{seq:04}"));
            if allow_llm {
                if let Some(h) = draft_hypothesis(dir, lp) {
                    c.hypothesis = h;
                    llm_hypotheses += 1;
                }
            }
            candidate::append(dir, &c)?;
            new_candidates += 1;
        }
    }

    // 4. Measure the law-signals.
    let svc = service::service_signal(&obs);
    let pres = presence::presence_signal(&obs, now);

    Ok(TickReport {
        sensed,
        loops: detected.len(),
        new_candidates,
        llm_hypotheses,
        service: svc.measure,
        presence: pres.measure,
        presence_withdrawn: pres.withdrawn,
    })
}

/// Whether the boundary on disk permits an action of `kind` (fail-closed on error).
fn boundary_allows(dir: &Path, kind: substrate_kernel::guard::ActionKind) -> bool {
    use substrate_kernel::boundary;
    use substrate_kernel::guard::{self, Action, Decision};
    match boundary::load(dir) {
        Ok(b) => guard::evaluate(&Action::new(kind, "cycle"), &b).decision == Decision::Allow,
        Err(_) => false,
    }
}

/// Resolve whether the boundary permits the connectivity probe (a Network action).
pub fn connectivity_allowed(dir: &Path) -> bool {
    boundary_allows(dir, substrate_kernel::guard::ActionKind::Network)
}

/// Resolve whether the boundary permits LLM consultation.
pub fn llm_allowed(dir: &Path) -> bool {
    boundary_allows(dir, substrate_kernel::guard::ActionKind::Llm)
}

/// Convenience: a tick whose connectivity and LLM use are gated by the boundary on
/// disk. This is what the daemon runs — outward reach only where a human opened it.
pub fn tick_gated(dir: &Path, now: i64) -> io::Result<TickReport> {
    tick(dir, now, connectivity_allowed(dir), llm_allowed(dir))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    struct Temp(PathBuf);
    impl Temp {
        fn new(t: &str) -> Self {
            let p = std::env::temp_dir().join(format!("substrate_cycle_test_{t}"));
            let _ = fs::remove_dir_all(&p);
            fs::create_dir_all(&p).unwrap();
            Temp(p)
        }
    }
    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn seed_recurring(dir: &Path) {
        // a served-facing event that recurs -> should become a loop with a candidate
        for ts in [100, 200] {
            let o = observation::Observation::new(
                "client",
                "asks_for",
                "status_report",
                "",
                "test",
                ts,
                1.0,
            );
            observation::record(dir, o).unwrap();
        }
    }

    #[test]
    fn first_tick_senses_detects_and_generates() {
        let t = Temp::new("first");
        seed_recurring(&t.0);
        let r = tick(&t.0, 1_000_000, false, false).unwrap();
        assert!(r.sensed > 0, "host perception should record something");
        assert!(r.loops >= 1, "the recurring triple should form a loop");
        assert!(
            r.new_candidates >= 1,
            "an uncovered loop should get a candidate"
        );
        // a served-facing loop -> service signal is non-zero
        assert!(r.service > 0.0);
    }

    #[test]
    fn second_tick_is_idempotent_on_static_world() {
        let t = Temp::new("idem");
        seed_recurring(&t.0);
        let _ = tick(&t.0, 1_000_000, false, false).unwrap();
        let r2 = tick(&t.0, 1_000_000, false, false).unwrap();
        assert_eq!(r2.sensed, 0, "static host facts are deduped — nothing new");
        assert_eq!(
            r2.new_candidates, 0,
            "loops already covered — no new candidates"
        );
    }

    #[test]
    fn connectivity_gated_off_by_default_boundary() {
        let t = Temp::new("gate");
        // no boundary.json -> closed -> connectivity not allowed
        assert!(!connectivity_allowed(&t.0));
    }
}
