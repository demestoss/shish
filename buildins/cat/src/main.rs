use buildin_cat::Command;
use clap::Parser;

pub fn main() {
    let c = Command::parse();
    c.invoke();
}
