use anyhow::Result;
use clap::{Parser, Subcommand};
use site::Site;

mod config;
mod npm;
mod post;
mod site;

#[cfg(feature = "server")]
mod server;

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

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Build(config) => {
            let cache = jobber::Cache::new(config.build_cache_size);
            let s = Site::new(config, false);
            s.build_site_with_cache(&cache).unwrap();
        }
        Command::Serve(config) => {
            #[cfg(feature = "server")]
            server::serve(config)?;
            #[cfg(not(feature = "server"))]
            {
                _ = config;
                panic!("server feature not available, add -F server when building")
            }
        }
    }
    Ok(())
}
