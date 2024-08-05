use clap::Parser;
use colorful::{Colorful, RGB};
use rand::Rng;

#[derive(Parser, Debug)]
pub(crate) struct Command {
    #[clap(short = 'r')]
    random_color: bool,

    line: Vec<String>,
}

impl Command {
    pub(crate) fn execute(&self) {
        if self.random_color {
            let (r, g, b) = random_color();
            println!("{}", self.line.join(" ").gradient(RGB::new(r, g, b)));
        } else {
            println!("{}", self.line.join(" "));
        }
    }
}

fn random_color() -> (u8, u8, u8) {
    let mut rng = rand::thread_rng();
    (rng.gen(), rng.gen(), rng.gen())
}
