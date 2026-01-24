// Command modules commonly pass owned structs to simplify API
// and use patterns that are clearer than pedantically preferred alternatives
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::if_not_else)]
#![allow(clippy::single_match_else)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::fn_params_excessive_bools)]
#![allow(clippy::case_sensitive_file_extension_comparisons)]

pub mod completions;
pub mod edit;
pub mod link;
pub mod list;
pub mod logs;
pub mod mount;
pub mod path;
pub mod remove;
pub mod service;
pub mod show;
pub mod socket;
pub mod status;
pub mod target;
pub mod timer;
pub mod validate;

use crate::error::{Result, ValidationWarning};
use crate::systemd;
use crate::util::color;
use std::fs;
use std::path::Path;

/// Common output handling for unit creation commands
pub fn write_unit(
    content: &str,
    unit_path: &Path,
    output_path: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    let target_path =
        output_path.map_or_else(|| unit_path.to_path_buf(), |p| Path::new(p).to_path_buf());

    if dry_run {
        println!("Would write to: {}", target_path.display());
        println!();
        println!("{}", color::highlight_unit_file(content));
        return Ok(());
    }

    // Ensure parent directory exists
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&target_path, content)?;
    color::print_success(&format!("Created {}", target_path.display()));

    Ok(())
}

/// Handle installation and starting of a unit
pub fn handle_install_and_start(
    unit_name: &str,
    install: bool,
    start: bool,
    system: bool,
    dry_run: bool,
) -> Result<()> {
    if install {
        if dry_run {
            println!(
                "Would run: systemctl{}daemon-reload",
                if system { " " } else { " --user " }
            );
            println!(
                "Would run: systemctl{}enable {}",
                if system { " " } else { " --user " },
                unit_name
            );
        } else {
            systemd::daemon_reload(system)?;
            systemd::enable_unit(unit_name, system)?;
            color::print_success(&format!("Enabled {unit_name}"));
        }

        if start {
            if dry_run {
                println!(
                    "Would run: systemctl{}start {}",
                    if system { " " } else { " --user " },
                    unit_name
                );
            } else {
                systemd::start_unit(unit_name, system)?;
                color::print_success(&format!("Started {unit_name}"));
            }
        }
    }

    Ok(())
}

/// Validate common issues and print warnings
pub fn validate_and_warn(exec_path: Option<&str>, workdir: Option<&str>) -> Vec<ValidationWarning> {
    let mut warnings = Vec::new();

    // Check for relative paths in exec
    if let Some(exec) = exec_path {
        let first_part = exec.split_whitespace().next().unwrap_or(exec);
        if !first_part.starts_with('/') && !first_part.starts_with('$') {
            warnings.push(ValidationWarning::with_suggestion(
                format!("ExecStart path '{first_part}' is not absolute"),
                "Use absolute paths for reliability",
            ));
        }

        // Check if executable exists
        if first_part.starts_with('/') && !Path::new(first_part).exists() {
            warnings.push(ValidationWarning::new(format!(
                "Executable '{first_part}' not found"
            )));
        }

        // Check for shebang if it's a script
        if first_part.ends_with(".sh") || first_part.ends_with(".py") {
            if let Ok(content) = fs::read_to_string(first_part) {
                if !content.starts_with("#!") {
                    warnings.push(ValidationWarning::with_suggestion(
                        format!("Script '{first_part}' may be missing shebang"),
                        "Add #!/bin/bash or #!/usr/bin/env python at the start",
                    ));
                }
            }
        }
    }

    // Check if working directory exists
    if let Some(dir) = workdir {
        if dir.starts_with('/') && !Path::new(dir).exists() {
            warnings.push(ValidationWarning::new(format!(
                "Working directory '{dir}' does not exist"
            )));
        }
    }

    warnings
}

/// Print validation warnings
pub fn print_warnings(warnings: &[ValidationWarning]) {
    for warning in warnings {
        color::print_warning(&warning.to_string());
    }
}
