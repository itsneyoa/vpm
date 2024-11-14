use crate::arguments::{Args, Command};
use anyhow::{bail, Context};
use colored::Colorize;

mod alternatives;
mod install;
mod query;
mod reconfigure;
mod remove;

type Result<O = std::process::Output> = anyhow::Result<O>;

pub fn run(args: Args) -> Result<u8> {
    if args.command.root_needed() && (unsafe { libc::geteuid() } != 0) {
        bail!("This operation needs super-user privileges. Exiting.")
    }

    match args.command {
        Command::Sync => install::sync(),
        Command::Update => install::update(),
        Command::ListRepos => query::list_repos(),
        Command::AddRepo { repos } => install::add_repo(repos),
        Command::Info { package } => query::info(package),
        Command::FileList { package } => query::file_list(package),
        Command::Dependencies { package } => query::dependencies(package),
        Command::Reverse { package } => query::reverse(package),
        Command::Search { name } => query::search(name),
        Command::SearchFile { file } => query::search_file(file),
        Command::List => query::list(),
        Command::Install { packages } => install::install(packages),
        Command::DevInstall { packages } => install::dev_install(packages),
        Command::ListAlternatives { package, groups } => {
            alternatives::list_alternatives(package, groups)
        }
        Command::SetAlternative { package, groups } => {
            alternatives::set_alternatives(package, groups)
        }
        Command::Reconfigure { package } => reconfigure::reconfigure(package),
        Command::ForceInstall { packages } => install::force_install(packages),
        Command::Remove { packages } => remove::remove(packages),
        Command::RemoveRecursive { packages } => remove::remove_recursive(packages),
        Command::Cleanup => remove::cleanup(),
        Command::Autoremove => remove::autoremove(),
        Command::WhatProvides { file } => {
            println!("Relaying to xlocate - use xlocate -S to (re-)build cached DB.");
            let res = wrap_command(&["xlocate", &file]);

            if res.is_err() {
                println!("xlocate not found. Try installing the xtools package.");
            }

            res
        }
        Command::External(args) => {
            let xbps_cmd = format!("xbps-{}", args[0]);

            wrap_command(
                &[&xbps_cmd]
                    .into_iter()
                    .chain(&args[1..])
                    .map(AsRef::as_ref)
                    .collect::<Vec<_>>(),
            )
        }
    }
    .map(|output| output.status.code().unwrap_or(-1) as u8)
}

pub fn wrap_command(args: &[&str]) -> Result {
    let mut cmd = std::process::Command::new(args[0]);
    cmd.args(&args[1..]);

    let output = cmd
        .output()
        .context(format!("Failed to execute command {cmd:?}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let prefix = if *crate::config::VERBOSE.get().expect("Verbose not set") {
        format!("[{}] ", args.join(" ")).bright_black()
    } else {
        "".into()
    };

    if !stdout.is_empty() {
        for line in stdout.lines() {
            println!("{prefix}{line}")
        }
    }

    if !stderr.is_empty() {
        for line in stderr.lines() {
            println!("{prefix}{} {line}", "[Err]".red(), line = line.red())
        }
    }

    println!("{prefix}Returned with code {}", {
        let status = output.status.code().unwrap_or(-1);

        if status == 0 {
            status.to_string().green()
        } else {
            status.to_string().red()
        }
        .bold()
    });

    Ok(output)
}

impl Command {
    pub fn root_needed(&self) -> bool {
        match self {
            Self::Sync => true,
            Self::Update => true,
            Self::ListRepos => false,
            Self::AddRepo { .. } => true,
            Self::Info { .. } => false,
            Self::FileList { .. } => false,
            Self::Dependencies { .. } => false,
            Self::Reverse { .. } => false,
            Self::Search { .. } => false,
            Self::SearchFile { .. } => false,
            Self::List => false,
            Self::Install { .. } => true,
            Self::DevInstall { .. } => true,
            Self::ListAlternatives { .. } => false,
            Self::SetAlternative { .. } => true,
            Self::Reconfigure { .. } => true,
            Self::ForceInstall { .. } => true,
            Self::Remove { .. } => true,
            Self::RemoveRecursive { .. } => true,
            Self::Cleanup => true,
            Self::Autoremove => true,
            Self::WhatProvides { .. } => false,
            Self::External(_) => false,
        }
    }
}
