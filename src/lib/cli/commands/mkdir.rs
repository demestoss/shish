use crate::cli::path_utils::replace_home_dir;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub(crate) struct Command {
    path: PathBuf,
}

impl Command {
    pub(crate) fn execute(&self) -> anyhow::Result<()> {
        let path = replace_home_dir(self.path.as_path())?;
        println!("{}", path.display());
        Ok(())
    }
}
