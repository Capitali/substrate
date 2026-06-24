//! The Substrate CLI shell — a thin wrapper over the kernel.
//!
//! Argument parsing is hand-rolled and dependency-free on purpose: a small,
//! legible trust surface is part of the Law III commitment.

use std::process::ExitCode;

const USAGE: &str = "\
substrate — telos-first factory (genesis)

usage:
  substrate <command> [options]

commands:
  (none yet — the observation spine and the law-signals arrive next)

see docs/SOUL.md for the Three Laws this factory is built to serve.";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None | Some("help") | Some("-h") | Some("--help") => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        Some(cmd) => {
            eprintln!("substrate: unknown command '{cmd}'\n\n{USAGE}");
            ExitCode::FAILURE
        }
    }
}
