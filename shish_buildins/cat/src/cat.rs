use clap::Parser;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(about, author, version)]
/// Shish version of cat util to concat and print files
pub struct Command {
    /// Print the line numbers at the start of the line
    #[arg(short = 'n', long = "number")]
    number_lines: bool,
    /// Print line number only for non-blank lines
    #[arg(short = 'b', long = "number-nonblank", conflicts_with = "number_lines")]
    number_nonblank_lines: bool,

    /// Input file(s)
    #[arg(default_values_t = ["-".to_string()])]
    files: Vec<String>,
}

impl Command {
    pub fn invoke(&self) -> anyhow::Result<()> {
        for filename in self.files.iter() {
            match open(&filename) {
                Err(e) => eprintln!("failed to open {filename}: {e}"),
                Ok(buf) => {
                    let mut line_idx = 0;
                    for line in buf.lines().map(|l| l.unwrap()) {
                        let is_empty = line.trim().is_empty();
                        if self.number_nonblank_lines && is_empty {
                            println!();
                            continue;
                        }
                        line_idx += 1;
                        if self.number_lines || self.number_nonblank_lines {
                            print!("{line_idx:>6}{}", if is_empty { "" } else { "  " })
                        }
                        println!("{line}");
                    }
                }
            }
        }
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
