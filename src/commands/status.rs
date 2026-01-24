use crate::cli::StatusArgs;
use crate::error::Result;
use crate::systemd;

pub fn run(args: StatusArgs, _dry_run: bool, _no_interactive: bool) -> Result<()> {
    let status = systemd::unit_status(&args.name, args.system)?;
    println!("{status}");
    Ok(())
}
