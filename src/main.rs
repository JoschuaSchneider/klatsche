use colored::Colorize;
use std::{
    env::args,
    io::{stdin, Read},
    str::from_utf8,
};

struct Match {
    rank: usize,
    full_match: bool,
    exact_match: bool,
    alias: (String, String),
}

trait NormalizeCommandExt {
    fn normalize_command(&self) -> &str;
}

impl<'a> NormalizeCommandExt for &'a str {
    fn normalize_command(&self) -> &str {
        return self.trim_matches(|c| c == '\'' || c == '"');
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let typed_command = match args.get(1).map(|c| c.trim()) {
        Some(c) => c,
        None => return,
    };

    let mut buffer = Vec::new();
    match stdin().read_to_end(&mut buffer) {
        Ok(_) => (),
        Err(_) => return,
    }

    let mut alias_matches: Vec<Match> = from_utf8(&buffer)
        .unwrap()
        .lines()
        .filter_map(|line| match line.split('=').collect::<Vec<_>>()[0..2] {
            [alias, command] => Some((
                alias.normalize_command().to_string(),
                command.normalize_command().to_string(),
            )),
            _ => None,
        })
        .filter_map(|alias| {
            let rank = typed_command
                .chars()
                .zip(alias.1.chars())
                .fold(0, |acc, (c, a)| match c == a {
                    true => acc + 1,
                    false => acc,
                });

            let full_match = rank == alias.1.len();
            let exact_match = rank == typed_command.len();

            match full_match || exact_match {
                true => {
                    return Some(Match {
                        rank,
                        full_match,
                        exact_match,
                        alias,
                    });
                }
                false => None,
            }
        })
        .collect();

    alias_matches.sort_by(|a, b| {
        b.rank
            .partial_cmp(&a.rank)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let best_match = match alias_matches.iter().find(|m| m.full_match) {
        Some(m) => m,
        _ => return,
    };

    if !best_match.full_match {
        return;
    };

    let cmd_replaced_half = typed_command
        .chars()
        .take(best_match.rank)
        .collect::<String>();

    let same_ranked_aliases = alias_matches
        .iter()
        .take_while(|am| am.rank == best_match.rank && am.alias.0 != best_match.alias.0)
        .map(|am| am.alias.0.as_str())
        .collect::<Vec<_>>();

    let explore_hint = match same_ranked_aliases.len() {
        l if l > 0 => format!(
            "{} {}",
            ", explore similar:".bright_blue(),
            same_ranked_aliases.join(" ").bright_cyan()
        ),
        _ => "".into(),
    };

    match best_match.exact_match {
        true => {
            println!(
                "{} {} {} {} {}{}",
                " *big slap* ".italic().white().on_blue(),
                cmd_replaced_half.strikethrough().bright_cyan(),
                "use".bright_blue(),
                best_match.alias.0.bright_green().bold(),
                "instead".bright_blue(),
                explore_hint,
            );
        }
        false => {
            println!(
                "{} {} {} {} {}{}",
                " *slap* ".italic().white().on_blue(),
                "substitute".bright_blue(),
                cmd_replaced_half.strikethrough().bright_cyan(),
                "with".bright_blue(),
                best_match.alias.0.bright_green().bold(),
                explore_hint,
            );
        }
    }
}
