//! vpm - void package management utility for
//! XBPS, the X Binary Package System
//! Copyright (c) 2016 Armin Jenewein <a@m2m.pm>, GitHub: @netzverweigerer
//! For more information about XBPS, see:
//! https://github.com/voidlinux/xbps

use anyhow::Result;
use clap::ColorChoice;
use colored::control::{
    set_override as set_color_override, unset_override as unset_color_override,
};
use std::process::ExitCode;

mod arguments;
mod commands;
mod config;

fn main() -> Result<ExitCode> {
    let args = arguments::parse();
    config::init(&args);

    match args.color {
        ColorChoice::Auto => unset_color_override(),
        ColorChoice::Always => set_color_override(true),
        ColorChoice::Never => set_color_override(false),
    }

    commands::run(args).map(ExitCode::from)
}
