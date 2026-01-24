#![allow(dead_code)]

use crate::error::{MkunitError, Result};
use std::process::Command;

/// Minimum supported systemd version
pub const MIN_SYSTEMD_VERSION: u32 = 249;

/// Detected systemd version information
#[derive(Debug, Clone)]
pub struct SystemdVersion {
    pub major: u32,
    pub full_version: String,
}

impl SystemdVersion {
    /// Check if this version supports a specific feature
    #[must_use]
    pub fn supports_feature(&self, min_version: u32) -> bool {
        self.major >= min_version
    }

    /// Check if this version is at least the minimum supported
    #[must_use]
    pub fn is_supported(&self) -> bool {
        self.major >= MIN_SYSTEMD_VERSION
    }
}

/// Detect the installed systemd version
pub fn detect_version() -> Result<SystemdVersion> {
    let output = Command::new("systemctl")
        .arg("--version")
        .output()
        .map_err(|e| MkunitError::systemd_error(format!("Failed to run systemctl: {e}")))?;

    if !output.status.success() {
        return Err(MkunitError::systemd_error("systemctl --version failed"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_version(&stdout)
}

/// Parse systemd version from output
fn parse_version(output: &str) -> Result<SystemdVersion> {
    // Output format: "systemd 249 (249.11-0ubuntu3.9)"
    // or "systemd 252 (252-14.el9_2)"
    let first_line = output.lines().next().unwrap_or("");

    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(MkunitError::systemd_error(format!(
            "Could not parse systemd version from: {first_line}"
        )));
    }

    let version_str = parts[1];
    let major: u32 = version_str.parse().map_err(|_| {
        MkunitError::systemd_error(format!("Invalid systemd version number: {version_str}"))
    })?;

    Ok(SystemdVersion {
        major,
        full_version: first_line.to_string(),
    })
}

/// Check if systemd is available on this system
#[must_use]
pub fn is_systemd_available() -> bool {
    Command::new("systemctl")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version_ubuntu() {
        let output = "systemd 249 (249.11-0ubuntu3.9)\n+PAM +AUDIT +SELINUX";
        let version = parse_version(output).unwrap();
        assert_eq!(version.major, 249);
    }

    #[test]
    fn test_parse_version_rhel() {
        let output = "systemd 252 (252-14.el9_2)\n+PAM +AUDIT";
        let version = parse_version(output).unwrap();
        assert_eq!(version.major, 252);
    }

    #[test]
    fn test_version_supports_feature() {
        let version = SystemdVersion {
            major: 250,
            full_version: "systemd 250".to_string(),
        };
        assert!(version.supports_feature(249));
        assert!(version.supports_feature(250));
        assert!(!version.supports_feature(251));
    }

    #[test]
    fn test_version_is_supported() {
        let old_version = SystemdVersion {
            major: 240,
            full_version: "systemd 240".to_string(),
        };
        assert!(!old_version.is_supported());

        let new_version = SystemdVersion {
            major: 249,
            full_version: "systemd 249".to_string(),
        };
        assert!(new_version.is_supported());
    }
}
