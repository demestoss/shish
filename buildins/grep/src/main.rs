use buildin_grep::Command;
use clap::Parser;

fn main() {
    let command = Command::parse();
    if let Err(e) = command.invoke() {
        eprintln!("{e}");
        std::process::exit(1);
    };
}
