use std::num::NonZeroUsize;

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
            let s = Site::new(config, false);
            let cache = jobber::Cache::new(NonZeroUsize::new(1024).unwrap());
            s.build_site(&cache).unwrap();
        }
        Command::Serve(config) => {
            server::serve(config).await?;
        }
    }
    Ok(())
}
