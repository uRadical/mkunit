use crate::cli::ValidateArgs;
use crate::error::{MkunitError, Result};
use crate::systemd;
use crate::util::color;
use std::fs;
use std::path::Path;

#[allow(unused_assignments)]
pub fn run(args: ValidateArgs, _dry_run: bool, _no_interactive: bool) -> Result<()> {
    let path = Path::new(&args.file);

    if !path.exists() {
        return Err(MkunitError::invalid_argument(format!(
            "File not found: {}",
            args.file
        )));
    }

    color::print_info(&format!("Validating {}", args.file));

    // Read and do basic parsing
    let content = fs::read_to_string(path)?;
    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    // Basic syntax checks
    let mut in_section = false;
    let mut current_section = String::new();

    for (line_num, line) in content.lines().enumerate() {
        let line_num = line_num + 1;
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }

        // Section header
        if trimmed.starts_with('[') {
            if !trimmed.ends_with(']') {
                errors.push(format!(
                    "Line {line_num}: Malformed section header: {trimmed}"
                ));
            } else {
                current_section = trimmed[1..trimmed.len() - 1].to_string();
                in_section = true;

                // Check for valid section names
                let valid_sections = [
                    "Unit", "Service", "Timer", "Path", "Socket", "Mount", "Target", "Install",
                ];
                if !valid_sections.contains(&current_section.as_str()) {
                    warnings.push(format!(
                        "Line {line_num}: Unknown section [{current_section}]"
                    ));
                }
            }
            continue;
        }

        // Key=Value pairs
        if !in_section {
            warnings.push(format!(
                "Line {line_num}: Content outside of section: {trimmed}"
            ));
            continue;
        }

        if !trimmed.contains('=') {
            errors.push(format!(
                "Line {line_num}: Invalid syntax (missing '='): {trimmed}"
            ));
            continue;
        }

        let (key, value) = trimmed.split_once('=').unwrap();
        let key = key.trim();
        let value = value.trim();

        // Check for common issues
        if key == "ExecStart" || key == "ExecStartPre" || key == "ExecStartPost" {
            let cmd = value.trim_start_matches(['-', '+', '!', ':', '@'].as_ref());
            let first_word = cmd.split_whitespace().next().unwrap_or("");
            if !first_word.starts_with('/') && !first_word.starts_with('$') {
                warnings.push(format!(
                    "Line {line_num}: Exec path is not absolute: {first_word}"
                ));
            }
        }

        if key == "WorkingDirectory" && !value.starts_with('/') && !value.starts_with('~') {
            warnings.push(format!(
                "Line {line_num}: WorkingDirectory is not absolute: {value}"
            ));
        }
    }

    // Run systemd-analyze verify if available
    let verify_output = systemd::verify_unit(&args.file, false);
    if let Ok(output) = verify_output {
        if !output.is_empty() {
            println!("\nsystemd-analyze verify output:");
            println!("{output}");
        }
    }

    // Print results
    if errors.is_empty() && warnings.is_empty() {
        color::print_success("Unit file is valid");
    } else {
        for error in &errors {
            color::print_error(error);
        }
        for warning in &warnings {
            color::print_warning(warning);
        }

        if !errors.is_empty() {
            return Err(MkunitError::validation_error(format!(
                "{} error(s), {} warning(s)",
                errors.len(),
                warnings.len()
            )));
        }

        println!();
        color::print_warning(&format!("{} warning(s)", warnings.len()));
    }

    Ok(())
}
