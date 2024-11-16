use std::{ffi::OsStr, net::SocketAddr, num::NonZeroUsize, path::PathBuf, time::Duration};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Parser)]
pub struct BuildConfig {
    /// The source directory to read markdown from
    #[arg(short, long, default_value = "src", value_hint=clap::ValueHint::DirPath)]
    pub root_dir: PathBuf,

    /// The output directory to put generated files
    #[arg(short, long, default_value = "dist", value_hint=clap::ValueHint::DirPath)]
    pub output_dir: PathBuf,

    #[arg(long)]
    pub grammar_check: bool,

    #[arg(long)]
    pub minify: bool,

    #[arg(long, default_value = "1024")]
    pub build_cache_size: NonZeroUsize,

    /// Hide the screen clearing progress report
    #[arg(long)]
    pub no_progress: bool,
}

#[derive(Debug, Clone, Parser)]
pub struct ServerConfig {
    #[command(flatten)]
    pub build_config: BuildConfig,

    /// The address to bind the web server to
    #[arg(short, long, default_value = "0.0.0.0:3000")]
    pub addr: SocketAddr,

    #[arg(long)]
    pub hot_reload: bool,

    #[arg(long)]
    pub http_cache: bool,

    #[arg(long, value_parser = humantime::parse_duration, default_value = "200ms")]
    pub debounce_time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub convert_images: Option<ImageConvertFormat>,
    pub details: Details,
    pub pages: PagesConfig,
}

impl SiteConfig {
    pub fn validate(&self) -> anyhow::Result<()> {
        self.pages.validate()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagesConfig {
    pub featured: Vec<String>,
    pub pages: Vec<String>,
    pub posts: Vec<PostConfig>,
}

impl PagesConfig {
    pub fn validate(&self) -> anyhow::Result<()> {
        self.posts.iter().try_for_each(PostConfig::validate)
    }
}

#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
pub struct PostConfig {
    pub slug: String,
    pub image: Option<String>,
}

impl PostConfig {
    pub fn validate(&self) -> anyhow::Result<()> {
        if !self
            .slug
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return Err(anyhow::anyhow!(
                "{} contains invalid character, must be alphabetic or dash",
                self.slug
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Details {
    pub name: String,
    pub email: String,
    pub summary: String,
}

#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
pub enum ImageConvertFormat {
    Webp,
}

impl ImageConvertFormat {
    pub fn is_supported_convert_extension(&self, ext: Option<&OsStr>) -> bool {
        let Some(ext) = ext else {
            return false;
        };
        let exts = match self {
            Self::Webp => &["png", "jpeg"],
        };
        exts.iter()
            .any(|e| ext.to_string_lossy().eq_ignore_ascii_case(e))
    }

    pub fn extension(&self) -> &str {
        match self {
            Self::Webp => "webp",
        }
    }

    pub fn mime_type(&self) -> &str {
        match self {
            Self::Webp => "image/webp",
        }
    }
}
