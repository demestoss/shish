use std::path::{Path, PathBuf};

use buildin::Invoke;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Command {
    command: Vec<String>,
}

impl Invoke for Command {
    fn invoke(&self) -> anyhow::Result<i32> {
        let mut code = 0;
        self.command.iter().for_each(|param| match param.as_str() {
            "" => {}
            "cat" | "echo" | "touch" | "exit" | "type" | "pwd" | "cd" | "true" | "false" | "head" | "grepr"
            | "mkdir" => {
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

fn find_command_path(command: &str) -> Option<PathBuf> {
    let path_env = std::env::var("PATH").ok()?;
    path_env.split(':').find_map(|dir| {
        let path = Path::new(dir).join(command);
        match path.try_exists() {
            Ok(true) => Some(path),
            _ => None,
        }
    })
}
