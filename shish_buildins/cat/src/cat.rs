use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(about, author, version)]
/// Shish version of cat util to concat and print files
pub struct Command {
    #[command(flatten)]
    number_args: NumberArgs,

    /// Input file(s)
    #[arg(default_values_t = ["-".to_string()])]
    files: Vec<String>,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct NumberArgs {
    /// Print the line numbers at the start of the line
    #[arg(short = 'n')]
    number_lines: bool,
    /// Print line number only for non-blank lines
    #[arg(short = 'b')]
    number_nonblank_lines: bool,
}

impl Command {
    pub fn invoke(&self) -> anyhow::Result<()> {
        for filename in self.files.iter() {
            println!("{filename}");
        }
        Ok(())
    }
}

pub fn main() {
    let c = Command::parse();
    if let Err(e) = c.invoke() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
