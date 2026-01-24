use crate::cli::SocketArgs;
use crate::commands::{handle_install_and_start, write_unit};
use crate::error::Result;
use crate::systemd::{unit_path, UnitType};
use crate::templates::{SocketData, Templates};
use crate::util::prompt::PromptBuilder;

pub fn run(args: SocketArgs, dry_run: bool, no_interactive: bool) -> Result<()> {
    let prompts = PromptBuilder::new(no_interactive);

    // Get description
    let description = args
        .description
        .unwrap_or_else(|| format!("{} socket", args.name));

    // Ensure at least one listen is specified
    let has_listen = args.listen_stream.is_some()
        || args.listen_datagram.is_some()
        || args.listen_fifo.is_some();

    let listen_stream = if !has_listen {
        Some(prompts.required("Listen address (e.g., '8080', '/run/myapp.sock')")?)
    } else {
        args.listen_stream
    };

    // Build template data
    let data = SocketData {
        description,
        listen_stream,
        listen_datagram: args.listen_datagram,
        listen_fifo: args.listen_fifo,
        accept: args.accept,
        max_connections: args.max_connections,
        unit: args.unit,
    };

    // Render template
    let templates = Templates::new()?;
    let content = templates.render_socket(&data)?;

    // Get output path
    let unit_file_path = unit_path(&args.name, UnitType::Socket, args.system)?;

    // Write unit file
    write_unit(&content, &unit_file_path, args.output.as_deref(), dry_run)?;

    // Handle installation
    if args.output.is_none() {
        let unit_name = format!("{}.socket", args.name);
        handle_install_and_start(&unit_name, args.install, false, args.system, dry_run)?;
    }

    Ok(())
}
