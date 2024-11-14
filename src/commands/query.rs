use anyhow::Context;
use colored::Colorize;

use super::{wrap_command, Result};

pub fn list_repos() -> Result {
    println!("Configured repositories: ");
    let configured = wrap_command(&["xbps-query", "-v", "-L"])?;
    println!();

    println!("Available sub-repositories");
    let available = wrap_command(&["xbps-query", "-v", "-Rs", "void-repo"])?;
    println!();

    println!(
        r#"Use "{} addrepo <repository>" to add a sub-repository."#,
        env!("CARGO_BIN_NAME")
    );

    Ok(if configured.status.code().is_some_and(|code| code == 0) {
        available
    } else {
        configured
    })
}

pub fn info(package: String) -> Result {
    wrap_command(&["xbps-query", "-v", "-R", &package])
}

pub fn file_list(package: String) -> Result {
    wrap_command(&["xbps-query", "-v", "-R", "-f", &package])
}

pub fn dependencies(package: String) -> Result {
    wrap_command(&["xbps-query", "-v", "-R", "-x", &package])
}

pub fn reverse(package: String) -> Result {
    wrap_command(&["xbps-query", "-v", "-R,", "-X", &package])
}

pub fn search(name: String) -> Result {
    wrap_command(&["xbps-query", "-v", "-Rs", &name])
}

pub fn search_file(file: String) -> Result {
    wrap_command(&["xbps-query", "-v", "-o", &format!("*/{file}")])
}

pub fn list() -> Result {
    #[derive(Debug)]
    struct InstalledPackage<'a> {
        name: &'a str,
        version: &'a str,
        state: PackageState,
        description: &'a str,
    }

    impl<'a> std::fmt::Display for InstalledPackage<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{name} ({version}) [{state}] - {description}",
                name = self.name.blue(),
                version = self.version.green(),
                state = self.state.to_string().yellow(),
                description = self.description
            )
        }
    }

    #[derive(Debug)]
    enum PackageState {
        Installed,
        Unpacked,
        HalfRemoved,
        Unknown,
    }

    impl TryFrom<&str> for PackageState {
        type Error = anyhow::Error;

        fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
            match value {
                "ii" => Ok(Self::Installed),
                "uu" => Ok(Self::Unpacked),
                "hr" => Ok(Self::HalfRemoved),
                "??" => Ok(Self::Unknown),
                _ => anyhow::bail!("Unknown package state"),
            }
        }
    }

    impl std::fmt::Display for PackageState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Self::Installed => "Installed",
                    Self::Unpacked => "Unpacked",
                    Self::HalfRemoved => "Half Removed",
                    Self::Unknown => "Unknown",
                }
            )
        }
    }

    println!("Installed packages:");

    let mut cmd = std::process::Command::new("xbps-query");
    cmd.args(["-v", "-l"]);

    let output = cmd.output()?;

    let package_strings = String::from_utf8_lossy(&output.stdout);
    let mut packages = Vec::with_capacity(package_strings.len());

    for line in package_strings.lines() {
        let (state, line) = line.split_once(' ').context("No state or name/version")?;
        let state = PackageState::try_from(state)?;

        let (name_version, description) = line
            .split_once(' ')
            .context("No name/version or description")?;

        let (name, version) = name_version
            .rsplit_once('-')
            .context("Invalid name/version")?;

        let description = description.trim();

        packages.push(InstalledPackage {
            name,
            version,
            state,
            description,
        })
    }

    for (idx, package) in packages.iter().enumerate() {
        println!(
            "{n}    {package}",
            n = format_args!("[{}]", idx + 1).to_string().bright_black()
        )
    }

    Ok(output)
}
