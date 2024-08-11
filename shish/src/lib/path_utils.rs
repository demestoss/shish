use glob::MatchOptions;
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

pub(crate) fn replace_tilde(path: &str) -> String {
    let home_env = std::env::var("HOME");
    match (path.starts_with("~"), home_env) {
        (true, Ok(home)) => {
            format!("{}{}", home, path.chars().skip(1).collect::<String>())
        }
        _ => path.to_owned(),
    }
}

pub(crate) fn expand_glob(path: &str) -> anyhow::Result<Vec<String>> {
    let paths = glob::glob_with(
        path,
        MatchOptions {
            case_sensitive: false,
            require_literal_leading_dot: false,
            require_literal_separator: false,
        },
    )?;

    Ok(paths
        .flatten()
        .map(|p| p.into_os_string().into_string().unwrap())
        .collect::<Vec<_>>())
}
