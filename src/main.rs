mod creature;
mod db;
mod display;
mod growth;
mod sprites;
mod stats;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "termagotchi")]
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
                    let amount = growth::xp_for_activity(&activity);
                    db.add_xp(&seed, &stat_name, amount)?;
                    println!("  +{} {} XP", amount, stat_name);
                }
                None => {
                    eprintln!("Unknown activity: {}", activity);
                    eprintln!("Valid: debug, commit, chaos, read, hack");
                }
            }
        }
        Some(Commands::Reset) => {
            db.reset_xp(&seed)?;
            println!("Companion reset. XP cleared.");
        }
    }

    Ok(())
}
