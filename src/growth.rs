//! Maps real dev activities to stat XP.
//!
//! Command mappings are defined as data in COMMAND_RULES.
//! To add support for a new language or tool, just add entries to the table.

use crate::stats::StatName;

/// A rule mapping a shell command pattern to an activity.
struct CommandRule {
    /// First word of the command (or "*" for any)
    prefix: &'static str,
    /// Substring the command must contain (or "" for prefix-only match)
    contains: &'static str,
    /// The activity this maps to
    activity: &'static str,
}

/// An activity definition: name, stat, and XP value.
struct ActivityDef {
    name: &'static str,
    stat: StatName,
    xp: u32,
}

/// Activity definitions. To add a new activity, add it here.
const ACTIVITIES: &[ActivityDef] = &[
    // DEBUGGING
    ActivityDef { name: "test",    stat: StatName::Debugging, xp: 5 },
    ActivityDef { name: "debug",   stat: StatName::Debugging, xp: 5 },
    ActivityDef { name: "fix",     stat: StatName::Debugging, xp: 10 },
    ActivityDef { name: "lint",    stat: StatName::Debugging, xp: 3 },
    ActivityDef { name: "check",   stat: StatName::Debugging, xp: 3 },
    ActivityDef { name: "clippy",  stat: StatName::Debugging, xp: 3 },
    // PATIENCE
    ActivityDef { name: "commit",     stat: StatName::Patience, xp: 10 },
    ActivityDef { name: "review",     stat: StatName::Patience, xp: 5 },
    ActivityDef { name: "doc",        stat: StatName::Patience, xp: 5 },
    ActivityDef { name: "pr",         stat: StatName::Patience, xp: 10 },
    ActivityDef { name: "merge",      stat: StatName::Patience, xp: 10 },
    // CHAOS
    ActivityDef { name: "chaos",      stat: StatName::Chaos, xp: 3 },
    ActivityDef { name: "rm",         stat: StatName::Chaos, xp: 5 },
    ActivityDef { name: "force-push", stat: StatName::Chaos, xp: 10 },
    ActivityDef { name: "reset",      stat: StatName::Chaos, xp: 5 },
    ActivityDef { name: "yolo",       stat: StatName::Chaos, xp: 3 },
    ActivityDef { name: "nuke",       stat: StatName::Chaos, xp: 10 },
    // WISDOM
    ActivityDef { name: "read",   stat: StatName::Wisdom, xp: 3 },
    ActivityDef { name: "man",    stat: StatName::Wisdom, xp: 3 },
    ActivityDef { name: "search", stat: StatName::Wisdom, xp: 3 },
    ActivityDef { name: "docs",   stat: StatName::Wisdom, xp: 3 },
    ActivityDef { name: "learn",  stat: StatName::Wisdom, xp: 5 },
    ActivityDef { name: "fetch",  stat: StatName::Wisdom, xp: 3 },
    // SNARK
    ActivityDef { name: "hack",     stat: StatName::Snark, xp: 3 },
    ActivityDef { name: "pipe",     stat: StatName::Snark, xp: 3 },
    ActivityDef { name: "sed",      stat: StatName::Snark, xp: 3 },
    ActivityDef { name: "awk",      stat: StatName::Snark, xp: 3 },
    ActivityDef { name: "oneliner", stat: StatName::Snark, xp: 5 },
    ActivityDef { name: "alias",    stat: StatName::Snark, xp: 3 },
];

