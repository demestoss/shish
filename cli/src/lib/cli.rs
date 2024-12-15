use anyhow::bail;
use buildin::Invoke;
use clap::error::ErrorKind;
use clap::Parser;
use std::fs;
use std::process::{Child, Stdio};

use crate::external::execute_external_command;
use crate::parser;

#[derive(Debug, Parser)]
enum Buildin {
    Exit(buildin_exit::Command),
    Type(buildin_type::Command),
    Pwd(buildin_pwd::Command),
    Cd(buildin_cd::Command),
    Mkdir(buildin_mkdir::Command),
    True(buildin_true::Command),
    False(buildin_false::Command),
    Touch(buildin_touch::Command),
    Cat(buildin_cat::Command),
    Head(buildin_head::Command),
}

impl Buildin {
    fn invoke(&self) -> anyhow::Result<i32> {
        match self {
            Buildin::Cd(c) => c.invoke(),
            Buildin::Exit(c) => c.invoke(),
            Buildin::Type(c) => c.invoke(),
            Buildin::Pwd(c) => c.invoke(),
            Buildin::Mkdir(c) => c.invoke(),
            Buildin::True(c) => c.invoke(),
            Buildin::False(c) => c.invoke(),
            Buildin::Touch(c) => c.invoke(),
            Buildin::Cat(c) => c.invoke(),
            Buildin::Head(c) => c.invoke(),
        }
    }
}

pub fn handle_user_input(input: &str) -> anyhow::Result<i32> {
    let input = input.trim();

    if input.is_empty() {
        return Ok(0);
    }

    let args = parser::args(input)?;

    let mut command_args = vec![];
    let mut previous_command = None;

    for arg in args {
        match arg.as_str() {
            "|" => {
                let _stdout = get_stdout(&mut command_args)?;
                match execute_command(&command_args, previous_command, Stdio::piped())? {
                    CommandResult::Success => {
                        previous_command = None;
                    }
                    CommandResult::Failure(c) => return Ok(c),
                    CommandResult::Child(c) => {
                        previous_command = Some(c);
                    }
                };
                command_args.clear();
            }
            "&&" | "||" => {
                let stdout = get_stdout(&mut command_args)?.unwrap_or(Stdio::inherit());
                let res = execute_command(&command_args, previous_command, stdout)?;

                if arg == "&&" {
                    match res {
                        CommandResult::Success => {}
                        CommandResult::Failure(c) => return Ok(c),
                        CommandResult::Child(mut c) => {
                            let res = c.wait()?;
                            if !res.success() {
                                return Ok(res.code().unwrap_or(1));
                            }
                        }
                    }
                } else if arg == "||" {
                    match res {
                        CommandResult::Success => return Ok(0),
                        CommandResult::Failure(_) => {}
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
        let stdout = get_stdout(&mut command_args)?.unwrap_or(Stdio::inherit());
        match execute_command(&command_args, previous_command, stdout)? {
            CommandResult::Child(mut c) => {
                let res = c.wait()?;
                Ok(res.code().unwrap_or(1))
            }
            CommandResult::Failure(code) => Ok(code),
            CommandResult::Success => Ok(0),
        }
    } else {
        Ok(0)
    }
}

fn get_stdout(args: &mut Vec<String>) -> anyhow::Result<Option<Stdio>> {
    if !is_valid_redirection(args) {
        return Ok(None);
    }

    let file_name = args.pop().unwrap();
    args.pop(); // Remove the ">" symbol

    match fs::File::open(&file_name).or(fs::File::create(&file_name)) {
        Ok(file) => Ok(Some(Stdio::from(file))),
        Err(e) => bail!("{e}"),
    }
}

fn is_valid_redirection(args: &[String]) -> bool {
    if args.len() < 3 {
        return false;
    }
    if let Some(arg) = args.get(args.len() - 2) {
        if arg == ">" {
            return true;
        }
    }
    false
}

enum CommandResult {
    Success,
    Failure(i32),
    Child(Child),
}

fn execute_command(
    args: &[String],
    previous: Option<Child>,
    stdout: Stdio,
) -> anyhow::Result<CommandResult> {
    let mut args_n = vec!["".to_string()];
    args_n.append(&mut args.to_vec());

    match Buildin::try_parse_from(args_n) {
        Ok(c) => match c.invoke() {
            Ok(0) => Ok(CommandResult::Success),
            Ok(c) => Ok(CommandResult::Failure(c)),
            Err(e) => {
                eprintln!("{}: {e}", args[0]);
                // Should pick the error code from error
                Ok(CommandResult::Failure(1))
            }
        },
        Err(e) => match e.kind() {
            ErrorKind::DisplayHelp
            | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
            | ErrorKind::DisplayVersion => {
                println!("{e}");
                Ok(CommandResult::Success)
            }
            _ => execute_external_command(args, previous, stdout).map(CommandResult::Child),
        },
    }
}
