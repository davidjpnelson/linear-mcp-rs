mod auth;
mod cache;
mod client;
mod error;
mod format;
mod graphql;
mod server;
mod tools;
mod types;

use rmcp::ServiceExt;
use tracing_subscriber::EnvFilter;

const THIRD_PARTY_LICENSES: &str = include_str!("../THIRD_PARTY_LICENSES.yaml");

const HELP_TEXT: &str = concat!(
    "Usage: ",
    env!("CARGO_PKG_NAME"),
    " [FLAGS]\n",
    "\n",
    env!("CARGO_PKG_NAME"),
    " ",
    env!("CARGO_PKG_VERSION"),
    " - Linear MCP server\n",
    "\n",
    "With no flags, starts the MCP server on stdio (the default behavior).\n",
    "\n",
    "Flags:\n",
    "  --licenses    Print third-party license attributions and exit\n",
    "  --version     Print version and exit\n",
    "  --help, -h    Print this help message and exit\n",
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Handle CLI flags before any async work, logging, or auth initialization.
    // Only the first positional flag is considered.
    let mut args = std::env::args().skip(1);
    if let Some(arg) = args.next() {
        match arg.as_str() {
            "--licenses" => {
                print!("{}", THIRD_PARTY_LICENSES);
                return Ok(());
            }
            "--version" => {
                println!(
                    "{} {}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                );
                return Ok(());
            }
            "--help" | "-h" => {
                print!("{}", HELP_TEXT);
                return Ok(());
            }
            other => {
                eprintln!("error: unknown flag '{}'", other);
                eprintln!("Try '--help' for usage.");
                std::process::exit(2);
            }
        }
    }

    // Logging to stderr (MCP uses stdout for protocol messages)
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let api_key = auth::load_api_key().map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    tracing::info!("Starting linear-mcp server");

    let client = client::LinearClient::new(api_key)?;
    let server = server::LinearMcp::new(client);

    let service = server.serve(rmcp::transport::io::stdio()).await?;
    service.waiting().await?;

    Ok(())
}
