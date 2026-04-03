//! Deterministic creature generation from a seed string.
//! Ported from Claude Code's buddy/companion.ts.

use crate::stats::{Rarity, StatName, RARITIES, RARITY_WEIGHTS, STAT_NAMES};

/// All possible species.
pub const SPECIES: &[&str] = &[
    "duck", "goose", "blob", "cat", "dragon", "octopus", "owl", "penguin",
    "turtle", "snail", "ghost", "axolotl", "capybara", "cactus", "robot",
    "rabbit", "mushroom", "chonk",
];

/// Eye styles.
pub const EYES: &[&str] = &["·", "✦", "×", "◉", "@", "°"];

/// Hat types.
pub const HATS: &[&str] = &[
    "none", "crown", "tophat", "propeller", "halo", "wizard", "beanie", "tinyduck",
];

/// A fully rolled creature.
#[derive(Debug, Clone)]
pub struct Creature {
    pub species: String,
    pub eye: String,
    pub hat: String,
    pub rarity: Rarity,
    pub shiny: bool,
    pub base_stats: Vec<(StatName, u32)>,
    pub seed: String,
}

impl Creature {
    pub fn face(&self) -> String {
        let e = &self.eye;
        match self.species.as_str() {
            "duck" | "goose" => format!("({}> ", e),
            "blob" => format!("({}{})", e, e),
            "cat" => format!("={}w{}=", e, e),
            "dragon" => format!("<{}~{}>", e, e),
            "octopus" => format!("~({}{})~", e, e),
            "owl" => format!("({})({})", e, e),
            "penguin" => format!("({}>)", e),
            "turtle" => format!("[{}_{}]", e, e),
            "snail" => format!("{}(@)", e),
            "ghost" => format!("/{}{}\\", e, e),
            "axolotl" => format!("}}{}.{}{{", e, e),
            "capybara" => format!("({}oo{})", e, e),
            "cactus" => format!("|{}  {}|", e, e),
            "robot" => format!("[{}{}]", e, e),
            "rabbit" => format!("({}..{})", e, e),
            "mushroom" => format!("|{}  {}|", e, e),
            "chonk" => format!("({}.{})", e, e),
            _ => format!("({}{})", e, e),
        }
    }

    pub fn rarity_stars(&self) -> &str {
        match self.rarity {
            Rarity::Common => "★",
            Rarity::Uncommon => "★★",
            Rarity::Rare => "★★★",
            Rarity::Epic => "★★★★",
            Rarity::Legendary => "★★★★★",
        }
    }
}

// Mulberry32 -- tiny seeded PRNG
fn mulberry32(mut seed: u32) -> impl FnMut() -> f64 {
    move || {
        seed = seed.wrapping_add(0x6d2b79f5);
        let mut t = seed;
        t = (t ^ (t >> 15)).wrapping_mul(1 | t);
        t = (t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t))) ^ t;
        ((t ^ (t >> 14)) as f64) / 4294967296.0
    }
}

fn hash_string(s: &str) -> u32 {
    let mut h: u32 = 2166136261;
    for byte in s.bytes() {
        h ^= byte as u32;
        h = h.wrapping_mul(16777619);
    }
    h
}

fn pick<'a>(rng: &mut impl FnMut() -> f64, arr: &'a [&str]) -> &'a str {
    arr[(rng() * arr.len() as f64) as usize]
}

fn roll_rarity(rng: &mut impl FnMut() -> f64) -> Rarity {
    let total: u32 = RARITY_WEIGHTS.iter().map(|(_, w)| w).sum();
    let mut roll = rng() * total as f64;
    for (rarity, weight) in RARITY_WEIGHTS {
        roll -= *weight as f64;
        if roll < 0.0 {
            return *rarity;
        }
    }
    Rarity::Common
}

fn rarity_floor(rarity: Rarity) -> u32 {
    match rarity {
        Rarity::Common => 5,
        Rarity::Uncommon => 15,
        Rarity::Rare => 25,
        Rarity::Epic => 35,
        Rarity::Legendary => 50,
    }
}

fn roll_stats(rng: &mut impl FnMut() -> f64, rarity: Rarity) -> Vec<(StatName, u32)> {
    let floor = rarity_floor(rarity);

    let peak_idx = (rng() * STAT_NAMES.len() as f64) as usize;
    let mut dump_idx = (rng() * STAT_NAMES.len() as f64) as usize;
    while dump_idx == peak_idx {
        dump_idx = (rng() * STAT_NAMES.len() as f64) as usize;
    }

    STAT_NAMES
        .iter()
        .enumerate()
        .map(|(i, &name)| {
            let value = if i == peak_idx {
                (floor + 50 + (rng() * 30.0) as u32).min(100)
            } else if i == dump_idx {
                (floor as i32 - 10 + (rng() * 15.0) as i32).max(1) as u32
            } else {
                floor + (rng() * 40.0) as u32
            };
            (name, value)
        })
        .collect()
}

const SALT: &str = "friend-2026-401";

/// Roll a creature from a seed string. Deterministic.
pub fn roll(seed: &str) -> Creature {
    let key = format!("{}{}", seed, SALT);
    let mut rng = mulberry32(hash_string(&key));

    let rarity = roll_rarity(&mut rng);
    let species = pick(&mut rng, SPECIES).to_string();
    let eye = pick(&mut rng, EYES).to_string();
    let hat = if rarity == Rarity::Common {
        "none".to_string()
    } else {
        pick(&mut rng, HATS).to_string()
    };
    let shiny = rng() < 0.01;
    let base_stats = roll_stats(&mut rng, rarity);

    Creature {
        species,
        eye,
        hat,
        rarity,
        shiny,
        base_stats,
        seed: seed.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_is_deterministic() {
        let a = roll("testuser");
        let b = roll("testuser");
        assert_eq!(a.species, b.species);
        assert_eq!(a.rarity, b.rarity);
        assert_eq!(a.eye, b.eye);
        assert_eq!(a.hat, b.hat);
        assert_eq!(a.shiny, b.shiny);
    }

    #[test]
    fn different_seeds_different_creatures() {
        let a = roll("alice");
        let b = roll("bob");
        // Not guaranteed different but extremely unlikely to match all
        assert!(a.species != b.species || a.eye != b.eye || a.rarity != b.rarity);
    }

    #[test]
    fn stats_have_correct_count() {
        let c = roll("testuser");
        assert_eq!(c.base_stats.len(), 5);
    }

    #[test]
    fn stats_are_in_range() {
        let c = roll("testuser");
        for (_, value) in &c.base_stats {
            assert!(*value >= 1 && *value <= 100);
        }
    }

    #[test]
    fn face_returns_something() {
        let c = roll("testuser");
        assert!(!c.face().is_empty());
    }
}
