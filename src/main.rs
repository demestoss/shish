use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::{exit, Command};
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    loop {
        print_line_start()?;
        let command = get_user_input()?;
        handle_user_input(&command)?;
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

fn handle_user_input(input: &str) -> Result<(), io::Error> {
    let input = input.trim();
    let (command, command_args) = input.split_once(" ").unwrap_or((input, ""));

    match command {
        "" => {}
        "echo" => println!("{command_args}"),
        "exit" => handle_exit_command(command_args),
        "type" => handle_type_command(command_args),
        "pwd" => handle_pwd_command(),
        _ => match find_command_path(command) {
            Some(path) => {
                let output = Command::new(path)
                    .args(command_args.split_whitespace())
                    .output()
                    .expect("failed to execute process");

                io::stdout().write_all(&output.stdout)?;
                io::stderr().write_all(&output.stderr)?;
            }
            None => println!("{command}: command not found"),
        },
    };
    Ok(())
}

fn handle_exit_command(args: &str) {
    let status_code = i8::from_str(args).unwrap_or_default();
    exit(status_code as i32);
}

fn handle_type_command(args: &str) {
    args.split_whitespace().for_each(|param| match param {
        "" => {}
        "echo" | "exit" | "type" | "pwd" => println!("{param} is a shell builtin"),
        command => match find_command_path(command) {
            Some(path) => println!("{command} is {path}"),
            None => println!("{param}: not found"),
        },
    })
}

fn find_command_path(command: &str) -> Option<String> {
    let path_env = env::var("PATH").ok()?;
    path_env.split(':').find_map(|dir| {
        let path = format!("{dir}/{command}");
        Path::new(&path).exists().then_some(path)
    })
}

fn handle_pwd_command() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("{err}"),
    }
}
