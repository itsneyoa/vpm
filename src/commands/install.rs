use super::{wrap_command, Result};

pub fn sync() -> Result {
    println!("Synchronizing remote repository data");
    wrap_command(&["xbps-install", "-S"])
}

pub fn update() -> Result {
    println!("Running system update");
    wrap_command(&["xbps-install", "-Suv"])
}

pub fn add_repo(repos: Vec<String>) -> Result {
    for repo in repos {
        println!(r#"Adding repository: "{repo}""#);
        wrap_command(&["xbps-install", &repo])?;
    }
    println!();

    println!("Synchronizing remote repository data");
    wrap_command(&["xbps-install", "-S"])
}

pub fn install(packages: Vec<String>) -> Result {
    println!("Installing {}", packages.join(", "));

    wrap_command(
        &["xbps-install", "-S"]
            .into_iter()
            .chain(packages.iter().map(AsRef::as_ref))
            .collect::<Vec<_>>(),
    )
}

pub fn dev_install(packages: Vec<String>) -> Result {
    let packages = packages
        .into_iter()
        .flat_map(|pkg| [pkg.clone(), format!("{pkg}-devel")])
        .collect();

    install(packages)
}

pub fn force_install(packages: Vec<String>) -> Result {
    println!("Force installing {}", packages.join(", "));

    wrap_command(
        &["xbps-install", "-Sf"]
            .into_iter()
            .chain(packages.iter().map(AsRef::as_ref))
            .collect::<Vec<_>>(),
    )
}
