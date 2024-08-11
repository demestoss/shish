use crate::path_utils::{expand_glob, replace_tilde};

pub(crate) fn args(input: &str) -> anyhow::Result<Vec<String>> {
    let args = shlex::split(input).unwrap_or(Vec::new());
    let mut parsed_args = Vec::new();

    for arg in args {
        let arg = if arg.starts_with('~') {
            replace_tilde(&arg)
        } else {
            arg
        };
        if arg.contains('*') {
            let glob_args = expand_glob(&arg)?;
            parsed_args.extend(glob_args);
        } else {
            parsed_args.push(arg)
        }
    }

    Ok(parsed_args)
}
