use crate::cli::TimerArgs;
use crate::commands::{handle_install_and_start, write_unit};
use crate::error::{MkunitError, Result};
use crate::systemd::{unit_path, UnitType};
use crate::templates::{Templates, TimerData};
use crate::util::prompt::PromptBuilder;

pub fn run(args: TimerArgs, dry_run: bool, no_interactive: bool) -> Result<()> {
    let prompts = PromptBuilder::new(no_interactive);

    // Get unit to trigger
    let unit = args
        .unit
        .unwrap_or_else(|| format!("{}.service", args.name));

    // Get description
    let description = args
        .description
        .unwrap_or_else(|| format!("{} timer", args.name));

    // Ensure at least one trigger is specified
    let has_trigger = args.on_calendar.is_some()
        || args.on_boot.is_some()
        || args.on_startup.is_some()
        || args.on_active.is_some()
        || args.on_unit_active.is_some()
        || args.on_unit_inactive.is_some();

    let on_calendar = if !has_trigger {
        // Prompt for trigger
        let trigger = prompts
            .required("Timer trigger (e.g., 'daily', '*-*-* 04:00:00', or '5m' for on-boot)")?;

        // Detect if it's a duration or calendar expression
        if trigger.ends_with('s') || trigger.ends_with('m') || trigger.ends_with('h') {
            // Looks like a duration, use on-boot
            return Err(MkunitError::invalid_argument(
                "For boot-relative timers, use --on-boot flag explicitly",
            ));
        }
        Some(trigger)
    } else {
        args.on_calendar
    };

    // Build template data
    let data = TimerData {
        description,
        on_calendar,
        on_boot: args.on_boot,
        on_startup: args.on_startup,
        on_active: args.on_active,
        on_unit_active: args.on_unit_active,
        on_unit_inactive: args.on_unit_inactive,
        persistent: args.persistent,
        randomize_delay: args.randomize_delay,
        unit,
        wanted_by: args.wanted_by,
    };

    // Render template
    let templates = Templates::new()?;
    let content = templates.render_timer(&data)?;

    // Get output path
    let unit_file_path = unit_path(&args.name, UnitType::Timer, args.system)?;

    // Write unit file
    write_unit(&content, &unit_file_path, args.output.as_deref(), dry_run)?;

    // Handle installation
    if args.output.is_none() {
        let unit_name = format!("{}.timer", args.name);
        handle_install_and_start(&unit_name, args.install, false, args.system, dry_run)?;
    }

    Ok(())
}
