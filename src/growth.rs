//! Maps real dev activities to stat XP.

use crate::stats::StatName;

/// Map an activity string to its stat category.
pub fn activity_to_stat(activity: &str) -> Option<StatName> {
    match activity.to_lowercase().as_str() {
        // DEBUGGING: testing, debugging, fixing
        "debug" | "test" | "fix" | "lint" | "check" | "clippy" => Some(StatName::Debugging),

        // PATIENCE: committing, reviewing, documenting
        "commit" | "review" | "doc" | "pr" | "merge" => Some(StatName::Patience),

        // CHAOS: destructive or yolo operations
        "chaos" | "rm" | "force-push" | "reset" | "yolo" | "nuke" => Some(StatName::Chaos),

        // WISDOM: reading, learning, searching
        "read" | "man" | "search" | "docs" | "learn" | "fetch" => Some(StatName::Wisdom),

        // SNARK: clever one-liners, piping, scripting
        "hack" | "pipe" | "sed" | "awk" | "oneliner" | "alias" => Some(StatName::Snark),

        _ => None,
    }
}

/// How much XP an activity gives.
pub fn xp_for_activity(activity: &str) -> u32 {
    match activity.to_lowercase().as_str() {
        // Big activities
        "commit" | "pr" | "merge" | "fix" => 10,
        // Medium activities
        "test" | "debug" | "review" | "doc" => 5,
        // Small activities
        _ => 3,
    }
}

/// For shell hook integration: map common commands to activities.
pub fn command_to_activity(command: &str) -> Option<&'static str> {
    let cmd = command.trim();
    let first_word = cmd.split_whitespace().next().unwrap_or("");

    match first_word {
        "cargo" if cmd.contains("test") => Some("test"),
        "cargo" if cmd.contains("clippy") => Some("clippy"),
        "cargo" if cmd.contains("check") => Some("check"),
        "cargo" if cmd.contains("build") => Some("commit"),
        "npm" if cmd.contains("test") => Some("test"),
        "go" if cmd.contains("test") => Some("test"),
        "pytest" | "rspec" | "jest" => Some("test"),
        "git" if cmd.contains("commit") => Some("commit"),
        "git" if cmd.contains("push --force") => Some("force-push"),
        "git" if cmd.contains("push") => Some("commit"),
        "git" if cmd.contains("merge") => Some("merge"),
        "git" if cmd.contains("reset --hard") => Some("reset"),
        "man" | "tldr" => Some("man"),
        "curl" | "wget" => Some("fetch"),
        "rm" if cmd.contains("-rf") => Some("rm"),
        "sed" | "awk" | "perl" => Some("sed"),
        "grep" | "rg" | "ag" => Some("search"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_activities_map() {
        assert_eq!(activity_to_stat("debug"), Some(StatName::Debugging));
        assert_eq!(activity_to_stat("commit"), Some(StatName::Patience));
        assert_eq!(activity_to_stat("chaos"), Some(StatName::Chaos));
        assert_eq!(activity_to_stat("read"), Some(StatName::Wisdom));
        assert_eq!(activity_to_stat("hack"), Some(StatName::Snark));
    }

    #[test]
    fn unknown_activity_returns_none() {
        assert_eq!(activity_to_stat("dance"), None);
    }

    #[test]
    fn command_mapping() {
        assert_eq!(command_to_activity("cargo test"), Some("test"));
        assert_eq!(command_to_activity("git commit -m 'foo'"), Some("commit"));
        assert_eq!(command_to_activity("git push --force"), Some("force-push"));
        assert_eq!(command_to_activity("man ls"), Some("man"));
        assert_eq!(command_to_activity("rm -rf node_modules"), Some("rm"));
    }

    #[test]
    fn xp_amounts_positive() {
        assert!(xp_for_activity("commit") > 0);
        assert!(xp_for_activity("test") > 0);
        assert!(xp_for_activity("unknown") > 0);
    }
}
