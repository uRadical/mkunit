use crate::cli::LogsArgs;
use crate::error::Result;
use crate::systemd;

pub fn run(args: LogsArgs, _dry_run: bool, _no_interactive: bool) -> Result<()> {
    systemd::unit_logs(
        &args.name,
        args.system,
        Some(args.lines),
        args.follow,
        args.since.as_deref(),
    )?;
    Ok(())
}
