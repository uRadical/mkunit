use crate::cli::ListArgs;
use crate::error::Result;
use crate::systemd::{is_mkunit_created, list_units, unit_name_from_path};
use crate::util::color;
use colored::Colorize;

pub fn run(args: ListArgs, _dry_run: bool, _no_interactive: bool) -> Result<()> {
    if args.all {
        println!("{}", "User units:".bold());
        list_scope(false)?;
        println!();
        println!("{}", "System units:".bold());
        list_scope(true)?;
    } else {
        list_scope(args.system)?;
    }

    Ok(())
}

fn list_scope(system: bool) -> Result<()> {
    let units = list_units(system)?;

    if units.is_empty() {
        println!("  No units found");
        return Ok(());
    }

    for unit_path in units {
        let name = unit_name_from_path(&unit_path).unwrap_or_default();
        let is_mkunit = is_mkunit_created(&unit_path);

        let marker = if is_mkunit {
            color::success("●")
        } else {
            color::hint("○")
        };

        println!("  {marker} {}", color::unit_name(&name));
    }

    println!();
    println!(
        "  {} = created by mkunit, {} = other",
        color::success("●"),
        color::hint("○")
    );

    Ok(())
}
