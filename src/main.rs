//! TheShit - A command-line utility to fix and enhance shell commands.
//!
//! See [README](https://github.com/AsfhtgkDavid/theshit) for more details.
mod cli;
mod errors;
mod fix;
mod misc;
mod shells;

use clap::Parser;
use cli::{Cli, Command};
use crossterm::style::Stylize;
use errors::TheShitError;
use std::env;
use std::io::ErrorKind;
use std::str::FromStr;

fn main() {
    #[cfg(not(feature = "standard_panic"))]
    misc::set_panic_hook();

    if let Err(e) = run() {
        eprintln!("{}: {}", "Error".red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), TheShitError> {
    let args = Cli::parse();

    let shell = args
        .shell
        .and_then(|shell| shells::Shell::from_str(&shell).ok())
        .or_else(shells::get_current_shell)
        .ok_or(TheShitError::ShellDetectionFailed)?;

    match args.command {
        Command::Alias { name } => {
            let program_path =
                env::current_exe().map_err(TheShitError::ExePathNotFound)?;
            let alias = shell.get_shell_function(&name, program_path.as_path());
            println!("{alias}");
        }
        Command::Fix => {
            let command = env::var("SH_PREV_CMD")
                .map_err(|_| TheShitError::PrevCmdNotSet)?;
            let expand_command = misc::expand_aliases(&command, shell.get_aliases());
            let fixed_command = fix::fix_command(command, expand_command)?;
            println!("{fixed_command}");
        }
        Command::Setup { name } => {
            let program_path =
                env::current_exe().map_err(TheShitError::ExePathNotFound)?;
            match shell.setup_alias(&name, program_path.as_path()) {
                Ok(_) => println!(
                    "{}",
                    format!("Alias setup successfully for {shell:?} as {name}").green()
                ),
                Err(e) if e.kind() == ErrorKind::AlreadyExists => {
                    println!("{}", "Alias already exists, skipping alias setup.".yellow());
                }
                Err(e) => return Err(TheShitError::Io(e)),
            }
            let config_dir = dirs::config_dir().ok_or(TheShitError::ConfigDirNotFound)?;
            match misc::create_default_fix_rules(config_dir.join("theshit/fix_rules")) {
                Ok(_) => println!("{}", "Default rules setup successfully".green()),
                Err(e) if e.kind() == ErrorKind::AlreadyExists => {
                    println!(
                        "{}",
                        "Default rules already exist, skipping rules setup.".yellow()
                    );
                }
                Err(e) => return Err(TheShitError::Io(e)),
            }
        }
    }
    Ok(())
}
