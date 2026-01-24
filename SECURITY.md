# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in mkunit, please report it responsibly:

1. **Do not** open a public GitHub issue
2. Email the maintainer directly with:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

You can expect:
- Acknowledgment within 48 hours
- Status update within 7 days
- Credit in the security advisory (unless you prefer anonymity)

## Security Considerations

mkunit interacts with systemd and can:
- Create/modify unit files in user or system directories
- Execute `systemctl` and `journalctl` commands
- Read environment variables
- Launch external editors

### Best Practices

- Review generated unit files before installing with `--dry-run`
- Use `--hardening` flag for services that don't need full system access
- Avoid storing secrets in unit files; use `EnvironmentFile` instead
- System units (`--system`) require appropriate permissions

### Known Limitations

- Unit file validation is basic; use `systemd-analyze verify` for thorough checks
- The tool trusts user input for paths and commands
- Editor integration inherits the security context of the shell
