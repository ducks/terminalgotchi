# termagotchi

A terminal companion that grows with you. Your dev activity feeds it XP.

## How It Works

Your username is hashed to deterministically generate a companion creature with a species, rarity, eyes, hat, and base stats. The creature is permanent -- you can't reroll.

As you work, your shell hook feeds commands to termagotchi. `cargo test` gives DEBUGGING XP. `git commit` gives PATIENCE XP. `rm -rf` gives CHAOS XP. The stats grow over time based on how you actually code.

## Install

```bash
cargo install termagotchi
```

Requires Rust 1.88+.

## Setup

### 1. See your companion

```bash
termagotchi
```

### 2. Install the shell hook

```bash
# Print hook code for your shell
termagotchi hook bash   # or: zsh, nu, fish

# Then add the output to your shell config
termagotchi hook nu >> ~/.config/nushell/config.nu
```

The hook silently runs `termagotchi feed-cmd` after every command. No output, no delay.

### 3. Watch it grow

```bash
# Full stat card
termagotchi card

# XP breakdown
termagotchi stats

# Animated idle
termagotchi watch
```

## Commands

| Command | Description |
|---------|-------------|
| `termagotchi` | Show your companion card (default) |
| `termagotchi card` | Same as above |
| `termagotchi watch` | Animated idle loop |
| `termagotchi stats` | XP and growth breakdown |
| `termagotchi feed <activity>` | Manual XP: debug, commit, chaos, read, hack |
| `termagotchi feed-cmd <cmd>` | Auto-detect activity from a command |
| `termagotchi hook [shell]` | Print shell hook code |
| `termagotchi reset` | Clear all XP |

## Activity Mapping

| Commands | Stat | Examples |
|----------|------|----------|
| test, lint, debug | DEBUGGING | `cargo test`, `pytest`, `clippy` |
| commit, merge, PR | PATIENCE | `git commit`, `git merge` |
| rm, force-push, reset | CHAOS | `rm -rf`, `git push --force` |
| man, docs, search | WISDOM | `man ls`, `curl`, `grep` |
| sed, awk, piping | SNARK | `sed`, `awk`, one-liners |

10 XP = 1 stat point. Base stats come from your deterministic roll.

## Creatures

18 species: duck, goose, blob, cat, dragon, octopus, owl, penguin, turtle, snail, ghost, axolotl, capybara, cactus, robot, rabbit, mushroom, chonk.

5 rarity tiers: Common (60%), Uncommon (25%), Rare (10%), Epic (4%), Legendary (1%).

Non-common creatures get hats. 1% chance of shiny.

## Data

XP stored in `~/.local/share/termagotchi/growth.db` (SQLite).

Creature is not stored -- it's regenerated from your username hash every time. You can't fake a rarity by editing files.

## Options

```bash
# Use a different seed (not your username)
termagotchi --seed mycustomseed

# Works with all commands
termagotchi --seed mycustomseed watch
```

## License

MIT
