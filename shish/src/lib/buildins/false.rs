use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Always returns failure 1 exit code")]
pub(crate) struct Command {}

impl Command {
    pub(crate) fn execute(&self) -> anyhow::Result<i32> {
        Ok(1)
    }
}
