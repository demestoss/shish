use clap::Parser;

pub mod echo;

#[derive(Parser)]
#[command(about = "Always returns failure 1 exit code")]
struct Command {}

impl Command {
    fn invoke(&self) {
        std::process::abort();
    }
}

pub fn main() {
    let c = Command::parse();
    c.invoke();
}
