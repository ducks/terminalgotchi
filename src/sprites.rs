//! ASCII sprites for all 18 species, 3 frames each.
//! Ported from Claude Code's buddy/sprites.ts.

/// Get the sprite frames for a species. Each frame is 4-5 lines.
/// `{E}` in the original is replaced with the eye character.
pub fn frames(species: &str, eye: &str) -> Vec<Vec<String>> {
    let raw = raw_frames(species);
    raw.iter()
        .map(|frame| {
            frame
                .iter()
                .map(|line| line.replace("{E}", eye))
                .collect()
        })
        .collect()
}

fn raw_frames(species: &str) -> Vec<Vec<&'static str>> {
    match species {
        "duck" => vec![
            vec!["    __      ", "  <({E} )___  ", "   (  ._>   ", "    `--'    "],
            vec!["    __      ", "  <({E} )___  ", "   (  ._>   ", "    `--'~   "],
            vec!["    __      ", "  <({E} )___  ", "   (  .__>  ", "    `--'    "],
        ],
        "goose" => vec![
            vec!["     ({E}>    ", "     ||     ", "   _(__)_   ", "    ^^^^    "],
            vec!["    ({E}>     ", "     ||     ", "   _(__)_   ", "    ^^^^    "],
            vec!["     ({E}>>   ", "     ||     ", "   _(__)_   ", "    ^^^^    "],
        ],
        "blob" => vec![
            vec!["   .----.   ", "  ( {E}  {E} )  ", "  (      )  ", "   `----'   "],
            vec!["  .------.  ", " (  {E}  {E}  ) ", " (        ) ", "  `------'  "],
            vec!["    .--.    ", "   ({E}  {E})   ", "   (    )   ", "    `--'    "],
        ],
        "cat" => vec![
            vec!["   /\\_/\\    ", "  ( {E}   {E})  ", "  (  w  )   ", "  (\")_(\")   "],
            vec!["   /\\_/\\    ", "  ( {E}   {E})  ", "  (  w  )   ", "  (\")_(\")~  "],
            vec!["   /\\-/\\    ", "  ( {E}   {E})  ", "  (  w  )   ", "  (\")_(\")   "],
        ],
        "dragon" => vec![
            vec!["  /^\\  /^\\  ", " <  {E}  {E}  > ", " (   ~~   ) ", "  `-vvvv-'  "],
            vec!["  /^\\  /^\\  ", " <  {E}  {E}  > ", " (        ) ", "  `-vvvv-'  "],
            vec!["  /^\\  /^\\  ", " <  {E}  {E}  > ", " (   ~~   ) ", "  `-vvvv-'  "],
        ],
        "octopus" => vec![
            vec!["   .----.   ", "  ( {E}  {E} )  ", "  (______)  ", "  /\\/\\/\\/\\  "],
            vec!["   .----.   ", "  ( {E}  {E} )  ", "  (______)  ", "  \\/\\/\\/\\/  "],
            vec!["   .----.   ", "  ( {E}  {E} )  ", "  (______)  ", "  /\\/\\/\\/\\  "],
        ],
        "owl" => vec![
            vec!["   /\\  /\\   ", "  (({E})({E}))  ", "  (  ><  )  ", "   `----'   "],
            vec!["   /\\  /\\   ", "  (({E})({E}))  ", "  (  ><  )  ", "   .----.   "],
            vec!["   /\\  /\\   ", "  (({E})(-))  ", "  (  ><  )  ", "   `----'   "],
        ],
        "penguin" => vec![
            vec!["  .---.     ", "  ({E}>{E})     ", " /(   )\\    ", "  `---'     "],
            vec!["  .---.     ", "  ({E}>{E})     ", " |(   )|    ", "  `---'     "],
            vec!["  .---.     ", "  ({E}>{E})     ", " /(   )\\    ", "  `---'     "],
        ],
        "turtle" => vec![
            vec!["   _,--._   ", "  ( {E}  {E} )  ", " /[______]\\ ", "  ``    ``  "],
            vec!["   _,--._   ", "  ( {E}  {E} )  ", " /[______]\\ ", "   ``  ``   "],
            vec!["   _,--._   ", "  ( {E}  {E} )  ", " /[======]\\ ", "  ``    ``  "],
        ],
        "snail" => vec![
            vec![" {E}    .--.  ", "  \\  ( @ )  ", "   \\_`--'   ", "  ~~~~~~~   "],
            vec!["  {E}   .--.  ", "  |  ( @ )  ", "   \\_`--'   ", "  ~~~~~~~   "],
            vec![" {E}    .--.  ", "  \\  ( @  ) ", "   \\_`--'   ", "   ~~~~~~   "],
        ],
        "ghost" => vec![
            vec!["   .----.   ", "  / {E}  {E} \\  ", "  |      |  ", "  ~`~``~`~  "],
            vec!["   .----.   ", "  / {E}  {E} \\  ", "  |      |  ", "  `~`~~`~`  "],
            vec!["   .----.   ", "  / {E}  {E} \\  ", "  |      |  ", "  ~~`~~`~~  "],
        ],
        "axolotl" => vec![
            vec!["}~(______)~{", "}~({E} .. {E})~{", "  ( .--. )  ", "  (_/  \\_)  "],
            vec!["~}(______){~", "~}({E} .. {E}){~", "  ( .--. )  ", "  (_/  \\_)  "],
            vec!["}~(______)~{", "}~({E} .. {E})~{", "  (  --  )  ", "  ~_/  \\_~  "],
        ],
        "capybara" => vec![
            vec!["  n______n  ", " ( {E}    {E} ) ", " (   oo   ) ", "  `------'  "],
            vec!["  n______n  ", " ( {E}    {E} ) ", " (   Oo   ) ", "  `------'  "],
            vec!["  u______n  ", " ( {E}    {E} ) ", " (   oo   ) ", "  `------'  "],
        ],
        "cactus" => vec![
            vec![" n  ____  n ", " | |{E}  {E}| | ", " |_|    |_| ", "   |    |   "],
            vec!["    ____    ", " n |{E}  {E}| n ", " |_|    |_| ", "   |    |   "],
            vec![" n  ____  n ", " | |{E}  {E}| | ", " |_|    |_| ", "   |    |   "],
        ],
        "robot" => vec![
            vec!["   .[||].   ", "  [ {E}  {E} ]  ", "  [ ==== ]  ", "  `------'  "],
            vec!["   .[||].   ", "  [ {E}  {E} ]  ", "  [ -==- ]  ", "  `------'  "],
            vec!["   .[||].   ", "  [ {E}  {E} ]  ", "  [ ==== ]  ", "  `------'  "],
        ],
        "rabbit" => vec![
            vec!["   (\\__/)   ", "  ( {E}  {E} )  ", " =(  ..  )= ", "  (\")__(\"  )"],
            vec!["   (|__/)   ", "  ( {E}  {E} )  ", " =(  ..  )= ", "  (\")__(\"  )"],
            vec!["   (\\__/)   ", "  ( {E}  {E} )  ", " =( .  . )= ", "  (\")__(\"  )"],
        ],
        "mushroom" => vec![
            vec![" .-o-OO-o-. ", "(__________)", "   |{E}  {E}|   ", "   |____|   "],
            vec![" .-O-oo-O-. ", "(__________)", "   |{E}  {E}|   ", "   |____|   "],
            vec![" .-o-OO-o-. ", "(__________)", "   |{E}  {E}|   ", "   |____|   "],
        ],
        "chonk" => vec![
            vec!["  /\\    /\\  ", " ( {E}    {E} ) ", " (   ..   ) ", "  `------'  "],
            vec!["  /\\    /|  ", " ( {E}    {E} ) ", " (   ..   ) ", "  `------'  "],
            vec!["  /\\    /\\  ", " ( {E}    {E} ) ", " (   ..   ) ", "  `------'~ "],
        ],
        _ => vec![
            vec!["   .----.   ", "  ( {E}  {E} )  ", "  (      )  ", "   `----'   "],
        ],
    }
}

/// Hat overlays (replace the top line if present).
pub fn hat_line(hat: &str) -> Option<&'static str> {
    match hat {
        "crown" => Some("   \\^^^/    "),
        "tophat" => Some("   [___]    "),
        "propeller" => Some("    -+-     "),
        "halo" => Some("   (   )    "),
        "wizard" => Some("    /^\\     "),
        "beanie" => Some("   (___)    "),
        "tinyduck" => Some("    ,>      "),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_species_have_frames() {
        let species = [
            "duck", "goose", "blob", "cat", "dragon", "octopus", "owl",
            "penguin", "turtle", "snail", "ghost", "axolotl", "capybara",
            "cactus", "robot", "rabbit", "mushroom", "chonk",
        ];
        for s in species {
            let f = frames(s, "o");
            assert!(f.len() >= 1, "no frames for {}", s);
            assert!(f[0].len() >= 4, "too few lines for {}", s);
        }
    }

    #[test]
    fn eye_substitution_works() {
        let f = frames("duck", "X");
        assert!(f[0][1].contains("X"));
    }

    #[test]
    fn hats_return_something() {
        assert!(hat_line("crown").is_some());
        assert!(hat_line("none").is_none());
    }
}
