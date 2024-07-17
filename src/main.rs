#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    loop {
        print_line_start()?;
        let command = get_user_command()?;
        handle_user_command(&command);
    }
}

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

fn handle_user_command(command: &str) {
    let command = command.trim();
    let (command_name, command_params) = command.split_once(" ").unwrap_or((command, ""));

    match command_name {
        "" => {}
        "echo" => {
            println!("{command_params}");
        }
        "exit" => {
            let status_code = i8::from_str(command_params);
            match status_code {
                Ok(code) => exit(code as i32),
                _ => exit(0),
            }
        }
        _ => {
            println!("{command_name}: command not found");
        }
    }
}