/// Command-to-activity rules. Checked in order, first match wins.
/// To add a new language/tool, just add rows here.
const COMMAND_RULES: &[CommandRule] = &[
    // Rust
    CommandRule { prefix: "cargo",   contains: "test",      activity: "test" },
    CommandRule { prefix: "cargo",   contains: "clippy",    activity: "clippy" },
    CommandRule { prefix: "cargo",   contains: "check",     activity: "check" },
    CommandRule { prefix: "cargo",   contains: "build",     activity: "commit" },
    CommandRule { prefix: "cargo",   contains: "doc",       activity: "doc" },
    // JavaScript / TypeScript
    CommandRule { prefix: "npm",     contains: "test",      activity: "test" },
    CommandRule { prefix: "npm",     contains: "run lint",  activity: "lint" },
    CommandRule { prefix: "npx",     contains: "jest",      activity: "test" },
    CommandRule { prefix: "yarn",    contains: "test",      activity: "test" },
    CommandRule { prefix: "bun",     contains: "test",      activity: "test" },
    CommandRule { prefix: "jest",    contains: "",          activity: "test" },
    CommandRule { prefix: "vitest",  contains: "",          activity: "test" },
    CommandRule { prefix: "eslint",  contains: "",          activity: "lint" },
    // Python
    CommandRule { prefix: "pytest",  contains: "",          activity: "test" },
    CommandRule { prefix: "python",  contains: "-m pytest", activity: "test" },
    CommandRule { prefix: "python",  contains: "-m unittest", activity: "test" },
    CommandRule { prefix: "mypy",    contains: "",          activity: "check" },
    CommandRule { prefix: "ruff",    contains: "",          activity: "lint" },
    CommandRule { prefix: "flake8",  contains: "",          activity: "lint" },
    CommandRule { prefix: "pip",     contains: "install",   activity: "commit" },
    // Ruby
    CommandRule { prefix: "rspec",   contains: "",          activity: "test" },
    CommandRule { prefix: "bundle",  contains: "exec rspec", activity: "test" },
    CommandRule { prefix: "rake",    contains: "test",      activity: "test" },
    CommandRule { prefix: "rubocop", contains: "",          activity: "lint" },
    // Go
    CommandRule { prefix: "go",      contains: "test",      activity: "test" },
    CommandRule { prefix: "go",      contains: "build",     activity: "commit" },
    CommandRule { prefix: "go",      contains: "vet",       activity: "lint" },
    CommandRule { prefix: "golangci-lint", contains: "",    activity: "lint" },
    // Nix
    CommandRule { prefix: "nix-build",       contains: "",  activity: "commit" },
    CommandRule { prefix: "nixos-rebuild",    contains: "",  activity: "chaos" },
    CommandRule { prefix: "nix-shell",        contains: "",  activity: "hack" },
    // Git
    CommandRule { prefix: "git",     contains: "commit",       activity: "commit" },
    CommandRule { prefix: "git",     contains: "push --force",  activity: "force-push" },
    CommandRule { prefix: "git",     contains: "push -f",       activity: "force-push" },
    CommandRule { prefix: "git",     contains: "push",          activity: "commit" },
    CommandRule { prefix: "git",     contains: "merge",         activity: "merge" },
    CommandRule { prefix: "git",     contains: "reset --hard",  activity: "reset" },
    CommandRule { prefix: "git",     contains: "rebase",        activity: "chaos" },
    CommandRule { prefix: "git",     contains: "log",           activity: "read" },
    CommandRule { prefix: "git",     contains: "diff",          activity: "read" },
    // Docker / containers
    CommandRule { prefix: "docker",  contains: "build",    activity: "commit" },
    CommandRule { prefix: "docker",  contains: "rm",       activity: "rm" },
    CommandRule { prefix: "docker",  contains: "prune",    activity: "nuke" },
    // Docs / reading
    CommandRule { prefix: "man",     contains: "",         activity: "man" },
    CommandRule { prefix: "tldr",    contains: "",         activity: "man" },
    CommandRule { prefix: "curl",    contains: "",         activity: "fetch" },
    CommandRule { prefix: "wget",    contains: "",         activity: "fetch" },
    // Destructive
    CommandRule { prefix: "rm",      contains: "-rf",      activity: "rm" },
    CommandRule { prefix: "rm",      contains: "-r",       activity: "rm" },
    // Text wrangling
    CommandRule { prefix: "sed",     contains: "",         activity: "sed" },
    CommandRule { prefix: "awk",     contains: "",         activity: "awk" },
    CommandRule { prefix: "perl",    contains: "-e",       activity: "sed" },
    CommandRule { prefix: "jq",      contains: "",         activity: "hack" },
    // Search
    CommandRule { prefix: "grep",    contains: "",         activity: "search" },
    CommandRule { prefix: "rg",      contains: "",         activity: "search" },
    CommandRule { prefix: "ag",      contains: "",         activity: "search" },
    CommandRule { prefix: "find",    contains: "",         activity: "search" },
    // Make
    CommandRule { prefix: "make",    contains: "test",     activity: "test" },
    CommandRule { prefix: "make",    contains: "build",    activity: "commit" },
    CommandRule { prefix: "make",    contains: "clean",    activity: "rm" },
    CommandRule { prefix: "make",    contains: "",         activity: "commit" },
];

