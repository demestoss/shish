use clap::Parser;

#[derive(Parser, Debug)]
pub struct Command {}

impl Command {
    pub fn invoke(&self) -> anyhow::Result<i32> {
        let path = std::env::current_dir()?;
        println!("{}", path.display());
        Ok(0)
    }
}
