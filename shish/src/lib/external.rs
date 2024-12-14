use crate::path_utils::find_command_path;
use anyhow::bail;
use std::process;
use std::process::{Child, Stdio};

pub(crate) fn execute_external_command(
    args: &[String],
    previous: Option<Child>,
    stdout: Stdio,
) -> anyhow::Result<Child> {
    let Some(command) = find_command_path(&args[0]) else {
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
            bail!("{}: {}", args[0], e)
        }
    }
}
