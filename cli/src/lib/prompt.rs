use buildin::Retrieve;
use std::io::{self, Write};

use crate::parser::replace_home_with_tilde;

pub fn prompt() -> anyhow::Result<()> {
    print_current_dir()?;
    print!(" > ");
    io::stdout().flush()?;
    Ok(())
}

fn print_current_dir() -> anyhow::Result<()> {
    let pwd_command = buildin_pwd::Command::new();
    let path = pwd_command.retrieve()?;
    let path = replace_home_with_tilde(&path);
    let name = path.file_name().unwrap_or_default();
    print!("{}", name.to_str().unwrap());
    Ok(())
}
