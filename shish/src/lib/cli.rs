use anyhow::{anyhow, bail};
use clap::error::ErrorKind;
use clap::Parser;
use std::fs;
use std::process::{Child, Stdio};

use crate::buildins;
use crate::external::execute_external_command;

#[derive(Debug, Parser)]
enum SpecialBuildin {
    Exit(buildins::exit::Command),
    Type(buildins::r#type::Command),
    Pwd(buildins::pwd::Command),
    Cd(buildins::cd::Command),
    Mkdir(buildins::mkdir::Command),
    True(buildins::r#true::Command),
    False(buildins::r#false::Command),
}

impl SpecialBuildin {
    fn execute(&self) -> anyhow::Result<i32> {
        match self {
            SpecialBuildin::Cd(c) => c.execute(),
            SpecialBuildin::Exit(c) => {
                c.execute();
                Ok(0)
            }
            SpecialBuildin::Type(c) => c.execute(),
            SpecialBuildin::Pwd(c) => c.execute(),
            SpecialBuildin::Mkdir(c) => c.execute(),
            SpecialBuildin::True(c) => c.execute(),
            SpecialBuildin::False(c) => c.execute(),
        }
    }
}

pub fn handle_user_input(input: &str) -> anyhow::Result<i32> {
    let input = input.trim();

    if input.is_empty() {
        return Ok(0);
    }

    let args = shlex::split(input).unwrap_or(Vec::new());

    let mut command_args = vec![];
    let mut previous_command = None;

    for arg in args {
        match arg.as_str() {
            "|" => {
                match execute_command(&command_args, previous_command, Stdio::piped())? {
                    CommandResult::Code(code) => match code {
                        0 => previous_command = None,
                        c => return Ok(c),
                    },
                    CommandResult::Child(c) => previous_command = Some(c),
                };
                command_args.clear();
            }
            "&&" | "||" => {
                let stdout = get_stdout(&mut command_args)?;
                let res = execute_command(&command_args, previous_command, stdout)?;

                if arg == "&&" {
                    match res {
                        CommandResult::Code(code) => match code {
                            0 => {}
                            c => return Ok(c),
                        },
                        CommandResult::Child(mut c) => {
                            let res = c.wait()?;
                            if !res.success() {
                                return Ok(res.code().unwrap_or(1));
                            }
                        }
                    }
                } else if arg == "||" {
                    match res {
                        CommandResult::Code(code) => match code {
                            0 => return Ok(0),
                            _ => {}
                        },
                        CommandResult::Child(mut c) => {
                            if c.wait()?.success() {
                                return Ok(0);
                            }
                        }
                    }
                }
                previous_command = None;
                command_args.clear();
            }
            _ => command_args.push(arg),
        }
    }

    if !command_args.is_empty() {
        let stdout = get_stdout(&mut command_args)?;
        match execute_command(&command_args, previous_command, stdout)? {
            CommandResult::Child(mut c) => {
                let res = c.wait()?;
                Ok(res.code().unwrap_or(1))
            }
            CommandResult::Code(code) => Ok(code),
        }
    } else {
        Ok(0)
    }
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

enum CommandResult {
    Code(i32),
    Child(Child),
}

fn execute_command(
    args: &Vec<String>,
    previous: Option<Child>,
    stdout: Stdio,
) -> anyhow::Result<CommandResult> {
    let mut args_n = vec!["".to_string()];
    args_n.append(&mut args.clone());

    match SpecialBuildin::try_parse_from(args_n) {
        Ok(c) => {
            let res = c.execute().map_err(|e| anyhow!("{}: {e}", args[0]))?;
            Ok(CommandResult::Code(res))
        }
        Err(e) => match e.kind() {
            ErrorKind::DisplayHelp
            | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
            | ErrorKind::DisplayVersion => {
                println!("{e}");
                Ok(CommandResult::Code(0))
            }
            _ => execute_external_command(&args, previous, stdout).map(|c| CommandResult::Child(c)),
        },
    }
}
