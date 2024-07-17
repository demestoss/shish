#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use std::{env, process};

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
    let (command, command_args) = input.split_once(" ").unwrap_or((input, ""));

    match handle_command(command, command_args) {
        Ok(_) => {}
        Err(err) => eprintln!("{command}: {err}"),
    }
}

fn handle_command(command: &str, args: &str) -> Result<(), io::Error> {
    match command {
        "" => {}
        "exit" => handle_exit_command(args),
        "type" => handle_type_command(args),
        "echo" => handle_echo_command(args),
        "pwd" => handle_pwd_command()?,
        "cd" => handle_cd_command(args)?,
        _ => handle_external_command(command, args)?,
    };
    Ok(())
}

fn handle_exit_command(args: &str) {
    let status_code = i8::from_str(args).unwrap_or_default();
    process::exit(status_code as i32);
}

fn handle_type_command(args: &str) {
    args.split_whitespace().for_each(|param| match param {
        "" => {}
        "echo" | "exit" | "type" | "pwd" | "cd" => println!("{param} is a shell builtin"),
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
        check_path_exists(&path)
    })
}

fn check_path_exists(path: &str) -> Option<String> {
    Path::new(&path).exists().then_some(path.to_string())
}

fn handle_echo_command(args: &str) {
    println!("{args}")
}

fn handle_pwd_command() -> Result<(), io::Error> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}

fn handle_cd_command(args: &str) -> Result<(), io::Error> {
    match check_path_exists(args) {
        None => println!("cd: {args}: No such file or directory"),
        Some(path) => env::set_current_dir(path)?,
    };
    Ok(())
}

fn handle_external_command(command: &str, args: &str) -> Result<(), io::Error> {
    match find_command_path(command) {
        Some(path) => {
            let output = Command::new(path).args(args.split_whitespace()).output()?;
            io::stdout().write_all(&output.stdout)?;
            io::stderr().write_all(&output.stderr)?;
        }
        None => println!("{command}: command not found"),
    };
    Ok(())
}
