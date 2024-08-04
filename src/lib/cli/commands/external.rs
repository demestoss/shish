use crate::cli::path_utils::find_command_path;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, io};

#[derive(Debug)]
pub(crate) struct Command {
    input: String,
}

impl Command {
    pub(crate) fn new(input: &str) -> Self {
        Self {
            input: input.to_owned(),
        }
    }

    pub(crate) fn execute(&self) -> anyhow::Result<i32> {
        let input = self.input.trim();
        let (command, args) = input.split_once(" ").unwrap_or((input, ""));

        let path = if let Some(path) = find_buildin_path(&command) {
            Some(path)
        } else {
            find_command_path(&command)
        };

        match path {
            Some(path) => {
                let output = std::process::Command::new(path)
                    .args(args.split_whitespace())
                    .output()?;
                io::stdout().write_all(&output.stdout)?;
                io::stderr().write_all(&output.stderr)?;
                Ok(output.status.code().unwrap_or(1))
            }
            None => {
                println!("{command}: command not found");
                Ok(0)
            }
        }
    }
}

fn find_buildin_path(command: &str) -> Option<PathBuf> {
    match command {
        "true" | "false" => Some(get_buildin_path(&command)),
        _ => None,
    }
}

fn get_buildin_path(bin: &str) -> PathBuf {
    if let Ok(exec_path) = env::current_exe() {
        let mut binary_dir = exec_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
        binary_dir.push(&bin);
        binary_dir
    } else {
        let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
        let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
        PathBuf::from(format!("{}/{}/{}", target_dir, profile, bin))
    }
}
