use crate::cli::EditArgs;
use crate::error::Result;
use crate::systemd::{self, find_unit};
use crate::util::{color, editor, prompt};

pub fn run(args: EditArgs, dry_run: bool, no_interactive: bool) -> Result<()> {
    // Find the unit file
    let unit_path = find_unit(&args.name, args.system)?;

    if dry_run {
        println!("Would edit: {}", unit_path.display());
        return Ok(());
    }

    color::print_info(&format!("Editing {}", unit_path.display()));

    // Edit the file
    let was_modified = editor::edit_file(&unit_path)?;

    if !was_modified {
        color::print_info("No changes made");
        return Ok(());
    }

    color::print_success("File saved");

    // Reload daemon
    if !args.no_reload {
        color::print_info("Reloading systemd daemon...");
        systemd::daemon_reload(args.system)?;
        color::print_success("Daemon reloaded");
    }

    // Prompt for restart
    if !args.no_restart {
        let unit_name = unit_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&args.name);

        if systemd::is_unit_active(unit_name, args.system) {
            let restart = prompt::prompt_confirm(
                &format!("Unit '{unit_name}' is active. Restart it?"),
                true,
                no_interactive,
            )?;

            if restart {
                systemd::restart_unit(unit_name, args.system)?;
                color::print_success(&format!("Restarted {unit_name}"));
            }
        }
    }

    Ok(())
}
