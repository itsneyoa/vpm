use clap::{ColorChoice, Parser, Subcommand};

/// vpm - void package management utility for XBPS
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Subcommand
    #[clap(subcommand)]
    pub command: Command,
    /// Enable/Disable colorized output
    #[clap(short, long, default_value_t)]
    pub color: ColorChoice,
    /// Show XBPS command translations during execution
    #[clap(short, long, default_value_t)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Synchronize remote repository data
    ///
    /// Will execute `xbps-install -S`
    Sync,
    /// Update the system
    ///
    /// Will execute `xbps-install -Sduv`
    #[command(visible_alias = "up", alias = "upgrade")]
    Update,
    /// List configured repositories
    ///
    /// Will execute `xbps-query -v -L`
    #[command(visible_aliases = ["lr", "repo-list","rl"], aliases = ["listrepos", "repolist"])]
    ListRepos,
    /// Add an additional repository
    ///
    /// Will execute `xbps-install <repos>`
    AddRepo {
        #[arg(num_args(1..))]
        repos: Vec<String>,
    },
    /// Show information about <package>
    ///
    /// Will execute `xbps-query-v -R <package>`
    Info { package: String },
    /// Show file list of <package>
    ///
    /// Will execute `xbps-query -v -R -f <package>`
    #[command(visible_alias = "fl")]
    FileList { package: String },
    /// Show dependencies for <package>
    ///
    /// Will execute `xbps-query -v -R -x <package>`
    #[command(visible_alias = "deps")]
    Dependencies { package: String },
    /// Show reverse dependendies of <package> (see man xbps-query)
    ///
    /// Will execute `xbps-query -v -R -X`
    #[command(visible_alias = "rv")]
    Reverse { package: String },
    /// Search for package by <name>
    ///
    /// Will execute `xbps-query -v -Rs <name>`
    #[command(visible_alias = "s")]
    Search { name: String },
    /// Search for package containing <file> (local)
    ///
    /// Will execute `xbps-query -v -o "*/<file>"`
    #[command(visible_alias = "sf", alias = "searchfile")]
    SearchFile { file: String },
    /// List installed packages
    ///
    /// Will execute `xbps-query -v -l`
    #[command(visible_alias = "ls")]
    List,
    /// Install <package(s)>
    ///
    /// Will execute `xbps-install -S <package>`
    #[command(visible_alias = "i")]
    Install {
        #[arg(num_args(1..))]
        packages: Vec<String>,
    },
    /// Install <package> (and corresponding <package>-devel package(s))
    ///
    /// Will execute `xbps-install -S <package> <package>-devel`
    #[command(visible_alias = "di", alias = "devinstall")]
    DevInstall {
        #[arg(num_args(1..))]
        packages: Vec<String>,
    },
    /// List alternative candidates
    ///
    /// Will execute `xbps-alternatives -l`
    #[command(visible_alias = "la", alias = "listalternatives")]
    ListAlternatives {
        package: String,
        groups: Vec<String>,
    },
    /// Set alternative for <package>
    ///
    /// Will execute `xbps-alternatives -s`
    #[command(visible_alias = "sa", alias = "setalternative")]
    SetAlternative {
        package: String,
        groups: Vec<String>,
    },
    /// Re-configure installed <package>
    ///
    /// Will execute `xbps-reconfigure -v <package>`
    #[command(visible_alias = "rc")]
    Reconfigure { package: String },
    /// Force installation of <package(s)>
    ///
    /// Will execute `xbps-install -f <package>`
    #[command(visible_alias = "fi", alias = "forceinstall")]
    ForceInstall {
        #[arg(num_args(1..))]
        packages: Vec<String>,
    },
    /// Remove <package(s)> from the system
    ///
    /// Will execute `xbps-remove -v <package>`
    #[command(visible_aliases = ["rm", "uninstall"])]
    Remove {
        #[arg(num_args(1..))]
        packages: Vec<String>,
    },
    /// Recursively remove package(s) (and its dependencies)
    ///
    /// Will execute `xbps-remove -v -R <package>`
    #[command(alias = "removerecursive")]
    RemoveRecursive {
        #[arg(num_args(1..))]
        packages: Vec<String>,
    },
    /// Clean up cache directory
    ///
    /// Will execute `xbps-remove -v -O`
    #[command(visible_alias = "cl")]
    Cleanup,
    /// Remove orphaned packages
    ///
    /// Will execute `xbps-remove -v -o`
    #[command(visible_alias = "ar")]
    Autoremove,
    /// Search for package containing <file>
    ///
    /// Will execute `xlocate <file>`
    #[command(visible_alias = "wp", alias = "whatprovides")]
    WhatProvides { file: String },
    /// Passthrough to xbps-* at `/usr/sbin/xbps-*`
    #[command(external_subcommand)]
    External(#[arg(num_args(1..))] Vec<String>),
}

pub fn parse() -> Args {
    Args::parse()
}
