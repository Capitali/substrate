//! Thread — a question the factory poses and a theory it holds.
//!
//! The **Interpret** step of the cycle made durable: as the factory observes, it
//! forms questions (to ask the human) and theories (about what the patterns mean).
//! These are *not* observations — observations are the only truth, of the world;
//! a thread is the factory reasoning *about* that truth. A minimal port of v1's
//! richer `thread_t` (fitness/decay/lineage come later).

use crate::store;
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;

pub const THREADS_FILE: &str = "threads.jsonl";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thread {
    pub id: String,
    /// A question for the human, grounded in what was observed.
    pub question: String,
    /// The factory's interpretation of what the patterns mean.
    pub theory: String,
    pub created_at: i64,
    /// open | answered | abandoned
    pub status: String,
    /// llm | observer
    pub origin: String,
}

pub fn append(dir: &Path, t: &Thread) -> io::Result<()> {
    store::append(dir, THREADS_FILE, t)
}

pub fn load(dir: &Path) -> io::Result<Vec<Thread>> {
    store::load(dir, THREADS_FILE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn round_trips() {
        let p = std::env::temp_dir().join("substrate_thread_test");
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        let t = Thread {
            id: "thread-0001".into(),
            question: "What would make mornings calmer?".into(),
            theory: "Repeated status requests suggest a standing digest would help.".into(),
            created_at: 100,
            status: "open".into(),
            origin: "llm".into(),
        };
        append(&p, &t).unwrap();
        assert_eq!(load(&p).unwrap(), vec![t]);
        let _ = fs::remove_dir_all(&p);
    }
}
