use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, author, version)]
/// Make directory in the current directory
pub struct Command {
    /// Create nested directories if it was provided in the name
    #[arg(short = 'p', default_value_t = true)]
    parent: bool,

    /// The name(s) of the directory(ies) to create
    dirs: Vec<String>,
}

impl Command {
    pub fn invoke(&self) -> anyhow::Result<()> {
        for dir in self.dirs.iter() {
            if std::fs::metadata(dir).is_ok() {
                eprintln!("mkdir: Directory already exists: {dir}");
                continue;
            }

            let res = if self.parent {
                std::fs::create_dir_all(dir)
            } else {
                std::fs::create_dir(dir)
            };

            if let Err(e) = res {
                eprintln!("mkdir: Failed to create directory: {dir}; {e}");
            }
        }

        Ok(())
    }
}
