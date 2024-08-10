use crate::path_utils::replace_home_dir;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub(crate) struct Command {
    path: PathBuf,
}

impl Command {
    pub(crate) fn execute(&self) -> anyhow::Result<i32> {
        let path = replace_home_dir(self.path.as_path())?;
        println!("{}", path.display());
        Ok(0)
    }
}
