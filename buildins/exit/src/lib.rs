use clap::Parser;

#[derive(Parser, Debug)]
pub struct Command {
    status_code: Option<i32>,
}

impl Command {
    pub fn invoke(&self) {
        let status_code = self.status_code.unwrap_or(0);
        std::process::exit(status_code);
    }
}
