use crate::path_utils::find_command_path;
use std::io;
use std::io::Write;

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

    pub(crate) fn execute(&self) -> anyhow::Result<()> {
        let input = self.input.trim();
        let (command, args) = input.split_once(" ").unwrap_or((input, ""));

        match find_command_path(command) {
            Some(path) => {
                let output = std::process::Command::new(path)
                    .args(args.split_whitespace())
                    .output()?;
                io::stdout().write_all(&output.stdout)?;
                io::stderr().write_all(&output.stderr)?;
            }
            None => println!("{command}: command not found"),
        };
        Ok(())
    }
}
