# terminalgotchi

A terminal companion that grows with you. Your dev activity feeds it XP.

## How It Works

Your username is hashed to deterministically generate a companion creature with a species, rarity, eyes, hat, and base stats. The creature is permanent -- you can't reroll.

As you work, your shell hook feeds commands to terminalgotchi. `cargo test` gives DEBUGGING XP. `git commit` gives PATIENCE XP. `rm -rf` gives CHAOS XP. The stats grow over time based on how you actually code.

## Install

```bash
cargo install terminalgotchi
```

Requires Rust 1.88+.

## Setup

### 1. See your companion

```bash
terminalgotchi
```

### 2. Install the shell hook

```bash
# Print hook code for your shell
terminalgotchi hook bash   # or: zsh, nu, fish

# Then add the output to your shell config
terminalgotchi hook nu >> ~/.config/nushell/config.nu
```

The hook silently runs `terminalgotchi feed-cmd` after every command. No output, no delay.

### 3. Watch it grow

```bash
# Full stat card
terminalgotchi card

# XP breakdown
terminalgotchi stats

# Animated idle
terminalgotchi watch
```

## Commands

| Command | Description |
|---------|-------------|
| `terminalgotchi` | Show your companion card (default) |
| `terminalgotchi card` | Same as above |
| `terminalgotchi watch` | Animated idle loop |
| `terminalgotchi stats` | XP and growth breakdown |
| `terminalgotchi feed <activity>` | Manual XP: debug, commit, chaos, read, hack |
| `terminalgotchi feed-cmd <cmd>` | Auto-detect activity from a command |
| `terminalgotchi hook [shell]` | Print shell hook code |
| `terminalgotchi reset` | Clear all XP |

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

XP stored in `~/.local/share/terminalgotchi/growth.db` (SQLite).

Creature is not stored -- it's regenerated from your username hash every time. You can't fake a rarity by editing files.

## Options

```bash
# Use a different seed (not your username)
terminalgotchi --seed mycustomseed

# Works with all commands
terminalgotchi --seed mycustomseed watch
```

## License

MIT
