use buildin_exit::Command;
use clap::Parser;

pub fn main() {
    let c = Command::parse();
    if let Err(e) = c.invoke() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
