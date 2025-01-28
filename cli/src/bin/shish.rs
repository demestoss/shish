use std::io::{self};

fn main() -> anyhow::Result<()> {
    loop {
        shish::prompt()?;
        let command = get_user_input()?;
        if let Err(e) = shish::handle_user_input(&command) {
            eprintln!("{e}")
        }
    }
}

fn get_user_input() -> Result<String, io::Error> {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    Ok(input)
}
