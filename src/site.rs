use std::path::Path;

use anyhow::{Context, Result};
use jobber::{Cache, JobCtx, JobIdBuilder};
use serde::Serialize;
use walkdir::WalkDir;

use crate::{
    config::{BuildConfig, ImageConvert, PostConfig, SiteConfig},
    post::PostDetails,
};

#[derive(Debug, Clone, Serialize)]
struct Info {
    details: crate::config::Details,
    posts: Vec<PostDetails>,
    featured: Vec<PostDetails>,
}

#[derive(Debug)]
pub struct Site {
    config: BuildConfig,
    include_hot_reload: bool,
}

impl Site {
    pub fn new(config: BuildConfig, include_hot_reload: bool) -> Self {
        Self {
            config,
            include_hot_reload,
        }
    }

    pub fn build_site(&self, cache: &Cache) -> Result<((), u64)> {
        println!("Building site...");
        let start = std::time::Instant::now();
        let ((), hash) = cache.root_job(
            JobIdBuilder::new("build_site").build(),
            |ctx: &mut JobCtx<'_>| {
                // self.copy_all_assets(ctx)?;
                self.copyfile(
                    ctx,
                    Path::new("assets/favicon.ico"),
                    Path::new("assets/favicon.ico"),
                )?;
                self.copyfile(ctx, Path::new("assets/robots.txt"), Path::new("robots.txt"))?;
                self.render_template_css(ctx, "style.css", Path::new("style.css"))?;
                self.render_template_js(ctx, "theme.js", Path::new("theme.js"))?;
                self.render_template_js(ctx, "navbar.js", Path::new("navbar.js"))?;
                self.render_all_posts(ctx)?;
                self.render_template_html(ctx, "index.html", Path::new("index.html"))?;
                self.render_template_html(ctx, "404.html", Path::new("404.html"))?;
                self.render_all_pages(ctx)?;
                Ok(())
            },
        )?;
        let end = std::time::Instant::now();
        let elapsed = (end - start).as_secs_f32();
        println!(" 🚀 Built {hash:x} ⏲️  {elapsed:.2} s");
        Ok(((), hash))
    }

    #[jobber::job]
    fn site_config_loader(&self, ctx: &mut JobCtx<'_>) -> Result<SiteConfig> {
        let config_path = self.config.root_dir.join("config.toml");
        ctx.depends_file(&config_path)?;
        let config_contents = std::fs::read_to_string(config_path)?;
        let cfg: SiteConfig = toml::from_str(&config_contents)?;
        Ok(cfg)
    }

    #[jobber::job]
    fn post_loader(&self, ctx: &mut JobCtx<'_>, post_config: &PostConfig) -> Result<PostDetails> {
        let site_config = self.site_config_loader(ctx)?;
        let src = self.config.root_dir.join("posts");
        site_config
            .posts
            .iter()
            .find(|&p| p.slug == post_config.slug)
            .context("could not find post")?;
        let path = src.join(format!("{}.md", post_config.slug));
        ctx.depends_file(&path)?;
        let contents = std::fs::read_to_string(&path).context(format!("read {path:?}"))?;
        let post = PostDetails::extract(post_config, &contents)
            .context(format!("extract post {:?}", post_config.slug))?;
        Ok(post)
    }

    #[jobber::job]
    fn post_loader_by_slug(&self, ctx: &mut JobCtx<'_>, slug: &str) -> Result<PostDetails> {
        let site_config = self.site_config_loader(ctx)?;
        let post_config = site_config
            .posts
            .iter()
            .find(|p| p.slug == slug)
            .context("could not find post")?;
        self.post_loader(ctx, post_config)
    }

    #[jobber::job]
    fn featured_posts(&self, ctx: &mut JobCtx<'_>) -> Result<Vec<PostDetails>> {
        let site_config = self.site_config_loader(ctx)?;
        site_config
            .featured
            .iter()
            .map(|slug| self.post_loader_by_slug(ctx, slug))
            .collect()
    }

    #[jobber::job]
    fn all_posts(&self, ctx: &mut JobCtx<'_>) -> Result<Vec<PostDetails>> {
        let site_config = self.site_config_loader(ctx)?;
        site_config
            .posts
            .iter()
            .map(|post_config| self.post_loader(ctx, post_config))
            .collect()
    }

    #[jobber::job]
    fn all_info(&self, ctx: &mut JobCtx<'_>) -> Result<Info> {
        let site_config = self.site_config_loader(ctx)?;
        let posts = self.all_posts(ctx)?;
        let featured = self.featured_posts(ctx)?;
        Ok(Info {
            details: site_config.details,
            posts,
            featured,
        })
    }

    #[jobber::job]
    fn copy_all_assets(&self, ctx: &mut JobCtx<'_>) -> Result<()> {
        let path = self.config.root_dir.join("assets");
        for res in WalkDir::new(path).min_depth(1) {
            let e = res?;
            if !e.file_type().is_file() {
                continue;
            }
            let p = e.path().strip_prefix(&self.config.root_dir)?;
            self.copyfile(ctx, p, p)?;
        }
        Ok(())
    }

    #[jobber::job]
    fn copyfile(&self, ctx: &mut JobCtx<'_>, src: &Path, dst: &Path) -> Result<()> {
        println!("copyfile {src:?} -> {dst:?}");
        let source = self.config.root_dir.join(src);
        let destination = self.config.output_dir.join(dst);
        ctx.depends_file(&source)?;
        if let Some(dir) = destination.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::copy(source, destination)?;
        Ok(())
    }

    #[jobber::job]
    fn convert_image(
        &self,
        ctx: &mut JobCtx<'_>,
        ty: &ImageConvert,
        src: &Path,
        dst: &Path,
    ) -> Result<()> {
        let source = self.config.root_dir.join(src);
        let destination = self.config.output_dir.join(dst);
        ctx.depends_file(&source)?;
        println!("convert to {ty:?} {src:?} {dst:?}");
        if let Some(dir) = destination.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut out = std::fs::File::create(destination)?;
        let img_fmt = match ty {
            ImageConvert::Webp => image::ImageFormat::WebP,
        };
        let img = image::io::Reader::open(source)?.decode()?;
        img.write_to(&mut out, img_fmt)?;
        Ok(())
    }

    #[jobber::job]
    fn template_loader(&self, ctx: &mut JobCtx<'_>) -> Result<tera::Tera> {
        let path = self
            .config
            .root_dir
            .join("templates")
            .join("**")
            .join("*")
            .display()
            .to_string();
        ctx.depends(jobber::Leaf::Glob(path.to_owned()))?;
        Ok(tera::Tera::new(&path)?)
    }

    fn render_template_html_common(
        &self,
        ctx: &mut JobCtx<'_>,
        templates: tera::Tera,
        render_ctx: tera::Context,
        src: &str,
        dst: &Path,
    ) -> Result<()> {
        let site_config = self.site_config_loader(ctx)?;
        let mut rendered = templates.render(src, &render_ctx)?;
        let img_regex = regex::Regex::new("<img[^>]* src=\"([^\"]+)\"[^>]*>")?;
        rendered = img_regex
            .replace_all(&rendered, |cap: &regex::Captures<'_>| {
                let img = cap.get(0).unwrap().as_str();
                let path = cap.get(1).unwrap().as_str().replace("&#x2F;", "/");
                path.strip_prefix('/')
                    .map(Path::new)
                    .map(|src| {
                        self.replace_img(ctx, img, src, &site_config.convert_images)
                            .unwrap()
                    })
                    .flatten()
                    .unwrap_or_else(|| img.to_owned())
            })
            .to_string();
        let rendered_bytes = if self.config.minify {
            minify_html::minify(rendered.as_bytes(), &minify_html::Cfg::spec_compliant())
        } else {
            rendered.as_bytes().to_owned()
        };
        let destination = self.config.output_dir.join(dst);
        if let Some(dir) = destination.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(destination, rendered_bytes)?;
        Ok(())
    }

    fn replace_img(
        &self,
        ctx: &mut JobCtx<'_>,
        img: &str,
        src: &Path,
        convert: &Option<ImageConvert>,
    ) -> Result<Option<String>> {
        self.copyfile(ctx, src, src)?;
        let Some(img_fmt) = convert else {
            return Ok(None);
        };
        if !img_fmt.is_supported_convert_extension(src.extension()) {
            return Ok(None);
        }
        let mut new_src = src.to_path_buf();
        new_src.set_extension(img_fmt.extension());
        self.convert_image(ctx, &img_fmt, src, &new_src)?;
        let mime_type = img_fmt.mime_type();
        let new_path_str = new_src.display();
        Ok(Some(format!(
            "<picture><source srcset=\"/{new_path_str}\" type=\"{mime_type}\"/>{img}</picture>"
        )))
    }

    #[jobber::job]
    fn render_template_html(&self, ctx: &mut JobCtx<'_>, src: &str, dst: &Path) -> Result<()> {
        println!("Render {src}");
        let templates = self.template_loader(ctx)?;
        let info = self.all_info(ctx)?;
        let mut render_ctx = tera::Context::from_serialize(info)?;
        render_ctx.insert("hot_reload", &self.include_hot_reload);
        self.render_template_html_common(ctx, templates, render_ctx, src, dst)
    }

    #[jobber::job]
    fn render_post(&self, ctx: &mut JobCtx<'_>, post_config: &PostConfig) -> Result<()> {
        println!("Render post {}", post_config.slug);
        let post = self.post_loader(ctx, post_config)?;
        let templates = self.template_loader(ctx)?;
        let mut render_ctx = tera::Context::from_serialize(post)?;
        render_ctx.insert("hot_reload", &self.include_hot_reload);
        let dst = Path::new("posts")
            .join(&post_config.slug)
            .join("index.html");
        self.render_template_html_common(ctx, templates, render_ctx, "post.html", &dst)
    }

    #[jobber::job]
    fn render_all_posts(&self, ctx: &mut JobCtx<'_>) -> Result<()> {
        let site_config = self.site_config_loader(ctx)?;
        for post_config in &site_config.posts {
            self.render_post(ctx, post_config)?;
        }
        Ok(())
    }

    #[jobber::job]
    fn render_all_pages(&self, ctx: &mut JobCtx<'_>) -> Result<()> {
        let site_config = self.site_config_loader(ctx)?;
        for page in &site_config.pages {
            self.render_template_html(
                ctx,
                &format!("{page}.html"),
                &Path::new(page).join("index.html"),
            )?;
        }
        Ok(())
    }

    #[jobber::job]
    fn render_template_js(&self, ctx: &mut JobCtx<'_>, src: &str, dst: &Path) -> Result<()> {
        println!("Render {src}");
        let templates = self.template_loader(ctx)?;
        let info = self.all_info(ctx)?;
        let mut render_ctx = tera::Context::from_serialize(info)?;
        render_ctx.insert("hot_reload", &self.include_hot_reload);
        let rendered = templates.render(src, &render_ctx)?;
        let rendered_bytes = if self.config.minify {
            let mut buf = Vec::new();
            minify_js::minify(
                &minify_js::Session::new(),
                minify_js::TopLevelMode::Global,
                rendered.as_bytes(),
                &mut buf,
            )
            .unwrap();
            buf
        } else {
            rendered.as_bytes().to_owned()
        };
        let destination = self.config.output_dir.join(dst);
        if let Some(dir) = destination.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(destination, rendered_bytes)?;
        Ok(())
    }

    #[jobber::job]
    fn render_template_css(&self, ctx: &mut JobCtx<'_>, src: &str, dst: &Path) -> Result<()> {
        println!("Render {src}");
        let templates = self.template_loader(ctx)?;
        let info = self.all_info(ctx)?;
        let mut render_ctx = tera::Context::from_serialize(info)?;
        render_ctx.insert("hot_reload", &self.include_hot_reload);
        let rendered = templates.render(src, &render_ctx)?;
        let rendered_bytes = if self.config.minify {
            css_minify::optimizations::Minifier::default()
                .minify(&rendered, css_minify::optimizations::Level::Three)
                .unwrap()
                .into_bytes()
        } else {
            rendered.as_bytes().to_owned()
        };
        let destination = self.config.output_dir.join(dst);
        if let Some(dir) = destination.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(destination, rendered_bytes)?;
        Ok(())
    }
}
