use std::env;
use std::path::{Path, PathBuf};

pub(crate) fn find_command_path(command: &str) -> Option<PathBuf> {
    let path_env = env::var("PATH").ok()?;
    path_env.split(':').find_map(|dir| {
        let path = Path::new(dir).join(command);
        match path.try_exists() {
            Ok(true) => Some(path),
            _ => None,
        }
    })
}

pub(crate) fn replace_home_dir(path: &Path) -> anyhow::Result<PathBuf> {
    match path.starts_with("~") {
        true => {
            let mut home_dir = PathBuf::from(std::env::var("HOME")?);
            home_dir.extend(path.iter().skip(1));
            Ok(home_dir)
        }
        false => Ok(path.to_owned()),
    }
}
