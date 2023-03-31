use std::io;
use std::io::Read;

pub struct Alias {
    pub name: String,
    pub command: String,
}

impl Alias {
    fn new(name: String, command: String) -> Alias {
        Alias { name, command: alias_resolve_command(command) }
    }
    fn from_definition(definition: &str) -> Option<Alias> {
        let parts: Vec<&str> = definition.split("=").collect();

        match (parts.get(0), parts.get(1)) {
            (Some(name), Some(command)) => Some(
                Alias::new(name.trim().to_string(), command.trim().to_string())
            ),
            _ => None,
        }
    }
    pub fn from_stdin() -> Vec<Alias> {
        let mut stdin = io::stdin();
        let mut buffer = Vec::new();
        stdin.read_to_end(&mut buffer).unwrap();
        let lines: Vec<&str> = std::str::from_utf8(&buffer).unwrap().lines().collect();

        lines.iter()
            .filter_map(|line| Alias::from_definition(line))
            .collect()
    }
}

fn alias_resolve_command(command: String) -> String {
    command.replace("'", "")
}