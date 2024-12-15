use std::path::PathBuf;

use buildin::{Invoke, Retrieve};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Command {}

impl Command {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Command {
    fn default() -> Self {
        Self::new()
    }
}

impl Invoke for Command {
    fn invoke(&self) -> anyhow::Result<i32> {
        let path = self.retrieve()?;
        println!("{}", path.display());
        Ok(0)
    }
}

impl Retrieve for Command {
    type Output = PathBuf;

    fn retrieve(&self) -> anyhow::Result<Self::Output> {
        let path = std::env::current_dir()?;
        Ok(path)
    }
}
