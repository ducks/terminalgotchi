//! Stat types and rarity definitions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl Rarity {
    pub fn name(&self) -> &str {
        match self {
            Rarity::Common => "Common",
            Rarity::Uncommon => "Uncommon",
            Rarity::Rare => "Rare",
            Rarity::Epic => "Epic",
            Rarity::Legendary => "Legendary",
        }
    }

    pub fn color(&self) -> &str {
        match self {
            Rarity::Common => "gray",
            Rarity::Uncommon => "green",
            Rarity::Rare => "blue",
            Rarity::Epic => "purple",
            Rarity::Legendary => "gold",
        }
    }
}

pub const RARITIES: &[Rarity] = &[
    Rarity::Common,
    Rarity::Uncommon,
    Rarity::Rare,
    Rarity::Epic,
    Rarity::Legendary,
];

pub const RARITY_WEIGHTS: &[(Rarity, u32)] = &[
    (Rarity::Common, 60),
    (Rarity::Uncommon, 25),
    (Rarity::Rare, 10),
    (Rarity::Epic, 4),
    (Rarity::Legendary, 1),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatName {
    Debugging,
    Patience,
    Chaos,
    Wisdom,
    Snark,
}

impl StatName {
    pub fn name(&self) -> &str {
        match self {
            StatName::Debugging => "DEBUGGING",
            StatName::Patience => "PATIENCE",
            StatName::Chaos => "CHAOS",
            StatName::Wisdom => "WISDOM",
            StatName::Snark => "SNARK",
        }
    }
}

impl std::fmt::Display for StatName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub const STAT_NAMES: &[StatName] = &[
    StatName::Debugging,
    StatName::Patience,
    StatName::Chaos,
    StatName::Wisdom,
    StatName::Snark,
];
