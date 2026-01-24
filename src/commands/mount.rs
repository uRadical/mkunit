use crate::cli::MountArgs;
use crate::commands::{handle_install_and_start, write_unit};
use crate::error::Result;
use crate::systemd::{unit_path, UnitType};
use crate::templates::{MountData, Templates};
use crate::util::prompt::PromptBuilder;

pub fn run(args: MountArgs, dry_run: bool, no_interactive: bool) -> Result<()> {
    let prompts = PromptBuilder::new(no_interactive);

    // Get what (source) - required
    let what = match args.what {
        Some(w) => w,
        None => prompts.required("Source device/path")?,
    };

    // Get where (mount point) - required
    let mount_where = match args.mount_where {
        Some(w) => w,
        None => prompts.required("Mount point")?,
    };

    // Get description
    let description = args
        .description
        .unwrap_or_else(|| format!("Mount {what} at {mount_where}"));

    // Build template data
    let data = MountData {
        description,
        what,
        r#where: mount_where,
        fs_type: args.fs_type,
        options: args.options,
        wanted_by: args.wanted_by,
    };

    // Render template
    let templates = Templates::new()?;
    let content = templates.render_mount(&data)?;

    // Get output path
    let unit_file_path = unit_path(&args.name, UnitType::Mount, args.system)?;

    // Write unit file
    write_unit(&content, &unit_file_path, args.output.as_deref(), dry_run)?;

    // Handle installation
    if args.output.is_none() {
        let unit_name = format!("{}.mount", args.name);
        handle_install_and_start(&unit_name, args.install, false, args.system, dry_run)?;
    }

    Ok(())
}
