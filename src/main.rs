use anyhow::Result;
use clap::{Parser, Subcommand};
use site::Site;

mod config;
mod post;
mod server;
mod site;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Subcommand)]
enum Command {
    Build(config::BuildConfig),
    Serve(config::ServerConfig),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Build(config) => {
            let cache = jobber::Cache::new(config.build_cache_size);
            let s = Site::new(config, false);
            s.build_site_with_cache(&cache).unwrap();
        }
        Command::Serve(config) => {
            server::serve(config).await?;
        }
    }
    Ok(())
}
