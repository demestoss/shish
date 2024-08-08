use anyhow::bail;
use clap::error::ErrorKind;
use clap::Parser;
use std::fs;
use std::process::{Child, Stdio};

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

    let args = shlex::split(input).unwrap_or(Vec::new());

    let mut command_args = vec![];
    let mut previous_command = None;

    for arg in args {
        match arg.as_str() {
            "|" => handle_pipe(&mut command_args, &mut previous_command),
            "&&" => handle_and(&mut command_args, &mut previous_command),
            "||" => handle_or(&mut command_args, &mut previous_command),
            _ => command_args.push(arg),
        }
    }

    if !command_args.is_empty() {
        match get_stdout(&mut command_args)
            .map(|stdout| execute_command(&command_args, previous_command, stdout))
        {
            Err(e) => eprintln!("{e}"),
            Ok(Some(c)) => c.wait(),
            _ => {}
        };
    }
}

fn handle_pipe(command_args: &mut Vec<String>, previous_command: &mut Option<Child>) {
    match execute_command(command_args, previous_command.take(), Stdio::piped()) {
        Err(e) => {
            eprintln!("{e}");
            *previous_command = None;
        }
        Ok(c) => *previous_command = c,
    }
    command_args.clear();
}

fn handle_and(command_args: &mut Vec<String>, previous_command: &mut Option<Child>) {
    match execute_command(command_args, previous_command.take(), Stdio::inherit()) {
        Err(e) => {
            eprintln!("{e}");
            return;
        }
        Ok(None) => {
            // TODO: internal buildins are not working here, always good path
        }
        Ok(Some(mut c)) => match c.wait() {
            Err(e) | Ok(st) if !st.success() => {
                eprintln!("{e}");
                return;
            }
            _ => {}
        },
    }
    *previous_command = None;
    command_args.clear();
}

fn handle_or(command_args: &mut Vec<String>, previous_command: &mut Option<Child>) {
    match execute_command(command_args, previous_command.take(), Stdio::inherit()) {
        Err(e) => eprintln!("{e}"),
        Ok(None) => {
            // TODO: internal buildins are not working here, always good path
        }
        Ok(Some(mut c)) => match c.wait() {
            Err(e) | Ok(st) if !st.success() => eprintln!("{e}"),
            _ => {
                return;
            }
        },
    }
    *previous_command = None;
    command_args.clear();
}

fn get_stdout(args: &mut Vec<String>) -> anyhow::Result<Stdio> {
    if args.len() < 3 {
        return Ok(Stdio::inherit());
    }
    let Some(arg) = args.get(args.len() - 2) else {
        return Ok(Stdio::inherit());
    };
    if arg != ">" {
        return Ok(Stdio::inherit());
    }

    let Some(file_name) = args.pop() else {
        return Ok(Stdio::inherit());
    };
    let _ = args.pop();

    match fs::File::open(&file_name).or(fs::File::create(&file_name)) {
        Ok(file) => Ok(Stdio::from(file)),
        Err(e) => bail!("{e}"),
    }
}

fn execute_command(
    args: &Vec<String>,
    previous: Option<Child>,
    stdout: Stdio,
) -> anyhow::Result<Option<Child>> {
    // This is bad
    let mut args_n = vec!["".to_string()];
    args_n.append(&mut args.clone());

    match SpecialBuildin::try_parse_from(args_n) {
        Ok(c) => {
            if let Err(err) = c.execute() {
                bail!("{}: {err}", args[0]);
            }
            Ok(None)
        }
        Err(e) => match e.kind() {
            ErrorKind::DisplayHelp
            | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
            | ErrorKind::DisplayVersion => {
                println!("{e}");
                Ok(None)
            }
            _ => execute_external_command(&args, previous, stdout).map(|c| Some(c)),
        },
    }
}
