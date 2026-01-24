use crate::cli::ShowArgs;
use crate::error::Result;
use crate::systemd::find_unit;
use crate::util::color;
use std::fs;

pub fn run(args: ShowArgs, _dry_run: bool, _no_interactive: bool) -> Result<()> {
    // Find the unit file
    let unit_path = find_unit(&args.name, args.system)?;

    // Read and display content
    let content = fs::read_to_string(&unit_path)?;

    println!("{}", color::path(&unit_path.display().to_string()));
    println!();
    println!("{}", color::highlight_unit_file(&content));

    Ok(())
}
