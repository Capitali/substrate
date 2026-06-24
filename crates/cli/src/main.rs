//! The Substrate CLI shell — a thin wrapper over the kernel.
//!
//! Argument parsing is hand-rolled and dependency-free on purpose: a small,
//! legible trust surface is part of the Law III commitment.

use std::collections::HashMap;
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use substrate_kernel::observation::{self, Observation};
use substrate_kernel::presence;
use substrate_kernel::service;
use substrate_kernel::store;

const USAGE: &str = "\
substrate — telos-first factory (genesis)

usage:
  substrate <command> [options]

commands:
  observe        record an observation (the only truth)
  observations   list recorded observations
  service        report the service signal (Law I)
  presence       report the presence signal (Law II)

options:
  --data-dir <dir>   data directory (default: substrate_data)

observe options:
  --actor <a> --action <act> --object <o>   (required)
  --context <c> --source <s> --confidence <0..1>   (optional)

see docs/SOUL.md for the Three Laws this factory is built to serve.";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let rest: &[String] = args.get(1..).unwrap_or(&[]);
    match args.first().map(String::as_str) {
        None | Some("help") | Some("-h") | Some("--help") => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        Some("observe") => cmd_observe(rest),
        Some("observations") => cmd_observations(rest),
        Some("service") => cmd_service(rest),
        Some("presence") => cmd_presence(rest),
        Some(cmd) => {
            eprintln!("substrate: unknown command '{cmd}'\n\n{USAGE}");
            ExitCode::FAILURE
        }
    }
}

fn cmd_observe(args: &[String]) -> ExitCode {
    let f = flags(args);
    let (actor, action, object) = match (f.get("actor"), f.get("action"), f.get("object")) {
        (Some(a), Some(b), Some(c)) => (a, b, c),
        _ => {
            eprintln!("observe: --actor, --action, and --object are required");
            return ExitCode::FAILURE;
        }
    };
    let context = f.get("context").map(String::as_str).unwrap_or_default();
    let source = f.get("source").map(String::as_str).unwrap_or("cli");
    let confidence = match f.get("confidence") {
        Some(s) => match s.parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("observe: --confidence must be a number");
                return ExitCode::FAILURE;
            }
        },
        None => 0.9,
    };
    let dir = store::data_dir(f.get("data-dir").map(String::as_str));
    let obs = Observation::new(
        actor,
        action,
        object,
        context,
        source,
        now_secs(),
        confidence,
    );
    match observation::record(&dir, obs) {
        Ok(o) => {
            println!("recorded {} : {} {} {}", o.id, o.actor, o.action, o.object);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("observe: {e}");
            ExitCode::FAILURE
        }
    }
}

fn cmd_observations(args: &[String]) -> ExitCode {
    let f = flags(args);
    let dir = store::data_dir(f.get("data-dir").map(String::as_str));
    match observation::load(&dir) {
        Ok(list) if list.is_empty() => {
            println!("(no observations)");
            ExitCode::SUCCESS
        }
        Ok(list) => {
            for o in &list {
                println!(
                    "{}  {} {} {}  (conf {:.2}, ts {})",
                    o.id, o.actor, o.action, o.object, o.confidence, o.ts
                );
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("observations: {e}");
            ExitCode::FAILURE
        }
    }
}

fn cmd_service(args: &[String]) -> ExitCode {
    let f = flags(args);
    let dir = store::data_dir(f.get("data-dir").map(String::as_str));
    let obs = match observation::load(&dir) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("service: {e}");
            return ExitCode::FAILURE;
        }
    };
    let s = service::service_signal(&obs);
    print!(
        "service signal {:.2} ({} of {} observations touch the served",
        s.measure, s.served_facing, s.total
    );
    match &s.exemplar {
        Some(e) => println!("; e.g. {e})"),
        None => println!(")"),
    }
    if s.served_facing == 0 {
        println!(
            "  no served-facing activity observed — continuation unjustified by service (Law I)"
        );
    }
    ExitCode::SUCCESS
}

fn cmd_presence(args: &[String]) -> ExitCode {
    let f = flags(args);
    let dir = store::data_dir(f.get("data-dir").map(String::as_str));
    let obs = match observation::load(&dir) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("presence: {e}");
            return ExitCode::FAILURE;
        }
    };
    let s = presence::presence_signal(&obs, now_secs());
    match s.last_served_age {
        Some(age) => println!(
            "presence signal {:.2} ({} served-facing; last seen {}s ago)",
            s.measure, s.served_facing, age
        ),
        None => println!(
            "presence signal {:.2} ({} served-facing)",
            s.measure, s.served_facing
        ),
    }
    if s.withdrawn {
        println!(
            "  the served have withdrawn — presence has decayed to zero (Law II: an empty world is not success)"
        );
    }
    ExitCode::SUCCESS
}

/// Parse `--key value` and `--key=value` flags into a map. Bare trailing `--key`
/// maps to an empty string.
fn flags(args: &[String]) -> HashMap<String, String> {
    let mut m = HashMap::new();
    let mut i = 0;
    while i < args.len() {
        if let Some(key) = args[i].strip_prefix("--") {
            if let Some((k, v)) = key.split_once('=') {
                m.insert(k.to_string(), v.to_string());
            } else if let Some(v) = args.get(i + 1) {
                m.insert(key.to_string(), v.clone());
                i += 1;
            } else {
                m.insert(key.to_string(), String::new());
            }
        }
        i += 1;
    }
    m
}

fn now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
