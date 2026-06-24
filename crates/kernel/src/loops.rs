//! Loop detection — the temporal view of the observation log.
//!
//! A loop is reality repeating itself: the same `actor · action · object` triple
//! recurring. Detection groups observations by that key; each group of two or more
//! is a loop. Faithful port of v1's `loop.c`. Pure: a function of the observations,
//! so persisting it is a rewrite, not an append.

use crate::observation::Observation;
use crate::store;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io;
use std::path::Path;

/// The detected-loops file (rewritten on each detect, never appended).
pub const LOOPS_FILE: &str = "loops.jsonl";

/// A recurring pattern across observations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Loop {
    pub id: String,
    pub name: String,
    pub description: String,
    pub loop_type: String,
    /// Comma-separated observation ids.
    pub observation_ids: String,
    pub observation_count: i32,
    pub first_seen: i64,
    pub last_seen: i64,
    /// Share of all observations belonging to this loop.
    pub recurrence_score: f64,
    pub friction_score: f64,
    pub opportunity_score: f64,
    pub confidence: f64,
}

/// FNV-1a — a small, deterministic, dependency-free hash so a loop keeps a stable
/// id across detection passes (candidate→loop links must stay consistent).
fn fnv1a(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

fn loop_key(o: &Observation) -> String {
    format!("{}|{}|{}", o.actor, o.action, o.object)
}

/// Detect loops: group observations by `actor|action|object`, keep groups of ≥2.
/// Pure — no I/O. Returns loops in stable key order.
pub fn detect(obs: &[Observation]) -> Vec<Loop> {
    // group indices by key, preserving determinism via BTreeMap (sorted by key)
    let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, o) in obs.iter().enumerate() {
        groups.entry(loop_key(o)).or_default().push(i);
    }
    let total = obs.len().max(1) as f64;

    let mut out = Vec::new();
    for (key, idxs) in groups {
        let n = idxs.len();
        if n < 2 {
            continue;
        }
        let first = &obs[idxs[0]];
        let ids: Vec<&str> = idxs.iter().map(|&i| obs[i].id.as_str()).collect();
        let first_seen = idxs.iter().map(|&i| obs[i].ts).min().unwrap_or(0);
        let last_seen = idxs.iter().map(|&i| obs[i].ts).max().unwrap_or(0);
        let confidence = if n >= 5 { 1.0 } else { n as f64 * 0.2 };
        out.push(Loop {
            id: format!("loop-{:012x}", fnv1a(&key) & 0xffff_ffff_ffff),
            name: format!("{}_{}", first.actor, first.action),
            description: format!("Repeated: {key}"),
            loop_type: "recurrence_loop".to_string(),
            observation_ids: ids.join(","),
            observation_count: n as i32,
            first_seen,
            last_seen,
            recurrence_score: n as f64 / total,
            friction_score: 0.5,
            opportunity_score: 0.5,
            confidence,
        });
    }
    out
}

/// The triple a loop was built from, recovered from its `description`
/// ("Repeated: actor|action|object"). `(actor, action, object)`, if parseable.
pub fn loop_triple(lp: &Loop) -> Option<(String, String, String)> {
    let rest = lp.description.strip_prefix("Repeated: ")?;
    let mut parts = rest.splitn(3, '|');
    Some((
        parts.next()?.to_string(),
        parts.next()?.to_string(),
        parts.next()?.to_string(),
    ))
}

/// Overwrite the loops file with exactly this set (detection is a pure rewrite).
pub fn save_all(dir: &Path, loops: &[Loop]) -> io::Result<()> {
    store::rewrite(dir, LOOPS_FILE, loops)
}

/// Load the detected loops.
pub fn load(dir: &Path) -> io::Result<Vec<Loop>> {
    store::load(dir, LOOPS_FILE)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obs(id: &str, actor: &str, action: &str, object: &str, ts: i64) -> Observation {
        let mut o = Observation::new(actor, action, object, "", "test", ts, 1.0);
        o.id = id.to_string();
        o
    }

    #[test]
    fn groups_recurring_triples_only() {
        let data = vec![
            obs("o1", "client", "asks_for", "report", 100),
            obs("o2", "client", "asks_for", "report", 200), // repeat -> loop
            obs("o3", "host", "reports", "cpu", 150),       // singleton -> no loop
        ];
        let loops = detect(&data);
        assert_eq!(loops.len(), 1);
        let lp = &loops[0];
        assert_eq!(lp.observation_count, 2);
        assert_eq!(lp.first_seen, 100);
        assert_eq!(lp.last_seen, 200);
        assert_eq!(lp.name, "client_asks_for");
        assert_eq!(lp.description, "Repeated: client|asks_for|report");
        assert!((lp.recurrence_score - 2.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn stable_id_across_passes() {
        let a = detect(&[obs("o1", "a", "b", "c", 1), obs("o2", "a", "b", "c", 2)]);
        let b = detect(&[obs("x", "a", "b", "c", 9), obs("y", "a", "b", "c", 10)]);
        assert_eq!(a[0].id, b[0].id); // id derives from the triple, not position
    }

    #[test]
    fn triple_recovers_from_description() {
        let loops = detect(&[
            obs("o1", "betty", "asks_for", "digest", 1),
            obs("o2", "betty", "asks_for", "digest", 2),
        ]);
        assert_eq!(
            loop_triple(&loops[0]),
            Some(("betty".into(), "asks_for".into(), "digest".into()))
        );
    }

    #[test]
    fn confidence_ramps_with_count() {
        let mk = |n: usize| {
            let data: Vec<_> = (0..n)
                .map(|i| obs(&format!("o{i}"), "a", "b", "c", i as i64))
                .collect();
            detect(&data)[0].confidence
        };
        assert!((mk(2) - 0.4).abs() < 1e-9);
        assert_eq!(mk(5), 1.0);
    }
}
