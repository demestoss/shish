use crate::commands;
use clap::error::ErrorKind;
use clap::Parser;

#[derive(Debug, Parser)]
enum SpecialBuildin {
    Exit(commands::exit::Command),
    Type(commands::r#type::Command),
    Pwd(commands::pwd::Command),
    Cd(commands::cd::Command),
    Mkdir(commands::mkdir::Command),
}

impl SpecialBuildin {
    fn execute(&self) -> anyhow::Result<()> {
        match self {
            SpecialBuildin::Cd(c) => c.execute(),
            SpecialBuildin::Exit(c) => Ok(c.execute()),
            SpecialBuildin::Type(c) => Ok(c.execute()),
            SpecialBuildin::Pwd(c) => c.execute(),
            SpecialBuildin::Mkdir(c) => c.execute(),
        }
    }
}

pub fn handle_user_input(input: &str) {
    let input = input.trim();

    if input.is_empty() {
        return;
    }

    let (command, _) = input.split_once(" ").unwrap_or((input, ""));

    let mut args = vec!["shish".to_owned()];
    args.extend(shlex::split(input).unwrap_or(Vec::new()));

    let parsed_command = SpecialBuildin::try_parse_from(&args);

    let command_result = match parsed_command {
        Ok(c) => {
            let result = c.execute();
            result
        }
        Err(err) => match err.kind() {
            ErrorKind::DisplayHelp => {
                println!("{}", err);
                Ok(())
            }
            _ => {
                let _ = err.exit_code();

                commands::external::Command::new(&input)
                    .execute()
                    .map(|_| ())
            }
        },
    };

    match command_result {
        Ok(_) => {}
        Err(err) => eprintln!("{command}: {err}"),
    }
}
