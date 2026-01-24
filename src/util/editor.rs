use crate::error::{MkunitError, Result};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;

/// Get the editor command from environment
fn get_editor() -> String {
    env::var("VISUAL")
        .or_else(|_| env::var("EDITOR"))
        .unwrap_or_else(|_| "vi".to_string())
}

/// Open a file in the user's editor and return whether it was modified
pub fn edit_file(path: &Path) -> Result<bool> {
    let editor = get_editor();

    // Get the file's modification time before editing
    let mtime_before = fs::metadata(path)
        .and_then(|m| m.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH);

    // Parse editor command (might have arguments like "code --wait")
    let mut parts = editor.split_whitespace();
    let program = parts.next().ok_or_else(|| {
        MkunitError::editor_error("No editor configured (set $VISUAL or $EDITOR)")
    })?;

    let mut cmd = Command::new(program);
    for arg in parts {
        cmd.arg(arg);
    }
    cmd.arg(path);

    log::debug!("Opening editor: {cmd:?}");

    let status = cmd.status().map_err(|e| {
        MkunitError::editor_error(format!("Failed to launch editor '{program}': {e}"))
    })?;

    if !status.success() {
        return Err(MkunitError::editor_error(format!(
            "Editor exited with status: {}",
            status.code().unwrap_or(-1)
        )));
    }

    // Check if the file was modified
    let mtime_after = fs::metadata(path)
        .and_then(|m| m.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH);

    Ok(mtime_after != mtime_before)
}

/// Create a temporary file with content and open in editor
/// Returns the new content if modified
#[allow(dead_code)]
pub fn edit_content(content: &str, suffix: &str) -> Result<Option<String>> {
    let temp_dir = env::temp_dir();
    let temp_path = temp_dir.join(format!("mkunit-edit-{}{suffix}", std::process::id()));

    fs::write(&temp_path, content)?;

    let was_modified = edit_file(&temp_path)?;

    if was_modified {
        let new_content = fs::read_to_string(&temp_path)?;
        fs::remove_file(&temp_path).ok();
        Ok(Some(new_content))
    } else {
        fs::remove_file(&temp_path).ok();
        Ok(None)
    }
}

/// Check if an editor is available
#[must_use]
#[allow(dead_code)]
pub fn editor_available() -> bool {
    let editor = get_editor();
    let program = editor.split_whitespace().next().unwrap_or("vi");
    which(program).is_some()
}

/// Simple which implementation
fn which(program: &str) -> Option<std::path::PathBuf> {
    if let Ok(path) = env::var("PATH") {
        for dir in path.split(':') {
            let full_path = Path::new(dir).join(program);
            if full_path.exists() && full_path.is_file() {
                return Some(full_path);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_editor_default() {
        // This test just verifies the function doesn't panic
        let _ = get_editor();
    }

    #[test]
    fn test_which_known_command() {
        // /bin/sh should exist on most Unix systems
        assert!(which("sh").is_some() || which("bash").is_some());
    }
}
