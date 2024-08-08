use anyhow::Context;
use assert_cmd::Command;
use std::fs;

#[test]
fn runs() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.arg("Hello").assert().success().stdout("Hello\n");
    Ok(())
}

#[test]
fn runs_no_args() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.assert().success().stdout("\n");
    Ok(())
}

#[test]
fn hello1() -> anyhow::Result<()> {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> anyhow::Result<()> {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> anyhow::Result<()> {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> anyhow::Result<()> {
    run(&["Hello", "there", "-n"], "tests/expected/hello2.n.txt")
}

fn run(args: &[&str], expected_file: &str) -> anyhow::Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echo")?;
    let output = cmd
        .args(args)
        .output()
        .context("failed to execute command")?;

    let stdout = String::from_utf8(output.stdout).context("invalid UTF-8")?;
    assert_eq!(stdout, expected);
    Ok(())
}
