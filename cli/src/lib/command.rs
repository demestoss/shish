use anyhow::bail;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::process;
use std::process::{Child, Stdio};
use std::str::FromStr;

pub(crate) fn execute_command(
    args: &[String],
    previous: Option<Child>,
    stdout: Stdio,
) -> anyhow::Result<Child> {
    let command_name = &args[0];
    let args = &args[1..];
    let stdin = previous.map_or(Stdio::inherit(), |output: Child| {
        Stdio::from(output.stdout.unwrap())
    });
    if let Some(command) = find_command_in_builins(command_name) {
        spawn_command(command, args, stdin, stdout)
    } else if let Some(command) = find_command_in_path(command_name) {
        spawn_command(command, args, stdin, stdout)
    } else if let Ok(true) = is_command_valid_path(command_name) {
        spawn_command("cd", &[command_name], stdin, stdout)
    } else {
        bail!("{}: command not found", args[0])
    }
}

fn find_command_in_builins(command: &str) -> Option<PathBuf> {
    let debug_buildin_path = PathBuf::from_str("target/debug/").ok()?;
    // let buildin_name = String::from("shish-") + command;
    let buildin = debug_buildin_path.join(command);
    match buildin.try_exists() {
        Ok(true) => Some(buildin),
        _ => None,
    }
}

fn find_command_in_path(command: &str) -> Option<PathBuf> {
    let path_env = std::env::var("PATH").ok()?;
    path_env.split(':').find_map(|dir| {
        let path = Path::new(dir).join(command);
        match path.try_exists() {
            Ok(true) => Some(path),
            _ => None,
        }
    })
}

fn is_command_valid_path(command: &str) -> Result<bool, std::io::Error> {
    let command = if command.eq("...") { "../.." } else { command };
    let current_dir = current_dir()?;
    let path = current_dir.join(command);
    path.try_exists()
}

fn spawn_command<A: AsRef<Path>>(
    command: A,
    args: &[impl AsRef<Path>],
    stdin: Stdio,
    stdout: Stdio,
) -> anyhow::Result<Child> {
    let command = command.as_ref();
    let output = process::Command::new(command)
        .args(args.iter().map(|v| v.as_ref()))
        .stdin(stdin)
        .stdout(stdout)
        .spawn();
    match output {
        Ok(output) => Ok(output),
        Err(e) => {
            bail!("{}: {}", command.display(), e)
        }
    }
}
