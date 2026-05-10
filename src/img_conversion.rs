use std::{ffi::OsStr, path::Path};

use anyhow::{Context as _, Result};
use image::GenericImageView as _;
use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum ImageConvertFormat {
    Jxl,
    Webp,
    Avif,
    Png,
    Jpeg,
    Gif,
}

impl ImageConvertFormat {
    pub fn from_ext(ext: &OsStr) -> Option<Self> {
        match ext.to_str()?.to_ascii_lowercase().as_str() {
            "jxl" => Some(Self::Jxl),
            "webp" => Some(Self::Webp),
            "avif" => Some(Self::Avif),
            "png" => Some(Self::Png),
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "gif" => Some(Self::Gif),
            _ => None,
        }
    }

    pub fn can_convert(self, dest: Self) -> bool {
        match (self, dest) {
            (Self::Png | Self::Jpeg, Self::Jxl | Self::Webp | Self::Avif) => true,
            // (Self::Gif, Self::Jxl) => true,
            _ => false,
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Jxl => "jxl",
            Self::Webp => "webp",
            Self::Avif => "avif",
            Self::Png => "png",
            Self::Jpeg => "jpg",
            Self::Gif => "gif",
        }
    }

    pub fn mime_type(self) -> &'static str {
        match self {
            Self::Jxl => "image/jxl",
            Self::Webp => "image/webp",
            Self::Avif => "image/avif",
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::Gif => "image/gif",
        }
    }

    pub fn convert(
        self,
        src: &Path,
        target_cover_size: (u32, u32),
        quality: Quality,
        dst: &Path,
    ) -> Result<(u32, u32)> {
        let src_format = Self::from_ext(src.extension().context("missing source extension")?)
            .context("invalid source extension")?;
        debug_assert!(src_format.can_convert(self));
        let img = image::ImageReader::open(src)?.decode()?;
        let (width, height) = Self::calculate_img_size(img.dimensions(), target_cover_size);
        if let Some(dir) = dst.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut out = std::io::BufWriter::new(std::fs::File::create(dst)?);
        let resized = img.resize_to_fill(width, height, image::imageops::FilterType::Lanczos3);
        match (&src_format, self) {
            (Self::Gif, Self::Jxl) => {
                todo!("gif to jxl not implemented yet")
            }
            (_, Self::Jxl) => {
                let (pixels, layout) = if resized.has_alpha() {
                    (resized.to_rgba8().to_vec(), jxl_encoder::PixelLayout::Rgba8)
                } else {
                    (resized.to_rgb8().to_vec(), jxl_encoder::PixelLayout::Rgb8)
                };
                let config = quality.jxl_encoder();
                let mut encoder = config.encoder(resized.width(), resized.height(), layout)?;
                encoder.push_rows(&pixels, resized.height())?;
                encoder.finish_to(&mut out)?;
            }
            (_, Self::Webp) => {
                resized.write_to(&mut out, image::ImageFormat::WebP)?;
            }
            (_, Self::Avif) => {
                resized.write_to(&mut out, image::ImageFormat::Avif)?;
            }
            (_, Self::Png | Self::Jpeg | Self::Gif) => todo!(),
        }
        Ok((width, height))
    }

    fn calculate_img_size(actual_size: (u32, u32), target_cover_size: (u32, u32)) -> (u32, u32) {
        let (w, h) = actual_size;
        let new_w = w.min(target_cover_size.0);
        let new_h = new_w * h / w;
        let new_new_h = h.min(new_h);
        let new_new_w = new_new_h * new_w / new_h;
        (new_new_w, new_new_h)
    }
}

#[derive(Clone, Copy)]
pub enum Quality {
    Thumbnail,
    Hero,
}

impl Quality {
    fn jxl_encoder(self) -> jxl_encoder::LossyConfig {
        match self {
            Self::Thumbnail => jxl_encoder::LossyConfig::new(2.4)
                .with_mode(jxl_encoder::EncoderMode::Experimental)
                .with_effort(10)
                .with_progressive(jxl_encoder::ProgressiveMode::Single),
            Self::Hero => jxl_encoder::LossyConfig::new(1.0)
                .with_mode(jxl_encoder::EncoderMode::Experimental)
                .with_effort(10)
                .with_progressive(jxl_encoder::ProgressiveMode::DcVlfLfAc),
        }
    }
}
