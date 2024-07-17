#[allow(unused_imports)]
use std::io::{self, Write};

fn print_line_start() -> Result<(), io::Error> {
    print!("$ ");
    io::stdout().flush()
}

fn get_user_command() -> Result<String, io::Error> {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    Ok(input)
}

fn main() -> Result<(), io::Error> {
    loop {
        print_line_start()?;
        let command = get_user_command()?;

        println!("{}: command not found", command.trim_end());
    }
}
