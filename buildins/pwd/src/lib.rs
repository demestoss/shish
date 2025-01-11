use clap::Parser;

#[derive(Parser, Debug)]
pub struct Command {}

impl Command {
    pub fn invoke(&self) -> anyhow::Result<()> {
        let path = std::env::current_dir()?;
        println!("{}", path.display());
        Ok(())
    }
}
