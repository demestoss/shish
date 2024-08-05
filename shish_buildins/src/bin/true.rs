use clap::Parser;

#[derive(Parser)]
#[command(about = "Always returns success 0 exit code")]
struct Command {}

impl Command {
    fn invoke(&self) {
        std::process::exit(0);
    }
}

pub fn main() {
    let c = Command::parse();
    c.invoke();
}
