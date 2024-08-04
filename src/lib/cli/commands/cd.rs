use crate::cli::path_utils::replace_home_dir;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub(crate) struct Command {
    path: String,
}

impl Command {
    pub(crate) fn execute(&self) -> anyhow::Result<()> {
        if self.path.is_empty() {
            return Ok(());
        }

        let path = PathBuf::from(&self.path);
        let path = replace_home_dir(&path)?;
        let path_exists = path.try_exists()?;

        if path_exists {
            std::env::set_current_dir(path)?;
        } else {
            println!("cd: {}: No such file or directory", path.display());
        }
        Ok(())
    }
}
