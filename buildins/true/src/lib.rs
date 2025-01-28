use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Always returns success 0 status code")]
pub struct Command {}

impl Command {
    pub fn invoke(&self) {
        std::process::exit(0);
    }
}
