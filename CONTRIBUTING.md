# Contributing to mkunit

Thank you for considering contributing to mkunit!

## Development Setup

1. Install Rust (1.70 or later):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository:
   ```bash
   git clone https://github.com/alan/mkunit.git
   cd mkunit
   ```

3. Install development tools:
   ```bash
   cargo install just cargo-audit cargo-deny cargo-outdated cargo-machete
   ```

4. Run checks:
   ```bash
   just check
   ```

## Making Changes

1. Create a feature branch from `main`
2. Make your changes
3. Ensure all checks pass:
   ```bash
   just ci
   ```
4. Add tests for new functionality
5. Update documentation as needed
6. Submit a pull request

## Code Style

- Follow Rust standard style (enforced by `rustfmt`)
- All public items should have documentation comments
- Keep functions focused and reasonably sized
- Prefer clarity over cleverness

## Testing

- Add unit tests for new functions in the same file
- Add integration tests in `tests/integration.rs` for CLI behavior
- Test both success and error cases
- Use `--dry-run` and `--no-interactive` flags for deterministic tests

## Commit Messages

- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- First line should be 50 characters or less
- Reference issues and PRs where appropriate

## Reporting Issues

- Check existing issues before creating a new one
- Include your OS, Rust version, and systemd version
- Provide minimal reproduction steps
- Include relevant error messages

## Security

See [SECURITY.md](SECURITY.md) for reporting security vulnerabilities.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
