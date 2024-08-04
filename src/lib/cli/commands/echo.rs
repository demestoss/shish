use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct Command {
    line: String,
}

impl Command {
    pub(crate) fn execute(&self) {
        println!("{}", self.line)
    }
}
