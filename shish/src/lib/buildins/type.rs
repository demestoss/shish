use crate::path_utils::find_command_path;
use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct Command {
    command: Vec<String>,
}

impl Command {
    pub(crate) fn execute(&self) -> anyhow::Result<i32> {
        let mut code = 0;
        self.command.iter().for_each(|param| match param.as_str() {
            "" => {}
            "echo" | "exit" | "type" | "pwd" | "cd" | "true" | "false" | "mkdir" => {
                println!("{param} is a shell builtin")
            }
            command => match find_command_path(command) {
                Some(path) => println!("{command} is {}", path.display()),
                None => {
                    code = 1;
                    eprintln!("{param}: not found");
                }
            },
        });
        Ok(code)
    }
}
