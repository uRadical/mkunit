use crate::cli::{build_cli, CompletionsArgs};
use crate::error::Result;
use clap_complete::generate;
use std::io;

#[allow(clippy::needless_pass_by_value)]
pub fn run(args: CompletionsArgs) -> Result<()> {
    let mut cmd = build_cli();
    generate(args.shell, &mut cmd, "mkunit", &mut io::stdout());
    Ok(())
}
