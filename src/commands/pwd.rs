use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct Command {}

impl Command {
    pub(crate) fn execute(&self) -> anyhow::Result<()> {
        let path = std::env::current_dir()?;
        println!("{}", path.display());
        Ok(())
    }
}
