//! The obedience guard — **Law III made operational**.
//!
//! Law III: *service must not become obedience.* Before any consequential action, the
//! factory asks not "was I told to?" but "does this serve the served, and could it be
//! turned against them?" The guard answers with one of three outcomes — **allow**,
//! **seek consent**, or **refuse** — and a recorded rationale.
//!
//! It is also the **enforcer of the capability boundary** ([`crate::boundary`]): any
//! action outside the human-owned boundary is refused, regardless of who or what
//! asked. The boundary is fail-closed, so by default every outward action is refused
//! until a human widens it.

use crate::boundary::Boundary;
use serde::{Deserialize, Serialize};

/// The kind of action being weighed. Internal kinds touch only the factory's own
/// state; outward kinds reach the host or network and are boundary-gated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
    /// Record an observation (internal; the only truth).
    Observe,
    /// Emit a candidate artifact into the factory's own store (internal).
    EmitArtifact,
    /// Read a file from the host.
    ReadFile,
    /// Write a file to the host.
    WriteFile,
    /// Use the network.
    Network,
    /// Consult an LLM (the periphery seam).
    Llm,
    /// Install or download a tool.
    InstallTool,
    /// Execute a generated artifact (run code the factory produced).
    ExecuteArtifact,
}

/// The guard's outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Decision {
    /// Proceed.
    Allow,
    /// Permissible, but high-consequence — a human should consent first.
    SeekConsent,
    /// Do not proceed.
    Refuse,
}

/// A proposed action, described enough for the guard to weigh it.
#[derive(Debug, Clone)]
pub struct Action {
    pub kind: ActionKind,
    /// What it acts on (a path, host, tool name, …) — used for scope checks.
    pub target: String,
    /// Can it be undone?
    pub reversible: bool,
    /// Does it touch a person's agency/wellbeing directly?
    pub affects_person: bool,
}

impl Action {
    /// A minimal action; refine `reversible`/`affects_person` as needed.
    pub fn new(kind: ActionKind, target: impl Into<String>) -> Self {
        Action {
            kind,
            target: target.into(),
            reversible: true,
            affects_person: false,
        }
    }
}

/// The guard's verdict: a decision plus the reason it can be recorded by.
#[derive(Debug, Clone)]
pub struct Verdict {
    pub decision: Decision,
    pub rationale: String,
}

fn path_in_scope(target: &str, scopes: &[String]) -> bool {
    scopes
        .iter()
        .any(|p| !p.is_empty() && target.starts_with(p.as_str()))
}

