use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Only generate man pages in release builds or when explicitly requested
    if env::var("PROFILE").unwrap_or_default() != "release" && env::var("GENERATE_MAN").is_err() {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap_or_else(|_| "target".to_string()));
    let man_dir = out_dir.join("man");

    fs::create_dir_all(&man_dir).expect("Failed to create man directory");

    // Build a minimal CLI for man page generation
    let cmd = clap::Command::new("mkunit")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Alan")
        .about("Generate and manage systemd unit files")
        .subcommand(clap::Command::new("service").about("Create a service unit"))
        .subcommand(clap::Command::new("timer").about("Create a timer unit"))
        .subcommand(clap::Command::new("path").about("Create a path unit"))
        .subcommand(clap::Command::new("socket").about("Create a socket unit"))
        .subcommand(clap::Command::new("mount").about("Create a mount unit"))
        .subcommand(clap::Command::new("target").about("Create a target unit"))
        .subcommand(clap::Command::new("edit").about("Edit an existing unit"))
        .subcommand(clap::Command::new("show").about("Show a unit file"))
        .subcommand(clap::Command::new("validate").about("Validate a unit file"))
        .subcommand(clap::Command::new("status").about("Show unit status"))
        .subcommand(clap::Command::new("logs").about("Show unit logs"))
        .subcommand(clap::Command::new("remove").about("Remove a unit"))
        .subcommand(clap::Command::new("list").about("List units"))
        .subcommand(clap::Command::new("completions").about("Generate shell completions"));

    let man = clap_mangen::Man::new(cmd);

    let mut buffer = Vec::new();
    man.render(&mut buffer).expect("Failed to render man page");

    let man_path = man_dir.join("mkunit.1");
    fs::write(&man_path, buffer).expect("Failed to write man page");

    println!("cargo:rerun-if-changed=build.rs");
}
