use clap::Parser;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(about, author, version)]
/// Shish version of head utility to display first lines of a file
pub struct Command {
    /// Number of lines
    #[arg(short = 'n', long, default_value_t = 10, value_parser = clap::value_parser!(u64).range(1..))]
    lines: u64,

    /// Number of bytes
    #[arg(short = 'c', long, conflicts_with = "lines", value_parser = clap::value_parser!(u64).range(1..))]
    bytes: Option<u64>,

    /// Input file(s)
    #[arg(default_value = "-")]
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
