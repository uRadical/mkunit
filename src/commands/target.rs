use crate::cli::TargetArgs;
use crate::commands::{handle_install_and_start, write_unit};
use crate::error::Result;
use crate::systemd::{unit_path, UnitType};
use crate::templates::{TargetData, Templates};

pub fn run(args: TargetArgs, dry_run: bool, _no_interactive: bool) -> Result<()> {
    // Get description
    let description = args
        .description
        .unwrap_or_else(|| format!("{} target", args.name));

    // Join wants and requires lists
    let wants = if args.wants.is_empty() {
        None
    } else {
        Some(args.wants.join(" "))
    };

    let requires = if args.requires.is_empty() {
        None
    } else {
        Some(args.requires.join(" "))
    };

    // Build template data
    let data = TargetData {
        description,
        wants,
        requires,
        after: args.after,
        wanted_by: args.wanted_by,
    };

    // Render template
    let templates = Templates::new()?;
    let content = templates.render_target(&data)?;

    // Get output path
    let unit_file_path = unit_path(&args.name, UnitType::Target, args.system)?;

    // Write unit file
    write_unit(&content, &unit_file_path, args.output.as_deref(), dry_run)?;

    // Handle installation
    if args.output.is_none() {
        let unit_name = format!("{}.target", args.name);
        handle_install_and_start(&unit_name, args.install, false, args.system, dry_run)?;
    }

    Ok(())
}
