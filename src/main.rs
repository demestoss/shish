#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    loop {
        print_line_start()?;
        let command = get_user_input()?;
        handle_user_input(&command);
    }
}

fn print_line_start() -> Result<(), io::Error> {
    print!("$ ");
    io::stdout().flush()
}

fn get_user_input() -> Result<String, io::Error> {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    Ok(input)
}

fn handle_user_input(input: &str) {
    let input = input.trim();
    let (command_name, command_args) = input.split_once(" ").unwrap_or((input, ""));

    match command_name {
        "echo" => println!("{command_args}"),
        "exit" => {
            let status_code = i8::from_str(command_args).unwrap_or_default();
            exit(status_code as i32);
        }
        "type" => handle_type_command(command_args),
        "" => (),
        _ => println!("{command_name}: command not found"),
    };
}

fn handle_type_command(params: &str) {
    params.split(' ').for_each(|param| match param {
        "echo" | "exit" | "type" => println!("{param} is a shell builtin"),
        _ => println!("{param}: not found"),
    })
}
