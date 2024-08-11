use crate::path_utils;

pub(crate) fn args(input: &str) -> anyhow::Result<Vec<String>> {
    let args = shlex::split(input).unwrap_or(Vec::new());
    let mut parsed_args = Vec::new();

    for arg in args {
        let arg = if arg.starts_with('~') {
            path_utils::replace_tilde(&arg)
        } else {
            arg
        };
        if arg.contains('*') {
            match path_utils::expand_glob(&arg) {
                Ok(glob_args) => parsed_args.extend(glob_args),
                Err(_) => parsed_args.push(arg),
            }
        } else {
            parsed_args.push(arg)
        }
    }

    Ok(parsed_args)
}
