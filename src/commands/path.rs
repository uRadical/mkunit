use crate::cli::PathArgs;
use crate::commands::{handle_install_and_start, write_unit};
use crate::error::Result;
use crate::systemd::{unit_path, UnitType};
use crate::templates::{PathData, Templates};
use crate::util::prompt::PromptBuilder;

pub fn run(args: PathArgs, dry_run: bool, no_interactive: bool) -> Result<()> {
    let prompts = PromptBuilder::new(no_interactive);

    // Get unit to trigger
    let unit = args
        .unit
        .unwrap_or_else(|| format!("{}.service", args.name));

    // Get description
    let description = args
        .description
        .unwrap_or_else(|| format!("{} path watcher", args.name));

    // Ensure at least one path is specified
    let has_path = args.path_exists.is_some()
        || args.path_exists_glob.is_some()
        || args.path_changed.is_some()
        || args.path_modified.is_some()
        || args.directory_not_empty.is_some();

    let path_changed = if !has_path {
        Some(prompts.required("Path to watch")?)
    } else {
        args.path_changed
    };

    // Build template data
    let data = PathData {
        description,
        path_exists: args.path_exists,
        path_exists_glob: args.path_exists_glob,
        path_changed,
        path_modified: args.path_modified,
        directory_not_empty: args.directory_not_empty,
        make_directory: args.make_directory,
        unit,
        wanted_by: args.wanted_by,
    };

    // Render template
    let templates = Templates::new()?;
    let content = templates.render_path(&data)?;

    // Get output path
    let unit_file_path = unit_path(&args.name, UnitType::Path, args.system)?;

    // Write unit file
    write_unit(&content, &unit_file_path, args.output.as_deref(), dry_run)?;

    // Handle installation
    if args.output.is_none() {
        let unit_name = format!("{}.path", args.name);
        handle_install_and_start(&unit_name, args.install, false, args.system, dry_run)?;
    }

    Ok(())
}
