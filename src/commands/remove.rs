use crate::cli::RemoveArgs;
use crate::error::Result;
use crate::systemd::{self, find_unit, is_mkunit_created};
use crate::util::{color, prompt};
use std::fs;

pub fn run(args: RemoveArgs, dry_run: bool, no_interactive: bool) -> Result<()> {
    // Find the unit file
    let unit_path = find_unit(&args.name, args.system)?;

    let unit_name = unit_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&args.name);

    // Check if created by mkunit
    let is_mkunit = is_mkunit_created(&unit_path);

    // Confirm removal
    if !args.force {
        let warning = if is_mkunit {
            format!("Remove unit '{unit_name}'?")
        } else {
            format!("Unit '{unit_name}' was not created by mkunit. Remove anyway?")
        };

        let confirmed = prompt::prompt_confirm_or_abort(&warning, false, no_interactive)?;
        if !confirmed {
            color::print_info("Cancelled");
            return Ok(());
        }
    }

    if dry_run {
        println!("Would stop: {unit_name}");
        println!("Would disable: {unit_name}");
        println!("Would remove: {}", unit_path.display());
        return Ok(());
    }

    // Stop if active
    if systemd::is_unit_active(unit_name, args.system) {
        color::print_info(&format!("Stopping {unit_name}..."));
        systemd::stop_unit(unit_name, args.system)?;
    }

    // Disable if enabled
    if systemd::is_unit_enabled(unit_name, args.system) {
        color::print_info(&format!("Disabling {unit_name}..."));
        systemd::disable_unit(unit_name, args.system)?;
    }

    // Remove the file
    color::print_info(&format!("Removing {}...", unit_path.display()));
    fs::remove_file(&unit_path)?;

    // Reload daemon
    systemd::daemon_reload(args.system)?;

    color::print_success(&format!("Removed {unit_name}"));

    Ok(())
}
