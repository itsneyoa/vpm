use super::{wrap_command, Result};

pub fn list_alternatives(package: String, groups: Vec<String>) -> Result {
    // TODO: Print description of what's going to happen

    let mut args = vec!["xbps-alternatives", "-l", &package];
    let groups = groups.join(",");

    if !groups.is_empty() {
        args.push("-g");
        args.push(&groups);
    }

    wrap_command(&args)
}

pub fn set_alternatives(package: String, groups: Vec<String>) -> Result {
    // TODO: Print description of what's going to happen

    let mut args = vec!["xbps-alternatives", "-l", &package];
    let groups = groups.join(",");

    if !groups.is_empty() {
        args.push("-g");
        args.push(&groups);
    }

    wrap_command(&args)
}
