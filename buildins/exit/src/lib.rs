use clap::Parser;
use std::process;

#[derive(Parser, Debug)]
pub struct Command {
    status_code: Option<i32>,
}

impl Command {
    pub fn invoke(&self) -> anyhow::Result<i32> {
        let status_code = self.status_code.unwrap_or(0);
        process::exit(status_code);
    }
}
