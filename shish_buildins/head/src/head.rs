use clap::Parser;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(about, author, version)]
/// Shish version of head util to display first lines of a file
pub struct Command {
    /// Number of lines
    #[arg(short = 'n', long = "lines", default_value_t = 10)]
    lines: u64,
    /// Print line number only for non-blank lines
    #[arg(short = 'b', long = "number-nonblank", conflicts_with = "number_lines")]
    bytes: Option<u64>,

    /// Input file(s)
    #[arg(default_values_t = ["-".to_string()])]
    files: Vec<String>,
}

impl Command {
    pub fn invoke(&self) -> anyhow::Result<()> {

        Ok(())
    }
}

fn open(filename: &str) -> anyhow::Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn main() {
    let c = Command::parse();
    if let Err(e) = c.invoke() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
