use clap::Parser;
use std::path::PathBuf;
use crate::path_utils::replace_home_dir;

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
