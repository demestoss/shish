use clap::Parser;

#[derive(Parser)]
#[command(about = "Always returns failure 1 exit code")]
struct Command {}

pub fn main() {
    let _ = Command::parse();
    std::process::abort();
}
