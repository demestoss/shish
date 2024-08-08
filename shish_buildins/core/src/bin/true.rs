use clap::Parser;

#[derive(Parser)]
#[command(about = "Always returns success 0 exit code")]
struct Command {}

impl Command {
    fn invoke(&self) {
        std::process::exit(0);
    }
}

pub fn main() {
    let c = Command::parse();
    c.invoke();
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn true_success() -> anyhow::Result<()> {
        let mut cmd = Command::cargo_bin("true")?;
        cmd.assert().success();
        Ok(())
    }
}
