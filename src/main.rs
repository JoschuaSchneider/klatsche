mod alias;

use std::env::args;
use colored::Colorize;
use alias::{Alias};

fn main() {
    let aliases = Alias::from_stdin();

    let args: Vec<String> = args().collect();
    let typed_command = args.get(1).unwrap().trim();

    /*
        TODO: Expand already used aliases to give better hints
        "g status" -> "git status" -> use gst instead
     */
    let mut last_command_size: usize = 0;
    let mut most_relevant_alias: Option<Alias> = None;

    for alias in aliases {
        if alias.command.len() < last_command_size
            || !typed_command.starts_with(alias.command.as_str()) {
            continue;
        }
        last_command_size = alias.command.len();
        most_relevant_alias = Some(alias);
    }

    match most_relevant_alias {
        Some(alias) => println!(
            "{} {} {} {} {}",
            " *slap* ".italic().white().on_blue(),
            alias.command.strikethrough().bright_cyan(),
            "use".bright_blue(),
            alias.name.bright_green().bold(),
            "instead".bright_blue(),
        ),
        _ => ()
    }
}
