use colored::{ColoredString, Colorize};
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};

static COLOR_DISABLED: AtomicBool = AtomicBool::new(false);

/// Initialize color support based on environment and flags
pub fn init(no_color_flag: bool) {
    let should_disable = no_color_flag
        || env::var("NO_COLOR").is_ok()
        || env::var("TERM").map(|t| t == "dumb").unwrap_or(false);

    COLOR_DISABLED.store(should_disable, Ordering::SeqCst);

    if should_disable {
        colored::control::set_override(false);
    }
}

/// Check if color output is enabled
#[must_use]
pub fn is_enabled() -> bool {
    !COLOR_DISABLED.load(Ordering::SeqCst)
}

/// Style for success messages
pub fn success(text: &str) -> ColoredString {
    text.green().bold()
}

/// Style for error messages
pub fn error(text: &str) -> ColoredString {
    text.red().bold()
}

/// Style for warning messages
pub fn warning(text: &str) -> ColoredString {
    text.yellow().bold()
}

/// Style for info messages
pub fn info(text: &str) -> ColoredString {
    text.cyan()
}

/// Style for hints
pub fn hint(text: &str) -> ColoredString {
    text.dimmed()
}

/// Style for paths/files
pub fn path(text: &str) -> ColoredString {
    text.blue()
}

/// Style for unit names
pub fn unit_name(text: &str) -> ColoredString {
    text.magenta().bold()
}

/// Style for section headers in unit files
pub fn section_header(text: &str) -> ColoredString {
    text.cyan().bold()
}

/// Style for keys in unit files
pub fn unit_key(text: &str) -> ColoredString {
    text.green()
}

/// Style for values in unit files
pub fn unit_value(text: &str) -> ColoredString {
    text.normal()
}

/// Style for comments
pub fn comment(text: &str) -> ColoredString {
    text.dimmed()
}

/// Format a key-value pair for display
#[allow(dead_code)]
pub fn key_value(key: &str, value: &str) -> String {
    format!("{}: {}", key.cyan(), value)
}

/// Print a success message
pub fn print_success(message: &str) {
    println!("{} {message}", success("✓"));
}

/// Print an error message
pub fn print_error(message: &str) {
    eprintln!("{} {message}", error("Error:"));
}

/// Print a warning message
pub fn print_warning(message: &str) {
    eprintln!("{} {message}", warning("Warning:"));
}

/// Print an info message
pub fn print_info(message: &str) {
    println!("{} {message}", info("→"));
}

/// Syntax highlight a unit file content
#[must_use]
pub fn highlight_unit_file(content: &str) -> String {
    if !is_enabled() {
        return content.to_string();
    }

    content
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('#') || trimmed.starts_with(';') {
                comment(line).to_string()
            } else if trimmed.starts_with('[') && trimmed.ends_with(']') {
                section_header(line).to_string()
            } else if let Some((key, value)) = line.split_once('=') {
                format!("{}={}", unit_key(key), unit_value(value))
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
