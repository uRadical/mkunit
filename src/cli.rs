// CLI structs commonly have many bool flags - this is expected
#![allow(clippy::struct_excessive_bools)]

use clap::{Args, Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

/// A CLI tool for generating systemd unit files
#[derive(Parser, Debug)]
#[command(name = "mkunit")]
#[command(author = "Alan")]
#[command(version)]
#[command(about = "Generate and manage systemd unit files", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Show what would happen without making changes
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Fail instead of prompting for missing values
    #[arg(long, global = true)]
    pub no_interactive: bool,

    /// Disable colored output
    #[arg(long, global = true, env = "NO_COLOR")]
    pub no_color: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a service unit
    Service(ServiceArgs),

    /// Create a timer unit
    Timer(TimerArgs),

    /// Create a path unit
    Path(PathArgs),

    /// Create a socket unit
    Socket(SocketArgs),

    /// Create a mount unit
    Mount(MountArgs),

    /// Create a target unit
    Target(TargetArgs),

    /// Edit an existing unit
    Edit(EditArgs),

    /// Show a unit file
    Show(ShowArgs),

    /// Validate a unit file
    Validate(ValidateArgs),

    /// Show unit status
    Status(StatusArgs),

    /// Show unit logs
    Logs(LogsArgs),

    /// Remove a unit
    Remove(RemoveArgs),

    /// List units
    List(ListArgs),

    /// Generate shell completions
    Completions(CompletionsArgs),
}

/// Service type options
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum ServiceType {
    #[default]
    Simple,
    Exec,
    Forking,
    Oneshot,
    Notify,
}

impl std::fmt::Display for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple => write!(f, "simple"),
            Self::Exec => write!(f, "exec"),
            Self::Forking => write!(f, "forking"),
            Self::Oneshot => write!(f, "oneshot"),
            Self::Notify => write!(f, "notify"),
        }
    }
}

/// Restart policy options
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum RestartPolicy {
    No,
    #[default]
    OnFailure,
    Always,
    OnSuccess,
}

impl std::fmt::Display for RestartPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::No => write!(f, "no"),
            Self::OnFailure => write!(f, "on-failure"),
            Self::Always => write!(f, "always"),
            Self::OnSuccess => write!(f, "on-success"),
        }
    }
}

#[derive(Args, Debug)]
pub struct ServiceArgs {
    /// Service name
    pub name: String,

    /// Command to run
    #[arg(short = 'e', long)]
    pub exec: Option<String>,

    /// Unit description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Working directory
    #[arg(short, long)]
    pub workdir: Option<String>,

    /// Run as user
    #[arg(short, long)]
    pub user: Option<String>,

    /// Run as group
    #[arg(short, long)]
    pub group: Option<String>,

    /// Restart policy
    #[arg(short, long, value_enum, default_value_t = RestartPolicy::OnFailure)]
    pub restart: RestartPolicy,

    /// Seconds between restarts
    #[arg(long, default_value_t = 5)]
    pub restart_sec: u32,

    /// Service type
    #[arg(short = 't', long = "type", value_enum, default_value_t = ServiceType::Simple)]
    pub service_type: ServiceType,

    /// Environment variable (can be repeated)
    #[arg(long = "env", value_name = "KEY=VALUE")]
    pub env: Vec<String>,

    /// Path to environment file
    #[arg(long)]
    pub env_file: Option<String>,

    /// Start after unit(s)
    #[arg(long, default_value = "network.target")]
    pub after: String,

    /// Weak dependency
    #[arg(long)]
    pub wants: Option<String>,

    /// Strong dependency
    #[arg(long)]
    pub requires: Option<String>,

    /// Install target
    #[arg(long, default_value = "default.target")]
    pub wanted_by: String,

    /// Create as system service
    #[arg(long)]
    pub system: bool,

    /// Install and enable immediately
    #[arg(short, long)]
    pub install: bool,

    /// Start after installing
    #[arg(long)]
    pub start: bool,

    /// Write to path instead of installing
    #[arg(short, long)]
    pub output: Option<String>,

    /// Apply security hardening defaults
    #[arg(long)]
    pub hardening: bool,
}

#[derive(Args, Debug)]
pub struct TimerArgs {
    /// Timer name
    pub name: String,

    /// Service unit to trigger
    #[arg(short, long)]
    pub unit: Option<String>,

    /// Unit description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Calendar expression (e.g., "daily", "*-*-* 04:00:00")
    #[arg(long)]
    pub on_calendar: Option<String>,

    /// Seconds after boot
    #[arg(long)]
    pub on_boot: Option<String>,

    /// Seconds after systemd start
    #[arg(long)]
    pub on_startup: Option<String>,

    /// Seconds after timer activation
    #[arg(long)]
    pub on_active: Option<String>,

    /// Seconds after unit last activated
    #[arg(long)]
    pub on_unit_active: Option<String>,

    /// Seconds after unit last deactivated
    #[arg(long)]
    pub on_unit_inactive: Option<String>,

    /// Catch up missed runs
    #[arg(long)]
    pub persistent: bool,

    /// Random delay up to value
    #[arg(long)]
    pub randomize_delay: Option<String>,

    /// Install target
    #[arg(long, default_value = "timers.target")]
    pub wanted_by: String,

    /// Create as system timer
    #[arg(long)]
    pub system: bool,

    /// Install and enable immediately
    #[arg(short, long)]
    pub install: bool,

    /// Write to path instead of installing
    #[arg(short, long)]
    pub output: Option<String>,
}

