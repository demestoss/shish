use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    loop {
        print_line_start()?;
        let command = get_user_input()?;
        match shish::cli::handle_user_input(&command) {
            Err(e) => eprintln!("{e}"),
            Ok(_) => {}
        }
    }
}

fn print_line_start() -> Result<(), io::Error> {
    print!("€ ");
    io::stdout().flush()
}

fn get_user_input() -> Result<String, io::Error> {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    Ok(input)
}
