//! # mkunit
//!
//! A CLI tool for generating and managing systemd unit files.
//!
//! mkunit simplifies the creation and management of systemd units by providing
//! an intuitive command-line interface with both interactive and scriptable modes.
//!
//! ## Supported Unit Types
//!
//! - **service**: Long-running daemons and one-shot tasks
//! - **timer**: Scheduled execution (cron replacement)
//! - **path**: File/directory watching triggers
//! - **socket**: Socket activation for on-demand services
//! - **mount**: Filesystem mount points
//! - **target**: Grouping and synchronization points
//!
//! ## Example
//!
//! ```bash
//! # Create a hardened service
//! mkunit service myapp --exec "/usr/bin/myapp" --hardening --install
//!
//! # Create a daily timer
//! mkunit timer backup --on-calendar daily --persistent
//! ```

mod cli;
mod commands;
mod error;
mod systemd;
mod templates;
mod util;

use clap::Parser;
use cli::{Cli, Commands};
use error::ExitCode;
use std::process;

fn main() {
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    }

    // Initialize color support
    util::color::init(cli.no_color);

    // Run the command
    let result = run_command(cli);

    match result {
        Ok(()) => process::exit(ExitCode::Success as i32),
        Err(e) => {
            util::color::print_error(&e.to_string());

            // Print hint if available
            if let error::MkunitError::UnitNotFound {
                hint: Some(ref h), ..
            } = e
            {
                eprintln!();
                eprintln!("  Hint: {h}");
            }

            process::exit(e.exit_code() as i32);
        }
    }
}

fn run_command(cli: Cli) -> error::Result<()> {
    let dry_run = cli.dry_run;
    let no_interactive = cli.no_interactive;

    match cli.command {
        Commands::Service(args) => commands::service::run(args, dry_run, no_interactive),
        Commands::Timer(args) => commands::timer::run(args, dry_run, no_interactive),
        Commands::Path(args) => commands::path::run(args, dry_run, no_interactive),
        Commands::Socket(args) => commands::socket::run(args, dry_run, no_interactive),
        Commands::Mount(args) => commands::mount::run(args, dry_run, no_interactive),
        Commands::Target(args) => commands::target::run(args, dry_run, no_interactive),
        Commands::Edit(args) => commands::edit::run(args, dry_run, no_interactive),
        Commands::Show(args) => commands::show::run(args, dry_run, no_interactive),
        Commands::Validate(args) => commands::validate::run(args, dry_run, no_interactive),
        Commands::Status(args) => commands::status::run(args, dry_run, no_interactive),
        Commands::Logs(args) => commands::logs::run(args, dry_run, no_interactive),
        Commands::Remove(args) => commands::remove::run(args, dry_run, no_interactive),
        Commands::List(args) => commands::list::run(args, dry_run, no_interactive),
        Commands::Link(args) => commands::link::run(args, dry_run, no_interactive),
        Commands::Completions(args) => commands::completions::run(args),
    }
}
