use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn mkunit() -> Command {
    Command::cargo_bin("mkunit").unwrap()
}

#[test]
fn test_help() {
    mkunit()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Generate and manage systemd unit files",
        ));
}

#[test]
fn test_version() {
    mkunit()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("mkunit"));
}

#[test]
fn test_service_help() {
    mkunit()
        .args(["service", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--exec"));
}

#[test]
fn test_service_dry_run() {
    mkunit()
        .args([
            "service",
            "test-service",
            "--exec",
            "/usr/bin/test",
            "--dry-run",
            "--no-interactive",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ExecStart=/usr/bin/test"))
        .stdout(predicate::str::contains("Description=test-service service"));
}

#[test]
fn test_service_dry_run_with_options() {
    mkunit()
        .args([
            "service",
            "myapp",
            "--exec",
            "/usr/bin/myapp",
            "--description",
            "My Application",
            "--user",
            "myuser",
            "--restart",
            "always",
            "--type",
            "notify",
            "--env",
            "FOO=bar",
            "--env",
            "BAZ=qux",
            "--dry-run",
            "--no-interactive",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Description=My Application"))
        .stdout(predicate::str::contains("User=myuser"))
        .stdout(predicate::str::contains("Restart=always"))
        .stdout(predicate::str::contains("Type=notify"))
        .stdout(predicate::str::contains("Environment=\"FOO=bar\""))
        .stdout(predicate::str::contains("Environment=\"BAZ=qux\""));
}

#[test]
fn test_service_hardening() {
    mkunit()
        .args([
            "service",
            "secure-app",
            "--exec",
            "/usr/bin/secure",
            "--hardening",
            "--dry-run",
            "--no-interactive",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("NoNewPrivileges=true"))
        .stdout(predicate::str::contains("ProtectSystem=strict"))
        .stdout(predicate::str::contains("PrivateTmp=true"));
}

#[test]
fn test_timer_dry_run() {
    mkunit()
        .args([
            "timer",
            "test-timer",
            "--on-calendar",
            "daily",
            "--dry-run",
            "--no-interactive",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("OnCalendar=daily"))
        .stdout(predicate::str::contains("Unit=test-timer.service"));
}

#[test]
fn test_timer_with_persistent() {
    mkunit()
        .args([
            "timer",
            "backup",
            "--on-calendar",
            "*-*-* 04:00:00",
            "--persistent",
            "--randomize-delay",
            "1h",
            "--dry-run",
            "--no-interactive",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Persistent=true"))
        .stdout(predicate::str::contains("RandomizedDelaySec=1h"));
}

#[test]
fn test_path_dry_run() {
    mkunit()
        .args([
            "path",
            "test-path",
            "--path-changed",
            "/tmp/watch",
            "--dry-run",
            "--no-interactive",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("PathChanged=/tmp/watch"))
        .stdout(predicate::str::contains("Unit=test-path.service"));
}

#[test]
fn test_socket_dry_run() {
    mkunit()
        .args([
            "socket",
            "test-socket",
            "--listen-stream",
            "8080",
            "--dry-run",
            "--no-interactive",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ListenStream=8080"));
}

#[test]
fn test_mount_dry_run() {
    mkunit()
        .args([
            "mount",
            "test-mount",
            "--what",
            "/dev/sda1",
            "--where",
            "/mnt/data",
            "--type",
            "ext4",
            "--dry-run",
            "--no-interactive",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("What=/dev/sda1"))
        .stdout(predicate::str::contains("Where=/mnt/data"))
        .stdout(predicate::str::contains("Type=ext4"));
}

#[test]
fn test_target_dry_run() {
    mkunit()
        .args([
            "target",
            "test-target",
            "--description",
            "Test Target",
            "--wants",
            "foo.service",
            "--wants",
            "bar.service",
            "--dry-run",
            "--no-interactive",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Description=Test Target"))
        .stdout(predicate::str::contains("Wants=foo.service bar.service"));
}

#[test]
fn test_service_output_to_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.service");

    mkunit()
        .args([
            "service",
            "test",
            "--exec",
            "/usr/bin/test",
            "--output",
            output_path.to_str().unwrap(),
            "--no-interactive",
        ])
        .assert()
        .success();

    let content = std::fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("# Generated by mkunit"));
    assert!(content.contains("ExecStart=/usr/bin/test"));
}

#[test]
fn test_validate_valid_file() {
    let temp_dir = TempDir::new().unwrap();
    let unit_path = temp_dir.path().join("valid.service");

    std::fs::write(
        &unit_path,
        r#"[Unit]
Description=Valid Service

[Service]
Type=simple
ExecStart=/usr/bin/true

[Install]
WantedBy=default.target
"#,
    )
    .unwrap();

    mkunit()
        .args(["validate", unit_path.to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn test_validate_invalid_file() {
    let temp_dir = TempDir::new().unwrap();
    let unit_path = temp_dir.path().join("invalid.service");

    std::fs::write(
        &unit_path,
        r#"[Unit
Description=Invalid - missing bracket

no_section_content=bad
"#,
    )
    .unwrap();

    mkunit()
        .args(["validate", unit_path.to_str().unwrap()])
        .assert()
        .failure();
}

#[test]
fn test_completions_bash() {
    mkunit()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("_mkunit"));
}

#[test]
fn test_completions_zsh() {
    mkunit()
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef mkunit"));
}

#[test]
fn test_completions_fish() {
    mkunit()
        .args(["completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete"));
}

#[test]
fn test_no_color_flag() {
    mkunit()
        .args([
            "service",
            "test",
            "--exec",
            "/usr/bin/test",
            "--dry-run",
            "--no-interactive",
            "--no-color",
        ])
        .assert()
        .success();
}

#[test]
fn test_verbose_flag() {
    mkunit()
        .args([
            "service",
            "test",
            "--exec",
            "/usr/bin/test",
            "--dry-run",
            "--no-interactive",
            "-v",
        ])
        .assert()
        .success();
}

#[test]
fn test_service_missing_exec_no_interactive() {
    mkunit()
        .args(["service", "test", "--no-interactive"])
        .assert()
        .failure();
}

#[test]
fn test_validate_nonexistent_file() {
    mkunit()
        .args(["validate", "/nonexistent/path/unit.service"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));
}
