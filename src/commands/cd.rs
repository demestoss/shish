use crate::path_utils::check_path_exists;
use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct Command {
    path: String,
}

impl Command {
    pub(crate) fn execute(&self) -> anyhow::Result<()> {
        if self.path.is_empty() {
            return Ok(());
        }

        let path = self.replace_home_dir()?;

        match check_path_exists(&path) {
            Some(path) => std::env::set_current_dir(path)?,
            None => println!("cd: {path}: No such file or directory"),
        };
        Ok(())
    }

    fn replace_home_dir(&self) -> anyhow::Result<String> {
        match self.path.starts_with('~') {
            true => {
                let home_dir = std::env::var("HOME")?;
                Ok(home_dir + &self.path[1..])
            }
            false => Ok(self.path.to_string()),
        }
    }
}
