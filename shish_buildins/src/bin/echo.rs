use clap::Parser;
use colorful::{Colorful, RGB};
use rand::Rng;

#[derive(Parser, Debug)]
#[command(about = "Prints text to the stdout")]
pub struct Command {
    #[clap(short = 'r', help = "Prints in random color")]
    random_color: bool,
    #[clap(short = 'n', help = "Omit print newline")]
    no_newline: bool,

    line: Vec<String>,
}

impl Command {
    pub fn invoke(&self) {
        let output = self.line.join(" ");
        let output = if self.random_color {
            let (r, g, b) = random_color();
            output.gradient(RGB::new(r, g, b)).to_string()
        } else {
            output
        };

        if self.no_newline {
            print!("{}", output);
        } else {
            println!("{}", output);
        }
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
