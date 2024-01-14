use std::{ffi::OsStr, net::SocketAddr, num::NonZeroUsize, path::PathBuf};

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub convert_images: Option<ImageConvert>,
    pub details: Details,
    pub pages: Vec<String>,
    pub posts: Vec<PostConfig>,
    pub featured: Vec<String>,
}

#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
pub struct PostConfig {
    pub slug: String,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Details {
    pub name: String,
    pub email: String,
    pub summary: String,
}

#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
pub enum ImageConvert {
    Webp,
}

impl ImageConvert {
    pub fn is_supported_convert_extension(&self, ext: Option<&OsStr>) -> bool {
        let Some(ext) = ext else {
            return false;
        };
        let exts = match self {
            ImageConvert::Webp => &["png", "jpeg"],
        };
        exts.iter()
            .any(|e| ext.to_string_lossy().eq_ignore_ascii_case(e))
    }

    pub fn extension(&self) -> &str {
        match self {
            ImageConvert::Webp => "webp",
        }
    }

    pub fn mime_type(&self) -> &str {
        match self {
            ImageConvert::Webp => "image/webp",
        }
    }
}
