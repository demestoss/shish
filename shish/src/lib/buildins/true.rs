use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Always returns success 0 exit code")]
pub(crate) struct Command {}

impl Command {
    pub(crate) fn execute(&self) -> anyhow::Result<i32> {
        Ok(0)
    }
}
