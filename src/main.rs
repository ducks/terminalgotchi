mod creature;
mod db;
mod display;
mod growth;
mod quips;
mod sprites;
mod stats;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "terminalgotchi")]
#[command(about = "A terminal companion that grows with you")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Use a specific seed instead of username
    #[arg(long)]
    seed: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show your companion's full stat card
    Card,
    /// Watch your companion idle (animated)
    Watch,
    /// Show XP and growth progress
    Stats,
    /// Record activity manually (usually done via shell hook)
    Feed {
        /// Activity type: debug, commit, chaos, read, hack
        activity: String,
    },
    /// Auto-detect activity from a shell command (for shell hooks)
    FeedCmd {
        /// The shell command that was executed
        #[arg(trailing_var_arg = true)]
        command: Vec<String>,
    },
    /// Print shell hook code for your shell
    Hook {
        /// Shell type: bash, zsh, nu, fish
        shell: Option<String>,
    },
    /// Reset your companion (re-roll from seed)
    Reset,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let seed = cli.seed.unwrap_or_else(|| {
        std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "anon".to_string())
    });

    let db = db::Database::open()?;

    match cli.command {
        None | Some(Commands::Card) => {
            let creature = creature::roll(&seed);
            let xp = db.get_xp(&seed)?;
            display::show_card(&creature, &xp);
        }
        Some(Commands::Watch) => {
            let creature = creature::roll(&seed);
            display::animate(&creature)?;
        }
        Some(Commands::Stats) => {
            let creature = creature::roll(&seed);
            let xp = db.get_xp(&seed)?;
            display::show_stats(&creature, &xp);
        }
        Some(Commands::Feed { activity }) => {
            let stat = growth::activity_to_stat(&activity);
            match stat {
                Some(stat_name) => {
                    let creature = creature::roll(&seed);
                    let amount = growth::xp_for_activity(&activity);
                    db.add_xp(&seed, &stat_name, amount)?;
                    let q = quips::quip_for_stat(stat_name);
                    println!("  {} \x1b[2m\"{}\"\x1b[0m", creature.face(), q);
                    println!("  +{} {} XP", amount, stat_name);
                }
                None => {
                    eprintln!("Unknown activity: {}", activity);
                    eprintln!("Valid: debug, commit, chaos, read, hack");
                }
            }
        }
        Some(Commands::FeedCmd { command }) => {
            let cmd_str = command.join(" ");
            if let Some(activity) = growth::command_to_activity(&cmd_str) {
                if let Some(stat) = growth::activity_to_stat(activity) {
                    let creature = creature::roll(&seed);
                    let amount = growth::xp_for_activity(activity);
                    db.add_xp(&seed, &stat, amount)?;
                    // Show a brief quip -- shell hooks can pipe to /dev/null if noisy
                    let q = quips::quip_for_stat(stat);
                    println!("  {} \x1b[2m\"{}\"\x1b[0m  +{} {}", creature.face(), q, amount, stat);
                }
            }
        }
        Some(Commands::Hook { shell }) => {
            let shell_name = shell.unwrap_or_else(|| {
                std::env::var("SHELL")
                    .unwrap_or_default()
                    .rsplit('/')
                    .next()
                    .unwrap_or("bash")
                    .to_string()
            });
            print_hook(&shell_name);
        }
        Some(Commands::Reset) => {
            db.reset_xp(&seed)?;
            println!("Companion reset. XP cleared.");
        }
    }

    Ok(())
}

fn print_hook(shell: &str) {
    match shell {
        "bash" => println!(r#"# Add to ~/.bashrc
_terminalgotchi_hook() {{
    local cmd
    cmd=$(HISTTIMEFORMAT= history 1 | sed 's/^ *[0-9]* *//')
    terminalgotchi feed-cmd "$cmd" 2>/dev/null &
}}
PROMPT_COMMAND="_terminalgotchi_hook;${{PROMPT_COMMAND}}"
"#),
        "zsh" => println!(r#"# Add to ~/.zshrc
_terminalgotchi_hook() {{
    local cmd
    cmd=$(fc -ln -1)
    terminalgotchi feed-cmd "$cmd" 2>/dev/null &
}}
precmd_functions+=(_terminalgotchi_hook)
"#),
        "nu" | "nushell" => println!(r#"# Add to ~/.config/nushell/config.nu
$env.config.hooks.pre_prompt = ($env.config.hooks.pre_prompt | append {{||
    let last = (history | last 1 | get command.0)
    terminalgotchi feed-cmd $last
}})
"#),
        "fish" => println!(r#"# Add to ~/.config/fish/config.fish
function _terminalgotchi_hook --on-event fish_postexec
    terminalgotchi feed-cmd "$argv" 2>/dev/null &
end
"#),
        _ => {
            eprintln!("Unknown shell: {}", shell);
            eprintln!("Supported: bash, zsh, nu, fish");
            eprintln!();
            eprintln!("The hook should run after each command:");
            eprintln!("  terminalgotchi feed-cmd \"<last command>\"");
        }
    }
}