fn find_activity(name: &str) -> Option<&'static ActivityDef> {
    ACTIVITIES.iter().find(|a| a.name == name)
}

/// Map an activity string to its stat category.
pub fn activity_to_stat(activity: &str) -> Option<StatName> {
    find_activity(&activity.to_lowercase()).map(|a| a.stat)
}

/// How much XP an activity gives.
pub fn xp_for_activity(activity: &str) -> u32 {
    find_activity(&activity.to_lowercase())
        .map(|a| a.xp)
        .unwrap_or(3)
}

/// Map a shell command to an activity via the rules table.
pub fn command_to_activity(command: &str) -> Option<&'static str> {
    let cmd = command.trim();
    let first_word = cmd.split_whitespace().next().unwrap_or("");

    for rule in COMMAND_RULES {
        if first_word == rule.prefix || rule.prefix == "*" {
            if rule.contains.is_empty() || cmd.contains(rule.contains) {
                return Some(rule.activity);
            }
        }
    }

    None
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
    fn xp_amounts_positive() {
        assert!(xp_for_activity("commit") > 0);
        assert!(xp_for_activity("test") > 0);
        assert!(xp_for_activity("unknown") > 0);
    }

    // Rust
    #[test]
    fn rust_commands() {
        assert_eq!(command_to_activity("cargo test"), Some("test"));
        assert_eq!(command_to_activity("cargo clippy"), Some("clippy"));
        assert_eq!(command_to_activity("cargo build --release"), Some("commit"));
    }

    // Git
    #[test]
    fn git_commands() {
        assert_eq!(command_to_activity("git commit -m 'foo'"), Some("commit"));
        assert_eq!(command_to_activity("git push --force"), Some("force-push"));
        assert_eq!(command_to_activity("git push -f origin main"), Some("force-push"));
        assert_eq!(command_to_activity("git push"), Some("commit"));
        assert_eq!(command_to_activity("git merge feature"), Some("merge"));
        assert_eq!(command_to_activity("git reset --hard HEAD~1"), Some("reset"));
    }

    // Python
    #[test]
    fn python_commands() {
        assert_eq!(command_to_activity("pytest"), Some("test"));
        assert_eq!(command_to_activity("python -m pytest"), Some("test"));
        assert_eq!(command_to_activity("mypy src/"), Some("check"));
        assert_eq!(command_to_activity("ruff check ."), Some("lint"));
    }

    // Ruby
    #[test]
    fn ruby_commands() {
        assert_eq!(command_to_activity("rspec"), Some("test"));
        assert_eq!(command_to_activity("bundle exec rspec"), Some("test"));
        assert_eq!(command_to_activity("rubocop"), Some("lint"));
    }

    // Go
    #[test]
    fn go_commands() {
        assert_eq!(command_to_activity("go test ./..."), Some("test"));
        assert_eq!(command_to_activity("go build"), Some("commit"));
    }

    // JS/TS
    #[test]
    fn js_commands() {
        assert_eq!(command_to_activity("npm test"), Some("test"));
        assert_eq!(command_to_activity("jest"), Some("test"));
        assert_eq!(command_to_activity("eslint src/"), Some("lint"));
    }

    // Nix
    #[test]
    fn nix_commands() {
        assert_eq!(command_to_activity("nixos-rebuild switch"), Some("chaos"));
        assert_eq!(command_to_activity("nix-build"), Some("commit"));
    }

    // Destructive
    #[test]
    fn destructive_commands() {
        assert_eq!(command_to_activity("rm -rf node_modules"), Some("rm"));
        assert_eq!(command_to_activity("docker prune"), Some("nuke"));
    }

    // Text / search
    #[test]
    fn text_commands() {
        assert_eq!(command_to_activity("sed -i 's/foo/bar/' file"), Some("sed"));
        assert_eq!(command_to_activity("man ls"), Some("man"));
        assert_eq!(command_to_activity("rg pattern"), Some("search"));
        assert_eq!(command_to_activity("jq '.data'"), Some("hack"));
    }

    // Unknown
    #[test]
    fn unknown_command() {
        assert_eq!(command_to_activity("ls -la"), None);
        assert_eq!(command_to_activity("echo hello"), None);
    }
}
