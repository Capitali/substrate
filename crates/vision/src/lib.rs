//! Vision — the familiar's eye.
//!
//! This brick is environmental **perception of cameras**, the precondition of (consensual)
//! sight. *Discovery* — which cameras exist — is always permitted, exactly as perceiving
//! the host's interfaces is: the boundary governs *reach*, not *perception*. *Watching*
//! through a camera (capturing frames) is the most invasive reach and is boundary-gated,
//! fail-closed (`allow_camera`, the obedience guard); it never happens here — this module
//! only enumerates. **A camera being available is not authorization to watch it.**
//!
//! Read-only and dependency-light: enumeration shells out to system tools (macOS
//! `system_profiler`, Linux `/dev/video*`), keeping the trust surface small (Law III). The
//! gated capture and recognition that turn frames into observations arrive in later bricks.

#![forbid(unsafe_code)]

use familiar_kernel::observation::Observation;

const SENSE_CONF: f64 = 0.9;
const SOURCE: &str = "sensor";

/// Discover the cameras present in the environment — perception, always permitted. Each
/// becomes an observation `host has camera:<name>`. *Watching* one is a separate, gated act
/// (never done here). Best-effort and read-only: records what it can see, skips the rest.
pub fn discover(now: i64) -> Vec<Observation> {
    names()
        .into_iter()
        .map(|n| {
            Observation::new(
                "host",
                "has",
                format!("camera:{n}"),
                String::new(),
                SOURCE,
                now,
                SENSE_CONF,
            )
        })
        .collect()
}

/// The camera names on this host (macOS `system_profiler`, else Linux `/dev/video*`).
fn names() -> Vec<String> {
    #[cfg(target_os = "macos")]
    {
        if let Some(out) = run("system_profiler", &["SPCameraDataType"]) {
            return parse_system_profiler(&out);
        }
        Vec::new()
    }
    #[cfg(not(target_os = "macos"))]
    {
        match std::fs::read_dir("/dev") {
            Ok(rd) => {
                let mut v: Vec<String> = rd
                    .flatten()
                    .filter_map(|e| e.file_name().into_string().ok())
                    .filter(|n| n.starts_with("video"))
                    .collect();
                v.sort();
                v
            }
            Err(_) => Vec::new(),
        }
    }
}

#[cfg(target_os = "macos")]
fn run(cmd: &str, args: &[&str]) -> Option<String> {
    let out = std::process::Command::new(cmd).args(args).output().ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Parse `system_profiler SPCameraDataType`: camera entries are the 4-space-indented lines
/// ending in `:` — the 0-indent `Camera:` header and the 6-space `Model ID:` / `Unique ID:`
/// sub-fields are not cameras.
pub fn parse_system_profiler(s: &str) -> Vec<String> {
    s.lines()
        .filter(|l| {
            l.starts_with("    ") && !l.starts_with("      ") && l.trim_end().ends_with(':')
        })
        .map(|l| l.trim().trim_end_matches(':').to_string())
        .filter(|n| !n.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_camera_names_not_headers_or_subfields() {
        let sample = "Camera:\n\n    FaceTime HD Camera (Built-in):\n\n      Model ID: UVC Camera\n      Unique ID: 0x8020\n\n    Aphelion Camera:\n\n      Model ID: iPhone17,2\n";
        let names = parse_system_profiler(sample);
        assert_eq!(
            names,
            vec!["FaceTime HD Camera (Built-in)", "Aphelion Camera"]
        );
    }

    #[test]
    fn discover_is_best_effort_perception_and_never_panics() {
        // perception only — returns whatever it can enumerate (possibly empty), never errors,
        // and never captures a frame (watching is gated elsewhere).
        let cams = discover(1);
        for c in &cams {
            assert!(c.object.starts_with("camera:"));
            assert_eq!(c.action, "has");
        }
    }
}
