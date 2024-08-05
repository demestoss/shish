use clap::Parser;

#[derive(Parser)]
#[command(about = "Always returns success 0 exit code")]
struct Command {}

pub fn main() {
    let _ = Command::parse();
    std::process::exit(0);
}
