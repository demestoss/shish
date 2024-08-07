use std::process::Stdio;

use clap::error::ErrorKind;
use clap::Parser;

use crate::commands;
use crate::commands::external::execute_external_command;

#[derive(Debug, Parser)]
enum SpecialBuildin {
    Exit(commands::exit::Command),
    Type(commands::r#type::Command),
    Pwd(commands::pwd::Command),
    Cd(commands::cd::Command),
    Mkdir(commands::mkdir::Command),
}

impl SpecialBuildin {
    fn execute(&self) -> anyhow::Result<()> {
        match self {
            SpecialBuildin::Cd(c) => c.execute(),
            SpecialBuildin::Exit(c) => Ok(c.execute()),
            SpecialBuildin::Type(c) => Ok(c.execute()),
            SpecialBuildin::Pwd(c) => c.execute(),
            SpecialBuildin::Mkdir(c) => c.execute(),
        }
    }
}

pub fn handle_user_input(input: &str) {
    let input = input.trim();

    if input.is_empty() {
        return;
    }

    let mut args = shlex::split(input).unwrap_or(Vec::new());

    let mut command_args = vec![];
    let mut previous_command = None;

    for arg in args {
        match arg.as_str() {
            "|" => {
                match execute_command(&command_args) {
                    None => previous_command = None,
                    Some(_) => match execute_external_command(
                        &command_args,
                        previous_command,
                        Stdio::piped(),
                    ) {
                        Some(command) => previous_command = Some(command),
                        None => previous_command = None,
                    },
                }
                command_args.clear();
            }
            "&&" => {
                if let Some(_) = execute_command(&command_args) {
                    execute_external_command(&command_args, previous_command, Stdio::inherit());
                }
                previous_command = None;
                command_args.clear();
            }
            _ => command_args.push(arg),
        }
    }

    if command_args.len() {
        execute_external_command(&command_args, previous_command, Stdio::inherit());
    }
}

fn get_stdout(args: &mut [String]) -> Stdio {
    let Some(arg) = args.get(args.len() - 2) else {
        return Stdio::inherit();
    };
    if arg != ">" {
        return Stdio::inherit();
    }

    let file_name = args.pop();
}

fn execute_command(args: &[String]) -> Option<()> {
    match SpecialBuildin::try_parse_from(&args) {
        Ok(c) => {
            if let Err(err) = c.execute() {
                eprintln!("{}: {err}", args[0]);
            }
            None
        }
        Err(e) => match e.kind() {
            ErrorKind::DisplayHelp => {
                println!("{e}");
                None
            }
            _ => Some(()),
        },
    }
}
