pub mod paths;
pub mod version;

pub use paths::*;

use crate::error::{MkunitError, Result};
use std::process::Command;

/// Run systemctl command
pub fn systemctl(args: &[&str], system: bool) -> Result<String> {
    let mut cmd = Command::new("systemctl");

    if !system {
        cmd.arg("--user");
    }

    for arg in args {
        cmd.arg(arg);
    }

    log::debug!("Running: {cmd:?}");

    let output = cmd
        .output()
        .map_err(|e| MkunitError::systemd_error(format!("Failed to run systemctl: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(MkunitError::systemd_error(stderr.trim().to_string()));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Run daemon-reload
pub fn daemon_reload(system: bool) -> Result<()> {
    systemctl(&["daemon-reload"], system)?;
    Ok(())
}

/// Enable a unit
pub fn enable_unit(name: &str, system: bool) -> Result<()> {
    systemctl(&["enable", name], system)?;
    Ok(())
}

/// Disable a unit
pub fn disable_unit(name: &str, system: bool) -> Result<()> {
    systemctl(&["disable", name], system)?;
    Ok(())
}

/// Start a unit
pub fn start_unit(name: &str, system: bool) -> Result<()> {
    systemctl(&["start", name], system)?;
    Ok(())
}

/// Stop a unit
pub fn stop_unit(name: &str, system: bool) -> Result<()> {
    systemctl(&["stop", name], system)?;
    Ok(())
}

/// Restart a unit
pub fn restart_unit(name: &str, system: bool) -> Result<()> {
    systemctl(&["restart", name], system)?;
    Ok(())
}

/// Get unit status
pub fn unit_status(name: &str, system: bool) -> Result<String> {
    // Use --no-pager to avoid interactive output
    let mut cmd = Command::new("systemctl");

    if !system {
        cmd.arg("--user");
    }

    cmd.args(["status", "--no-pager", name]);

    log::debug!("Running: {cmd:?}");

    let output = cmd
        .output()
        .map_err(|e| MkunitError::systemd_error(format!("Failed to run systemctl: {e}")))?;

    // status returns non-zero for inactive units, which is fine
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Check if a unit is active
pub fn is_unit_active(name: &str, system: bool) -> bool {
    systemctl(&["is-active", "--quiet", name], system).is_ok()
}

/// Check if a unit is enabled
pub fn is_unit_enabled(name: &str, system: bool) -> bool {
    systemctl(&["is-enabled", "--quiet", name], system).is_ok()
}

/// Get unit logs via journalctl
pub fn unit_logs(
    name: &str,
    system: bool,
    lines: Option<u32>,
    follow: bool,
    since: Option<&str>,
) -> Result<()> {
    let mut cmd = Command::new("journalctl");

    if !system {
        cmd.arg("--user");
    }

    cmd.args(["--unit", name]);
    cmd.arg("--no-pager");

    if let Some(n) = lines {
        cmd.args(["-n", &n.to_string()]);
    }

    if follow {
        cmd.arg("-f");
    }

    if let Some(s) = since {
        cmd.args(["--since", s]);
    }

    log::debug!("Running: {cmd:?}");

    let status = cmd
        .status()
        .map_err(|e| MkunitError::systemd_error(format!("Failed to run journalctl: {e}")))?;

    if !status.success() && !follow {
        return Err(MkunitError::systemd_error(format!(
            "journalctl exited with status: {}",
            status.code().unwrap_or(-1)
        )));
    }

    Ok(())
}

/// List units matching a pattern
#[allow(dead_code)]
pub fn list_unit_files(pattern: Option<&str>, system: bool) -> Result<String> {
    let mut args = vec!["list-unit-files", "--no-pager"];
    if let Some(p) = pattern {
        args.push(p);
    }
    systemctl(&args, system)
}

/// Verify/validate a unit file
pub fn verify_unit(path: &str, system: bool) -> Result<String> {
    let mut cmd = Command::new("systemd-analyze");

    if !system {
        cmd.arg("--user");
    }

    cmd.args(["verify", path]);

    log::debug!("Running: {cmd:?}");

    let output = cmd
        .output()
        .map_err(|e| MkunitError::systemd_error(format!("Failed to run systemd-analyze: {e}")))?;

    // verify returns non-zero if there are warnings, but we still want the output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    Ok(format!("{stdout}{stderr}"))
}
