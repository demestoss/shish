use clap::Parser;
use std::process;

#[derive(Parser, Debug)]
pub(crate) struct Command {
    status_code: i8,
}

impl Command {
    pub(crate) fn execute(&self) {
        process::exit(self.status_code as i32);
    }
}
