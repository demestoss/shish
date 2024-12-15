use buildin::Invoke;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Always returns failure 1 exit code")]
pub struct Command {}

impl Invoke for Command {
     fn invoke(&self) -> anyhow::Result<i32> {
        Ok(1)
    }
}
