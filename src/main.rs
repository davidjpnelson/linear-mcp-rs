mod auth;
mod cache;
mod client;
mod error;
mod format;
mod graphql;
mod server;
mod tools;
#[allow(dead_code)]
mod types;

use rmcp::ServiceExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let client = client::LinearClient::new(api_key);
    let server = server::LinearMcp::new(client);

    let service = server.serve(rmcp::transport::io::stdio()).await?;
    service.waiting().await?;

    Ok(())
}
