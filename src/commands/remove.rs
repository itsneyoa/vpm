use super::{wrap_command, Result};

pub fn remove(packages: Vec<String>) -> Result {
    wrap_command(
        &["xbps-remove", "-v"]
            .into_iter()
            .chain(packages.iter().map(AsRef::as_ref))
            .collect::<Vec<_>>(),
    )
}

pub fn remove_recursive(packages: Vec<String>) -> Result {
    wrap_command(
        &["xbps-remove", "-v", "-R"]
            .into_iter()
            .chain(packages.iter().map(AsRef::as_ref))
            .collect::<Vec<_>>(),
    )
}

pub fn cleanup() -> Result {
    wrap_command(&["xbps-remove", "-v", "-O"])
}

pub fn autoremove() -> Result {
    wrap_command(&["xbps-remove", "-v", "-o"])
}
