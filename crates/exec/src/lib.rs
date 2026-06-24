//! A small, dependency-free, resource-limited script runner.
//!
//! Runs a shell script under a CPU limit (`ulimit -t`) and a wall-clock timeout
//! (enforced in-process by polling and killing), capturing capped output and
//! measuring cost. Pure `std` — no `unsafe`, no crates. Executing generated code is a
//! Law III matter; this runner is only ever invoked behind the `allow_execute`
//! boundary gate (see the obedience guard), and it bounds what a runaway can do.

use std::io;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

/// Resource limits for a run.
#[derive(Debug, Clone)]
pub struct Limits {
    /// CPU seconds (`ulimit -t`).
    pub cpu_secs: u64,
    /// Wall-clock seconds before the run is killed.
    pub wall_secs: u64,
    /// Max bytes of output retained (and used for the cost measure).
    pub output_cap: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            cpu_secs: 5,
            wall_secs: 10,
            output_cap: 8192,
        }
    }
}

/// The measured outcome of a run.
#[derive(Debug, Clone, PartialEq)]
pub struct RunResult {
    /// The script exited 0.
    pub exit_ok: bool,
    /// The wall-clock timeout was hit and the process was killed.
    pub timed_out: bool,
    /// Wall time in milliseconds.
    pub wall_ms: u128,
    /// Bytes of output produced (capped at `output_cap`).
    pub output_bytes: usize,
    /// Captured output (stdout+stderr), truncated to `output_cap`.
    pub output: String,
}

/// Run `script_path` under `limits`. Combined stdout/stderr go to a sibling `.out`
/// file, which is read back (capped). Wall timeout is enforced by polling `try_wait`
/// and killing on overrun.
pub fn run_script(script_path: &Path, limits: &Limits) -> io::Result<RunResult> {
    let out_path = script_path.with_extension("out");
    let cmd = format!(
        "ulimit -t {cpu}; sh '{script}' > '{out}' 2>&1",
        cpu = limits.cpu_secs,
        script = script_path.display(),
        out = out_path.display(),
    );

    let start = Instant::now();
    let mut child = Command::new("sh").arg("-c").arg(&cmd).spawn()?;

    let mut timed_out = false;
    let mut exit_ok = false;
    loop {
        match child.try_wait()? {
            Some(status) => {
                exit_ok = status.success();
                break;
            }
            None => {
                if start.elapsed() >= Duration::from_secs(limits.wall_secs) {
                    let _ = child.kill();
                    let _ = child.wait();
                    timed_out = true;
                    break;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
        }
    }
    let wall_ms = start.elapsed().as_millis();

    let raw = std::fs::read_to_string(&out_path).unwrap_or_default();
    let _ = std::fs::remove_file(&out_path);
    let output: String = raw.chars().take(limits.output_cap).collect();
    let output_bytes = output.len();

    Ok(RunResult {
        exit_ok,
        timed_out,
        wall_ms,
        output_bytes,
        output,
    })
}

/// Measured cost in [0,1]: a blend of wall time and output size against the limits;
/// a run that hit the wall is maximally costly. This is what folds into the trial's
/// complexity (Soul Rule 9 → Law I: a cheaper artifact outranks an expensive equal).
pub fn cost(r: &RunResult, limits: &Limits) -> f64 {
    if r.timed_out {
        return 1.0;
    }
    let wall = r.wall_ms as f64 / (limits.wall_secs.max(1) as f64 * 1000.0);
    let out = r.output_bytes as f64 / limits.output_cap.max(1) as f64;
    (0.5 * wall + 0.5 * out).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn tmp(tag: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("substrate_exec_test_{tag}"));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn runs_a_clean_script_cheaply() {
        let d = tmp("clean");
        let s = d.join("a.sh");
        fs::write(&s, "echo hello from substrate\n").unwrap();
        let r = run_script(&s, &Limits::default()).unwrap();
        assert!(r.exit_ok && !r.timed_out);
        assert!(r.output.contains("hello from substrate"));
        assert!(cost(&r, &Limits::default()) < 0.5);
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn nonzero_exit_is_recorded() {
        let d = tmp("fail");
        let s = d.join("b.sh");
        fs::write(&s, "exit 3\n").unwrap();
        let r = run_script(&s, &Limits::default()).unwrap();
        assert!(!r.exit_ok && !r.timed_out);
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn wall_timeout_is_enforced_and_maximally_costly() {
        let d = tmp("timeout");
        let s = d.join("c.sh");
        fs::write(&s, "sleep 3\n").unwrap();
        let limits = Limits {
            cpu_secs: 5,
            wall_secs: 1,
            output_cap: 8192,
        };
        let r = run_script(&s, &limits).unwrap();
        assert!(r.timed_out && !r.exit_ok);
        assert_eq!(cost(&r, &limits), 1.0);
        let _ = fs::remove_dir_all(&d);
    }
}
