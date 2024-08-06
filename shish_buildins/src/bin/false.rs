use clap::Parser;

pub mod echo;

#[derive(Parser)]
#[command(about = "Always returns failure 1 exit code")]
struct Command {}

impl Command {
    fn invoke(&self) {
        std::process::abort();
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
    fn false_failure() -> anyhow::Result<()> {
        let mut cmd = Command::cargo_bin("false")?;
        cmd.assert().failure();
        Ok(())
    }
}
