//! Terminal display for creature cards and animations.

use std::io::{Write, stdout};
use std::time::Duration;

use crate::creature::Creature;
use crate::db::XpTotals;
use crate::sprites;
use crate::stats::{Rarity, StatName, STAT_NAMES};

// Gruvbox colors
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const GRAY: &str = "\x1b[38;2;146;131;116m";
const YELLOW: &str = "\x1b[38;2;250;189;47m";
const GREEN: &str = "\x1b[38;2;184;187;38m";
const BLUE: &str = "\x1b[38;2;131;165;152m";
const PURPLE: &str = "\x1b[38;2;211;134;155m";
const RED: &str = "\x1b[38;2;251;73;52m";
const ORANGE: &str = "\x1b[38;2;254;128;25m";
const AQUA: &str = "\x1b[38;2;142;192;124m";

fn rarity_color(rarity: Rarity) -> &'static str {
    match rarity {
        Rarity::Common => GRAY,
        Rarity::Uncommon => GREEN,
        Rarity::Rare => BLUE,
        Rarity::Epic => PURPLE,
        Rarity::Legendary => YELLOW,
    }
}

fn stat_bar(value: u32, max: u32) -> String {
    let width = 20;
    let filled = ((value as f64 / max as f64) * width as f64) as usize;
    let empty = width - filled;
    format!(
        "{}{}{}{}",
        AQUA,
        "█".repeat(filled),
        DIM,
        "░".repeat(empty),
    )
}

/// Show the full creature card.
pub fn show_card(creature: &Creature, xp: &XpTotals) {
    let color = rarity_color(creature.rarity);
    let frames = sprites::frames(&creature.species, &creature.eye);
    let sprite = &frames[0];

    println!();
    println!("  {}┌────────────────────────────────────┐{}", DIM, RESET);

    // Sprite + info side by side
    let info_lines = vec![
        format!(
            "  {}{}{}  {} {}",
            BOLD, creature.face(), RESET, color, creature.species
        ),
        format!(
            "  {} {}{}{}", color, creature.rarity_stars(), RESET,
            if creature.shiny { format!(" {}✨ shiny{}", YELLOW, RESET) } else { String::new() }
        ),
        format!(
            "  {}rarity:{} {}{}{}", DIM, RESET, color, creature.rarity.name(), RESET
        ),
        if creature.hat != "none" {
            format!("  {}hat:{} {}", DIM, RESET, creature.hat)
        } else {
            String::new()
        },
    ];

    let max_lines = sprite.len().max(info_lines.len());
    for i in 0..max_lines {
        let sprite_part = sprite.get(i).map(|s| s.as_str()).unwrap_or("            ");
        let info_part = info_lines.get(i).map(|s| s.as_str()).unwrap_or("");
        println!("  {}│{} {} {}│{} {}",
            DIM, RESET, sprite_part, DIM, RESET, info_part
        );
    }

    println!("  {}├────────────────────────────────────┤{}", DIM, RESET);

    // Stats
    for &stat in STAT_NAMES {
        let base = creature.base_stats.iter()
            .find(|(s, _)| *s == stat)
            .map(|(_, v)| *v)
            .unwrap_or(0);
        let bonus = xp.get(stat);
        let total = (base + bonus / 10).min(100); // 10 XP = 1 stat point
        let bar = stat_bar(total, 100);

        let stat_color = match stat {
            StatName::Debugging => RED,
            StatName::Patience => BLUE,
            StatName::Chaos => ORANGE,
            StatName::Wisdom => GREEN,
            StatName::Snark => PURPLE,
        };

        println!(
            "  {}│{} {}{:<10}{} {} {}{:>3}{} {}│{}",
            DIM, RESET,
            stat_color, stat.name(), RESET,
            bar,
            BOLD, total, RESET,
            DIM, RESET,
        );

        if bonus > 0 {
            println!(
                "  {}│{}                          {}(+{} xp){} {}│{}",
                DIM, RESET, DIM, bonus, RESET, DIM, RESET,
            );
        }
    }

    println!("  {}└────────────────────────────────────┘{}", DIM, RESET);
    println!();
}

/// Show just XP and growth stats.
pub fn show_stats(creature: &Creature, xp: &XpTotals) {
    let color = rarity_color(creature.rarity);
    println!();
    println!(
        "  {} {} {} {}",
        creature.face(),
        color, creature.species, RESET
    );
    println!("  {}Total activities: {}{}{}", DIM, RESET, xp.total_events, RESET);
    println!();

    for &stat in STAT_NAMES {
        let base = creature.base_stats.iter()
            .find(|(s, _)| *s == stat)
            .map(|(_, v)| *v)
            .unwrap_or(0);
        let bonus_xp = xp.get(stat);
        let bonus_points = bonus_xp / 10;

        println!(
            "  {:<10}  base: {:>2}  xp: {:>4}  (+{} pts)",
            stat.name(), base, bonus_xp, bonus_points
        );
    }
    println!();
}

/// Animate the creature in a loop.
pub fn animate(creature: &Creature) -> anyhow::Result<()> {
    let all_frames = sprites::frames(&creature.species, &creature.eye);
    let color = rarity_color(creature.rarity);
    let hat = sprites::hat_line(&creature.hat);

    // Hide cursor
    print!("\x1b[?25l");
    let _ = stdout().flush();

    // Handle Ctrl+C to restore cursor
    ctrlc_handler();

    let mut frame_idx = 0;
    loop {
        // Clear and draw
        print!("\x1b[H\x1b[2J");

        println!();
        if let Some(hat_str) = hat {
            println!("  {}{}{}", color, hat_str, RESET);
        }

        let frame = &all_frames[frame_idx % all_frames.len()];
        for line in frame {
            println!("  {}{}{}", color, line, RESET);
        }

        println!();
        println!(
            "  {}{}{} {} {} {}{}",
            BOLD, creature.face(), RESET,
            color, creature.species, creature.rarity_stars(), RESET,
        );
        println!();
        println!("  {}Ctrl+C to exit{}", DIM, RESET);

        let _ = stdout().flush();
        frame_idx += 1;
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn ctrlc_handler() {
    ctrlc_restore_cursor();
}

fn ctrlc_restore_cursor() {
    // Set up Ctrl+C handler to restore cursor
    let _ = ctrlc::set_handler(move || {
        print!("\x1b[?25h"); // Show cursor
        let _ = stdout().flush();
        std::process::exit(0);
    });
}
