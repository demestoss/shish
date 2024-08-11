use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub(crate) struct Command {
    path: PathBuf,
}

impl Command {
    pub(crate) fn execute(&self) -> anyhow::Result<i32> {
        println!("{}", self.path.display());
        Ok(0)
    }
}
