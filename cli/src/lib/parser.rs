use glob::MatchOptions;

pub(crate) fn args(input: &str) -> anyhow::Result<Vec<String>> {
    let args = shlex::split(input).unwrap_or_default();
    let mut parsed_args = Vec::new();

    for arg in args {
        let arg = if arg.starts_with('~') {
            replace_tilde(&arg)
        } else {
            arg
        };
        if arg.contains('*') {
            match expand_glob(&arg) {
                Ok(glob_args) => parsed_args.extend(glob_args),
                Err(_) => parsed_args.push(arg),
            }
        } else {
            parsed_args.push(arg)
        }
    }

    Ok(parsed_args)
}

fn replace_tilde(path: &str) -> String {
    let home_env = std::env::var("HOME");
    match (path.starts_with("~"), home_env) {
        (true, Ok(home)) => {
            format!("{}{}", home, path.chars().skip(1).collect::<String>())
        }
        _ => path.to_owned(),
    }
}

fn expand_glob(path: &str) -> anyhow::Result<Vec<String>> {
    let paths = glob::glob_with(
        path,
        MatchOptions {
            case_sensitive: false,
            require_literal_leading_dot: false,
            require_literal_separator: false,
        },
    )?;

    Ok(paths
        .flatten()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>())
}
