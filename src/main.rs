mod commands;
mod path_utils;

use clap::Parser;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    loop {
        print_line_start()?;
        let command = get_user_input()?;
        handle_user_input(&command);
    }
}

#[derive(Debug, Parser)]
enum ShishCli {
    Exit(commands::exit::Command),
    Type(commands::r#type::Command),
    Echo(commands::echo::Command),
    Pwd(commands::pwd::Command),
    Cd(commands::cd::Command),
}

impl ShishCli {
    fn execute(&self) -> anyhow::Result<()> {
        match self {
            ShishCli::Cd(c) => c.execute(),
            ShishCli::Exit(c) => Ok(c.execute()),
            ShishCli::Type(c) => Ok(c.execute()),
            ShishCli::Echo(c) => Ok(c.execute()),
            ShishCli::Pwd(c) => c.execute(),
        }
    }
}

fn print_line_start() -> Result<(), io::Error> {
    print!("$ ");
    io::stdout().flush()
}

fn get_user_input() -> Result<String, io::Error> {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    Ok(input)
}

fn handle_user_input(input: &str) {
    let input = input.trim();

    if input.is_empty() {
        return;
    }

    let (command, _) = input.split_once(" ").unwrap_or((input, ""));

    let mut args = vec!["shish"];
    args.extend(input.split(" "));
    let parsed_command = ShishCli::try_parse_from(args);

    let command_result = match parsed_command {
        Ok(c) => c.execute(),
        Err(e) => commands::external::Command::new(&input).execute(),
    };

    match command_result {
        Ok(_) => {}
        Err(err) => eprintln!("{command}: {err}"),
    }
}
