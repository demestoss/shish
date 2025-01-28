use anyhow::bail;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Command {
    path: String,
}

impl Command {
    pub fn invoke(&self) -> anyhow::Result<()> {
        if self.path.is_empty() {
            return Ok(());
        }

        let path = PathBuf::from(&self.path);
        let path_exists = path.try_exists()?;

        if path_exists {
            std::env::set_current_dir(path)?;
            Ok(())
        } else {
            bail!("No such file or directory: {}", path.display())
        }
    }
}
