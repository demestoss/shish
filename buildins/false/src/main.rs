use buildin_false::Command;
use clap::Parser;

pub fn main() {
    let c = Command::parse();
    match c.invoke() {
        Ok(c) => std::process::exit(c),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
