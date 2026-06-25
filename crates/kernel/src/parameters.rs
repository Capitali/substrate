//! Co-owned operational parameters — the familiar's tunables (cadence, intervals).
//!
//! Unlike the capability boundary ([`crate::boundary`], human-owned: the familiar may
//! only ever *narrow* it), these are **shared**. At this early stage Ian may adjust them
//! from the Glass to influence the familiar. The design intent (a later brick) is that
//! the familiar may review and *revert* any change it can justify under the Three Laws,
//! trending toward "view, not set" as it matures — so the file carries a `last_set_by`
//! provenance marker now, even though nothing reverts yet.
//!
//! A missing file yields the defaults (never a broken daemon); a malformed file is a
//! hard error so corruption surfaces rather than silently resetting the familiar's pace.

use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;

use crate::store;

pub const PARAMETERS_FILE: &str = "parameters.json";

/// The familiar's adjustable operating parameters. `#[serde(default)]` means an older
/// or partial file still loads — any missing field takes its default.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Parameters {
    /// How often the familiar pauses to form a question + theory (seconds). The hourly
    /// default is why, before this was tunable, it seemed to ask only after a redeploy.
    pub theorize_every_secs: i64,
    /// The active (busy) cadence floor for the daemon (seconds).
    pub interval_floor_secs: u64,
    /// The cadence ceiling reached when the world goes quiet (seconds).
    pub interval_ceiling_secs: u64,
    /// Provenance: who last set these — `"observer"` (Ian, via the Glass) or
    /// `"familiar"` (a future self-adjustment/revert). Informational for now.
    pub last_set_by: String,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            theorize_every_secs: 3600,
            interval_floor_secs: 60,
            interval_ceiling_secs: 960,
            last_set_by: "default".to_string(),
        }
    }
}

impl Parameters {
    /// Load the parameters, falling back to defaults if the file is missing. A malformed
    /// file is an error (corruption surfaces, never silently resets).
    pub fn load(dir: &Path) -> io::Result<Self> {
        Ok(store::load_one(dir, PARAMETERS_FILE)?.unwrap_or_default())
    }

    /// Load, treating *any* error (missing or malformed) as the defaults — for callers
    /// that must never fail to run (the daemon cadence; a read-only viewer).
    pub fn load_or_default(dir: &Path) -> Self {
        Self::load(dir).unwrap_or_default()
    }

    /// Persist the parameters as a single pretty JSON object.
    pub fn save(&self, dir: &Path) -> io::Result<()> {
        std::fs::create_dir_all(dir)?;
        let s = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        std::fs::write(dir.join(PARAMETERS_FILE), s)
    }

    /// Clamp to sane ranges so a hand- or slider-edited file can't wedge the daemon
    /// (e.g. a zero cadence busy-loop, or a ceiling below the floor). Returns self so it
    /// can be chained at the read site: `Parameters::load_or_default(dir).sane()`.
    pub fn sane(mut self) -> Self {
        self.theorize_every_secs = self.theorize_every_secs.clamp(30, 86_400);
        self.interval_floor_secs = self.interval_floor_secs.clamp(5, 3_600);
        self.interval_ceiling_secs = self
            .interval_ceiling_secs
            .clamp(self.interval_floor_secs, 3_600);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    struct Temp(PathBuf);
    impl Temp {
        fn new(t: &str) -> Self {
            let p = std::env::temp_dir().join(format!("familiar_params_test_{t}"));
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

    #[test]
    fn missing_file_is_defaults() {
        let t = Temp::new("missing");
        assert_eq!(Parameters::load(&t.0).unwrap(), Parameters::default());
    }

    #[test]
    fn save_then_load_roundtrips() {
        let t = Temp::new("roundtrip");
        let p = Parameters {
            theorize_every_secs: 120,
            last_set_by: "observer".into(),
            ..Default::default()
        };
        p.save(&t.0).unwrap();
        assert_eq!(Parameters::load(&t.0).unwrap(), p);
    }

    #[test]
    fn sane_clamps_a_wedging_edit() {
        let p = Parameters {
            theorize_every_secs: 0,    // would never theorize-gate sanely
            interval_floor_secs: 0,    // would busy-loop
            interval_ceiling_secs: 10, // below a sane floor
            last_set_by: "observer".into(),
        }
        .sane();
        assert!(p.theorize_every_secs >= 30);
        assert!(p.interval_floor_secs >= 5);
        assert!(p.interval_ceiling_secs >= p.interval_floor_secs);
    }
}
