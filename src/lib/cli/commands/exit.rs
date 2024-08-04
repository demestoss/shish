use clap::Parser;
use std::process;

#[derive(Parser, Debug)]
pub(crate) struct Command {
    status_code: Option<i32>,
}

impl Command {
    pub(crate) fn execute(&self) {
        let status_code = self.status_code.unwrap_or(0);
        process::exit(status_code);
    }
}
