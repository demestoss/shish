use crate::path_utils::find_command_path;
use anyhow::bail;
use std::path::{Path, PathBuf};
use std::process::{Child, Stdio};
use std::{env, process};

pub(crate) fn execute_external_command(
    args: &[String],
    previous: Option<Child>,
    stdout: Stdio,
) -> anyhow::Result<Child> {
    let Some(command) = get_command_path(&args[0]) else {
        bail!("{}: command not found", args[0])
    };
    let stdin = previous.map_or(Stdio::inherit(), |output: Child| {
        Stdio::from(output.stdout.unwrap())
    });
    let output = process::Command::new(command)
        .args(&args[1..])
        .stdin(stdin)
        .stdout(stdout)
        .spawn();
    match output {
        Ok(output) => Ok(output),
        Err(e) => {
            bail!("{}", e)
        }
    }
}

fn get_command_path(command_name: &str) -> Option<PathBuf> {
    let path = if let Some(path) = find_buildin_path(&command_name) {
        Some(path)
    } else {
        find_command_path(&command_name)
    };

    return path;
}

fn find_buildin_path(command: &str) -> Option<PathBuf> {
    match command {
        "true" | "false" | "echo" => Some(get_buildin_path(&command)),
        _ => None,
    }
}

fn get_buildin_path(bin: &str) -> PathBuf {
    if let Ok(exec_path) = env::current_exe() {
        let mut binary_dir = exec_path
            .parent()
            .unwrap_or_else(|| Path::new("../../../.."))
            .to_path_buf();
        binary_dir.push(&bin);
        binary_dir
    } else {
        let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
        let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
        PathBuf::from(format!("{}/{}/{}", target_dir, profile, bin))
    }
}
