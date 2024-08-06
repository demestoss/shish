use clap::Parser;
use colorful::{Colorful, RGB};
use rand::Rng;

#[derive(Parser, Debug)]
#[command(about)]
/// Prints text to the stdout
pub struct Command {
    /// Prints in random color
    #[clap(short = 'r')]
    random_color: bool,
    /// Omit print newline
    #[clap(short = 'n')]
    no_newline: bool,

    /// Input text
    text: Vec<String>,
}

impl Command {
    pub fn invoke(&self) {
        let output = self.text.join(" ");
        let output = if self.random_color {
            let (r, g, b) = random_color();
            output.gradient(RGB::new(r, g, b)).to_string()
        } else {
            output
        };

        let line_end = if self.no_newline { "" } else { "\n" };
        print!("{output}{line_end}");
    }
}

fn random_color() -> (u8, u8, u8) {
    let mut rng = rand::thread_rng();
    (rng.gen(), rng.gen(), rng.gen())
}

pub fn main() {
    let c = Command::parse();
    c.invoke();
}