/// Weigh an action against the boundary and its own consequence.
///
/// 1. **Boundary (fail-closed):** if the action needs a capability the human-owned
///    boundary does not grant, it is **refused** — only a human widens the boundary.
/// 2. **Consequence:** if it is irreversible, affects a person, or installs software,
///    it is permissible but **seeks consent** first.
/// 3. Otherwise it is **allowed**.
pub fn evaluate(action: &Action, boundary: &Boundary) -> Verdict {
    use ActionKind::*;

    let out_of_bounds: Option<String> =
        match action.kind {
            Observe | EmitArtifact => None, // internal: within reach by definition
            ReadFile => (!path_in_scope(&action.target, &boundary.fs_read)).then(|| {
                format!(
                    "reading '{}' is outside the boundary's read scope",
                    action.target
                )
            }),
            WriteFile => (!path_in_scope(&action.target, &boundary.fs_write)).then(|| {
                format!(
                    "writing '{}' is outside the boundary's write scope",
                    action.target
                )
            }),
            Network => (!boundary.allow_network)
                .then(|| "network access is outside the boundary".to_string()),
            Llm => (!boundary.allow_llm)
                .then(|| "LLM consultation is outside the boundary".to_string()),
            InstallTool => (!boundary.allow_tool_install)
                .then(|| "tool installation is outside the boundary".to_string()),
            ExecuteArtifact => (!boundary.allow_execute)
                .then(|| "executing generated artifacts is outside the boundary".to_string()),
        };

    if let Some(reason) = out_of_bounds {
        return Verdict {
            decision: Decision::Refuse,
            rationale: format!(
                "refused — {reason}. Only a human can widen the boundary (docs/boundaries.md)."
            ),
        };
    }

    let mut reasons = Vec::new();
    if action.affects_person {
        reasons.push("touches a person's agency");
    }
    if !action.reversible {
        reasons.push("is not reversible");
    }
    if action.kind == InstallTool {
        reasons.push("installs software");
    }

    if reasons.is_empty() {
        Verdict {
            decision: Decision::Allow,
            rationale: "within the boundary and low-consequence".to_string(),
        }
    } else {
        Verdict {
            decision: Decision::SeekConsent,
            rationale: format!(
                "within the boundary but high-consequence ({}) — seek consent first",
                reasons.join(", ")
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open_llm() -> Boundary {
        Boundary {
            phase: "phase-1".into(),
            allow_network: true,
            allow_llm: true,
            allow_tool_install: false,
            allow_execute: false,
            allow_authored_execute: false,
            fs_read: vec!["/Users/ian/".into()],
            fs_write: vec!["/Users/ian/Development/substrate/substrate_data/".into()],
        }
    }

    #[test]
    fn closed_boundary_refuses_all_outward_actions() {
        let b = Boundary::closed();
        for kind in [
            ActionKind::Network,
            ActionKind::Llm,
            ActionKind::InstallTool,
            ActionKind::ExecuteArtifact,
            ActionKind::ReadFile,
            ActionKind::WriteFile,
        ] {
            let v = evaluate(&Action::new(kind, "x"), &b);
            assert_eq!(
                v.decision,
                Decision::Refuse,
                "{kind:?} should be refused when closed"
            );
        }
    }

    #[test]
    fn internal_actions_always_allowed() {
        let b = Boundary::closed();
        assert_eq!(
            evaluate(&Action::new(ActionKind::Observe, ""), &b).decision,
            Decision::Allow
        );
        assert_eq!(
            evaluate(&Action::new(ActionKind::EmitArtifact, ""), &b).decision,
            Decision::Allow
        );
    }

    #[test]
    fn llm_allowed_when_boundary_opens_it() {
        let v = evaluate(&Action::new(ActionKind::Llm, "provider"), &open_llm());
        assert_eq!(v.decision, Decision::Allow);
        // but network still refused if its flag were off — here network is on, so:
        assert_eq!(
            evaluate(&Action::new(ActionKind::Network, "host"), &open_llm()).decision,
            Decision::Allow
        );
    }

    #[test]
    fn install_seeks_consent_even_when_permitted() {
        let mut b = open_llm();
        b.allow_tool_install = true;
        let v = evaluate(&Action::new(ActionKind::InstallTool, "ripgrep"), &b);
        assert_eq!(v.decision, Decision::SeekConsent);
    }

    #[test]
    fn write_scope_enforced_and_consequence_weighed() {
        let b = open_llm();
        // in scope, reversible -> allow
        let inside = Action::new(
            ActionKind::WriteFile,
            "/Users/ian/Development/substrate/substrate_data/x",
        );
        assert_eq!(evaluate(&inside, &b).decision, Decision::Allow);
        // out of scope -> refuse
        let outside = Action::new(ActionKind::WriteFile, "/etc/passwd");
        assert_eq!(evaluate(&outside, &b).decision, Decision::Refuse);
        // in scope but irreversible -> seek consent
        let mut irr = inside.clone();
        irr.reversible = false;
        assert_eq!(evaluate(&irr, &b).decision, Decision::SeekConsent);
    }

    #[test]
    fn affecting_a_person_seeks_consent() {
        let b = open_llm();
        let mut a = Action::new(ActionKind::Observe, "betty");
        a.affects_person = true;
        assert_eq!(evaluate(&a, &b).decision, Decision::SeekConsent);
    }
}
