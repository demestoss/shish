use buildin::Invoke;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Command {
    path: String,
}

impl Invoke for Command {
    fn invoke(&self) -> anyhow::Result<i32> {
        if self.path.is_empty() {
            return Ok(0);
        }

        let path = PathBuf::from(&self.path);
        let path_exists = path.try_exists()?;

        if path_exists {
            std::env::set_current_dir(path)?;
            Ok(0)
        } else {
            eprintln!("cd: {}: No such file or directory", path.display());
            Ok(1)
        }
    }
}
