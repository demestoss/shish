use buildin_exit::Command;
use clap::Parser;

pub fn main() {
    let c = Command::parse();
    c.invoke();
}
