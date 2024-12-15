use buildin::Invoke;
use std::process;

use buildin_grepr::Command;
use clap::Parser;

fn main() {
    let command = Command::parse();
    match command.invoke() {
        Ok(code) => process::exit(code),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
