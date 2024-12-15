use buildin::Invoke;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Command {
    path: PathBuf,
}

impl Invoke for Command {
    fn invoke(&self) -> anyhow::Result<i32> {
        println!("Not implemented: {}", self.path.display());
        Ok(0)
    }
}
