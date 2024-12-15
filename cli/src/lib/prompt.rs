use buildin::Retrieve;
use std::io::{self, Write};

pub fn print() -> anyhow::Result<()> {
    print_current_dir()?;
    print!(" > ");
    io::stdout().flush()?;
    Ok(())
}

fn print_current_dir() -> anyhow::Result<()> {
    let pwd_command = buildin_pwd::Command::new();
    let path = pwd_command.retrieve()?;
    print!("{}", path.display());
    Ok(())
}
