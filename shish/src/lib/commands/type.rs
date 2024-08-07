use crate::path_utils::find_command_path;
use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct Command {
    command: String,
}

impl Command {
    pub(crate) fn execute(&self) {
        self.command
            .split_whitespace()
            .for_each(|param| match param {
                "" => {}
                "echo" | "exit" | "type" | "pwd" | "cd" | "true" | "false" | "mkdir" => {
                    println!("{param} is a shell builtin")
                }
                command => match find_command_path(command) {
                    Some(path) => println!("{command} is {}", path.display()),
                    None => eprintln!("{param}: not found"),
                },
            })
    }
}
