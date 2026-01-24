use crate::cli::ServiceArgs;
use crate::commands::{handle_install_and_start, print_warnings, validate_and_warn, write_unit};
use crate::error::Result;
use crate::systemd::{unit_path, UnitType};
use crate::templates::{ServiceData, Templates};
use crate::util::prompt::PromptBuilder;
use std::env;

pub fn run(args: ServiceArgs, dry_run: bool, no_interactive: bool) -> Result<()> {
    let prompts = PromptBuilder::new(no_interactive);

    // Get exec command (required)
    let exec = match args.exec {
        Some(e) => e,
        None => prompts.required("Command to run")?,
    };

    // Get description
    let description = args
        .description
        .unwrap_or_else(|| format!("{} service", args.name));

    // Get working directory
    let workdir = match args.workdir {
        Some(w) => Some(w),
        None => {
            let current = env::current_dir()
                .ok()
                .and_then(|p| p.to_str().map(String::from));
            if let Some(ref cwd) = current {
                let use_cwd = prompts.confirm(
                    &format!("Use current directory as working directory ({cwd})?"),
                    false,
                )?;
                if use_cwd {
                    current
                } else {
                    None
                }
            } else {
                None
            }
        }
    };

    // Get user
    let user = match args.user {
        Some(u) => Some(u),
        None => {
            if !args.system {
                None // User units run as the user by default
            } else {
                prompts.optional_empty("Run as user (leave empty for root)")?
            }
        }
    };

    // Build template data
    let data = ServiceData {
        description,
        after: if args.after.is_empty() {
            None
        } else {
            Some(args.after)
        },
        wants: args.wants,
        requires: args.requires,
        service_type: args.service_type.to_string(),
        exec: exec.clone(),
        workdir: workdir.clone(),
        user,
        group: args.group,
        restart: args.restart.to_string(),
        restart_sec: args.restart_sec,
        env: args.env,
        env_file: args.env_file,
        hardening: args.hardening,
        wanted_by: args.wanted_by,
    };

    // Validate and warn
    let warnings = validate_and_warn(Some(&exec), workdir.as_deref());
    print_warnings(&warnings);

    // Render template
    let templates = Templates::new()?;
    let content = templates.render_service(&data)?;

    // Get output path
    let unit_file_path = unit_path(&args.name, UnitType::Service, args.system)?;

    // Write unit file
    write_unit(&content, &unit_file_path, args.output.as_deref(), dry_run)?;

    // Handle installation
    if args.output.is_none() {
        let unit_name = format!("{}.service", args.name);
        handle_install_and_start(&unit_name, args.install, args.start, args.system, dry_run)?;
    }

    Ok(())
}
