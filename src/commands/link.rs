use crate::cli::LinkArgs;
use crate::commands::handle_install_and_start;
use crate::error::{MkunitError, Result};
use crate::systemd::{self, unit_dir, UnitType};
use crate::util::color;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;

#[allow(clippy::too_many_lines)]
pub fn run(args: LinkArgs, dry_run: bool, _no_interactive: bool) -> Result<()> {
    let source_path = Path::new(&args.file);

    // Validate source file exists
    if !source_path.exists() {
        return Err(MkunitError::invalid_argument(format!(
            "File not found: {}",
            args.file
        )));
    }

    // Validate it's a file (not a directory)
    if !source_path.is_file() {
        return Err(MkunitError::invalid_argument(format!(
            "Not a file: {}",
            args.file
        )));
    }

    // Get the filename and validate extension
    let filename = source_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| MkunitError::invalid_argument("Invalid filename"))?;

    let extension = source_path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| {
            MkunitError::invalid_argument(
                "File must have a systemd unit extension (.service, .timer, .path, .socket, .mount, .target)",
            )
        })?;

    // Validate it's a known unit type
    if UnitType::from_extension(extension).is_none() {
        return Err(MkunitError::invalid_argument(format!(
            "Unknown unit type: .{extension}. Expected .service, .timer, .path, .socket, .mount, or .target"
        )));
    }

    // Get absolute path to source
    let abs_source = source_path.canonicalize().map_err(|e| {
        MkunitError::IoError(std::io::Error::new(
            e.kind(),
            format!("Failed to resolve path {}: {e}", args.file),
        ))
    })?;

    // Get target directory
    let target_dir = unit_dir(args.system)?;
    let target_path = target_dir.join(filename);

    // Check if target already exists
    if target_path.exists() || target_path.is_symlink() {
        if args.force {
            if dry_run {
                println!("Would remove existing: {}", target_path.display());
            } else {
                // Remove existing file or symlink
                if target_path.is_symlink() || target_path.is_file() {
                    fs::remove_file(&target_path)?;
                } else {
                    return Err(MkunitError::invalid_argument(format!(
                        "Target exists and is not a file or symlink: {}",
                        target_path.display()
                    )));
                }
            }
        } else {
            // Check if it's already a symlink to the same target
            if target_path.is_symlink() {
                if let Ok(existing_target) = fs::read_link(&target_path) {
                    if existing_target == abs_source {
                        color::print_info(&format!(
                            "Already linked: {} -> {}",
                            target_path.display(),
                            abs_source.display()
                        ));
                        // Still handle install/start if requested
                        return handle_install_and_start(
                            filename,
                            args.install,
                            args.start,
                            args.system,
                            dry_run,
                        );
                    }
                }
            }
            return Err(MkunitError::invalid_argument(format!(
                "Target already exists: {}. Use --force to overwrite",
                target_path.display()
            )));
        }
    }

    // Ensure target directory exists
    if dry_run {
        if !target_dir.exists() {
            println!("Would create directory: {}", target_dir.display());
        }
        println!(
            "Would create symlink: {} -> {}",
            target_path.display(),
            abs_source.display()
        );
    } else {
        fs::create_dir_all(&target_dir)?;
        symlink(&abs_source, &target_path)?;
        color::print_success(&format!(
            "Linked {} -> {}",
            target_path.display(),
            abs_source.display()
        ));
    }

    // Handle daemon-reload, enable, and start
    if args.install || dry_run {
        if dry_run && !args.install {
            println!(
                "Would run: systemctl{}daemon-reload",
                if args.system { " " } else { " --user " }
            );
        } else {
            handle_install_and_start(filename, args.install, args.start, args.system, dry_run)?;
        }
    } else {
        // Always daemon-reload after creating a symlink
        if dry_run {
            println!(
                "Would run: systemctl{}daemon-reload",
                if args.system { " " } else { " --user " }
            );
        } else {
            systemd::daemon_reload(args.system)?;
        }
    }

    Ok(())
}
