use super::{wrap_command, Result};

pub fn reconfigure(package: String) -> Result {
    wrap_command(&["xbps-reconfigure", "-v", &package])
}
