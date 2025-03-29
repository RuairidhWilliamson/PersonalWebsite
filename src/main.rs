use anyhow::Result;
use clap::{Parser, Subcommand};
use site::Site;

mod config;
mod minify;
mod post;
mod progress;
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
    env_logger::init();
    let cli = Cli::parse();
    match cli.command {
        Command::Build(config) => {
            let cache = jobber::Cache::new(config.build_cache_size);
            Site::new(config, false).build_site_with_cache(&cache)?;
        }
        Command::Serve(config) => {
            #[cfg(feature = "server")]
            server::serve(config)?;
            #[cfg(not(feature = "server"))]
            {
                _ = config;
                panic!("server feature not available, add '-F server' when building")
            }
        }
    }
    Ok(())
}
