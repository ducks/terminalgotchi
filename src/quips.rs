//! Random quips the companion says when it gets XP.
//! Keeps the creature feeling alive without an LLM call.

use crate::stats::StatName;

/// Quips per stat, picked randomly on XP gain.
const DEBUGGING_QUIPS: &[&str] = &[
    "found another one, huh",
    "squish",
    "was it the semicolon",
    "rubber duck says hi",
    "green bar green bar green bar",
    "all passing? suspicious",
    "the test knows what you did",
    "bug squashed",
    "lgtm (the tests, not you)",
    "one less TODO",
];

const PATIENCE_QUIPS: &[&str] = &[
    "good commit",
    "shipped it",
    "another brick in the wall",
    "future you says thanks",
    "the diff looks clean",
    "one more for the log",
    "documented? wow",
    "steady hands",
    "merge day is a good day",
    "the review can wait til morning",
];

const CHAOS_QUIPS: &[&str] = &[
    "oh no",
    "bold move",
    "force push? really?",
    "yolo accepted",
    "the repo felt that",
    "scorched earth",
    "rm -rf and vibes",
    "that's one way to do it",
    "git reflog remembers",
    "chaos is a ladder",
];

const WISDOM_QUIPS: &[&str] = &[
    "reading is fundamental",
    "ah, the docs",
    "knowledge acquired",
    "rtfm energy",
    "a wise one",
    "man page speedrun",
    "curling the truth",
    "the scroll of knowledge",
    "one does not simply grep",
    "seeking answers",
];

const SNARK_QUIPS: &[&str] = &[
    "sed master",
    "pipe it again",
    "one-liner god",
    "awk yeah",
    "that's disgusting (nice)",
    "regex? in this economy?",
    "the pipeline grows",
    "aliased to perfection",
    "pearl before swine",
    "hack the planet",
];

/// Generic quips when no stat matches.
const GENERIC_QUIPS: &[&str] = &[
    "noted",
    "interesting",
    "hmm",
    "...",
    "*watches*",
    "carry on",
];

/// Pick a random quip for a stat.
pub fn quip_for_stat(stat: StatName) -> &'static str {
    let pool = match stat {
        StatName::Debugging => DEBUGGING_QUIPS,
        StatName::Patience => PATIENCE_QUIPS,
        StatName::Chaos => CHAOS_QUIPS,
        StatName::Wisdom => WISDOM_QUIPS,
        StatName::Snark => SNARK_QUIPS,
    };
    pick_random(pool)
}

/// Pick a generic quip.
pub fn quip_generic() -> &'static str {
    pick_random(GENERIC_QUIPS)
}

fn pick_random<'a>(pool: &[&'a str]) -> &'a str {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as usize;
    pool[nanos % pool.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_stats_have_quips() {
        for stat in &[
            StatName::Debugging,
            StatName::Patience,
            StatName::Chaos,
            StatName::Wisdom,
            StatName::Snark,
        ] {
            let q = quip_for_stat(*stat);
            assert!(!q.is_empty());
        }
    }

    #[test]
    fn generic_quips_exist() {
        let q = quip_generic();
        assert!(!q.is_empty());
    }
}