#[derive(Args, Debug)]
pub struct PathArgs {
    /// Path unit name
    pub name: String,

    /// Service to trigger
    #[arg(short, long)]
    pub unit: Option<String>,

    /// Unit description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Trigger when path exists
    #[arg(long)]
    pub path_exists: Option<String>,

    /// Trigger when glob matches
    #[arg(long)]
    pub path_exists_glob: Option<String>,

    /// Trigger when path changes
    #[arg(long)]
    pub path_changed: Option<String>,

    /// Trigger when path modified
    #[arg(long)]
    pub path_modified: Option<String>,

    /// Trigger when directory has contents
    #[arg(long)]
    pub directory_not_empty: Option<String>,

    /// Create watched directory
    #[arg(long)]
    pub make_directory: bool,

    /// Install target
    #[arg(long, default_value = "default.target")]
    pub wanted_by: String,

    /// Create as system unit
    #[arg(long)]
    pub system: bool,

    /// Install and enable
    #[arg(short, long)]
    pub install: bool,

    /// Write to path instead of installing
    #[arg(short, long)]
    pub output: Option<String>,
}

#[derive(Args, Debug)]
pub struct SocketArgs {
    /// Socket name
    pub name: String,

    /// Service to activate
    #[arg(short, long)]
    pub unit: Option<String>,

    /// Unit description
    #[arg(short, long)]
    pub description: Option<String>,

    /// TCP or Unix stream socket
    #[arg(long)]
    pub listen_stream: Option<String>,

    /// UDP or Unix datagram socket
    #[arg(long)]
    pub listen_datagram: Option<String>,

    /// Named pipe
    #[arg(long)]
    pub listen_fifo: Option<String>,

    /// Spawn instance per connection
    #[arg(long)]
    pub accept: bool,

    /// Connection limit
    #[arg(long)]
    pub max_connections: Option<u32>,

    /// Create as system unit
    #[arg(long)]
    pub system: bool,

    /// Install and enable
    #[arg(short, long)]
    pub install: bool,

    /// Write to path instead of installing
    #[arg(short, long)]
    pub output: Option<String>,
}

#[derive(Args, Debug)]
pub struct MountArgs {
    /// Mount unit name
    pub name: String,

    /// Source device/path
    #[arg(long)]
    pub what: Option<String>,

    /// Mount point
    #[arg(long = "where")]
    pub mount_where: Option<String>,

    /// Filesystem type
    #[arg(short = 't', long = "type")]
    pub fs_type: Option<String>,

    /// Mount options
    #[arg(long)]
    pub options: Option<String>,

    /// Unit description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Install target
    #[arg(long, default_value = "multi-user.target")]
    pub wanted_by: String,

    /// Create as system unit (default for mount)
    #[arg(long, default_value_t = true)]
    pub system: bool,

    /// Install and enable
    #[arg(short, long)]
    pub install: bool,

    /// Write to path instead of installing
    #[arg(short, long)]
    pub output: Option<String>,
}

#[derive(Args, Debug)]
pub struct TargetArgs {
    /// Target name
    pub name: String,

    /// Unit description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Units this target wants (can be repeated)
    #[arg(long)]
    pub wants: Vec<String>,

    /// Units this target requires (can be repeated)
    #[arg(long)]
    pub requires: Vec<String>,

    /// Order after these units
    #[arg(long)]
    pub after: Option<String>,

    /// Install target
    #[arg(long, default_value = "default.target")]
    pub wanted_by: String,

    /// Create as system unit
    #[arg(long)]
    pub system: bool,

    /// Install and enable
    #[arg(short, long)]
    pub install: bool,

    /// Write to path instead of installing
    #[arg(short, long)]
    pub output: Option<String>,
}

#[derive(Args, Debug)]
pub struct EditArgs {
    /// Unit name
    pub name: String,

    /// Edit system unit
    #[arg(long)]
    pub system: bool,

    /// Skip daemon-reload after editing
    #[arg(long)]
    pub no_reload: bool,

    /// Skip restart prompt
    #[arg(long)]
    pub no_restart: bool,
}

#[derive(Args, Debug)]
pub struct ShowArgs {
    /// Unit name
    pub name: String,

    /// Show system unit
    #[arg(long)]
    pub system: bool,
}

#[derive(Args, Debug)]
pub struct ValidateArgs {
    /// Unit file path
    pub file: String,
}

#[derive(Args, Debug)]
pub struct StatusArgs {
    /// Unit name
    pub name: String,

    /// Query system unit
    #[arg(long)]
    pub system: bool,
}

#[derive(Args, Debug)]
pub struct LogsArgs {
    /// Unit name
    pub name: String,

    /// Query system unit
    #[arg(long)]
    pub system: bool,

    /// Follow log output
    #[arg(short, long)]
    pub follow: bool,

    /// Number of lines
    #[arg(short = 'n', long, default_value_t = 50)]
    pub lines: u32,

    /// Show logs since
    #[arg(long)]
    pub since: Option<String>,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Unit name
    pub name: String,

    /// Remove system unit
    #[arg(long)]
    pub system: bool,

    /// Skip confirmation
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    /// List system units
    #[arg(long)]
    pub system: bool,

    /// List both user and system
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Args, Debug)]
pub struct CompletionsArgs {
    /// Shell to generate completions for
    pub shell: Shell,
}

/// Build the CLI for use in build.rs
pub fn build_cli() -> clap::Command {
    Cli::augment_args(clap::Command::new("mkunit"))
}
