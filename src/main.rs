fn main() {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("--version") | Some("-V") => println!("sourceright {}", env!("CARGO_PKG_VERSION")),
        Some("--help") | Some("-h") | None => print_help(),
        Some("mcp") => {
            eprintln!("MCP server mode is planned but not implemented yet.");
            std::process::exit(2);
        }
        Some(command) => {
            eprintln!("unknown command: {command}");
            eprintln!("run `sourceright --help` for available commands");
            std::process::exit(2);
        }
    }
}

fn print_help() {
    println!(
        "sourceright\n\nUsage:\n  sourceright --help\n  sourceright --version\n  sourceright mcp\n\nSourceright is being bootstrapped as reference verification infrastructure."
    );
}
