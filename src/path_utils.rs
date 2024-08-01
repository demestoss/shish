use std::env;
use std::path::Path;

pub(crate) fn find_command_path(command: &str) -> Option<String> {
    let path_env = env::var("PATH").ok()?;
    path_env.split(':').find_map(|dir| {
        let path = format!("{dir}/{command}");
        check_path_exists(&path)
    })
}

pub(crate) fn check_path_exists(path: &str) -> Option<String> {
    Path::new(&path).exists().then_some(path.to_string())
}
