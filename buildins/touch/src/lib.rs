use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Command {
    path: PathBuf,
}

impl Command {
    pub fn invoke(&self) -> anyhow::Result<i32> {
        println!("{}", self.path.display());
        Ok(0)
    }
}
