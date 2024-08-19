use std::path::Path;

use anyhow::{Context, Result};
use jobber::{Cache, JobCtx, JobIdBuilder, Progress, ProgressReport, RootJobOutput};
use serde::Serialize;
use walkdir::WalkDir;

use crate::{
    config::{BuildConfig, ImageConvertFormat, PostConfig, SiteConfig},
    post::PostDetails,
};

#[derive(Debug, Clone, Serialize)]
struct Info {
    details: crate::config::Details,
    posts: Vec<PostDetails>,
    featured: Vec<PostDetails>,
}

pub struct SiteBuildProgress;

impl SiteBuildProgress {
    fn report_built(
        RootJobOutput {
            generation,
            hash,
            stats,
            completed_stats,
            ..
        }: &RootJobOutput<()>,
    ) {
        let elapsed = completed_stats.total_time;
        print!("{}c", 27 as char);
        println!();
        println!(" 🚀 Built {hash:x} ");
        println!(" Generation = {generation}");
        println!(
            " Jobs = {} / {} = {:.1}%",
            stats.jobs_run(),
            stats.total_jobs(),
            stats.jobs_cache_percent()
        );
        println!(" ⏱️  {elapsed:.1?}");
    }
}

impl Progress for SiteBuildProgress {
    fn report(
        &self,
        ProgressReport {
            generation, stats, ..
        }: ProgressReport,
    ) {
        print!("{}c", 27 as char);
        println!();
        println!(" 🔨 Building...");
        println!(" Generation = {generation}");
        println!(
            " Jobs = {} / {} = {:.1}%",
            stats.jobs_run(),
            stats.total_jobs(),
            stats.jobs_cache_percent()
        );
    }
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

    pub fn build_site_with_cache(&self, cache: &Cache) -> Result<u64> {
        let progress = SiteBuildProgress;
        let output = cache.root_job_with_progress(
            JobIdBuilder::new("build_site").build(),
            &progress,
            |ctx| self.build_site(ctx),
        )?;
        SiteBuildProgress::report_built(&output);

        Ok(output.hash)
    }

    fn build_site(&self, ctx: &mut JobCtx<'_>) -> Result<()> {
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
        self.render_template_js(ctx, "hl_all.js", Path::new("hl_all.js"))?;
        self.render_all_posts(ctx)?;
        self.render_template_html(ctx, "index.html", Path::new("index.html"))?;
        self.render_template_html(ctx, "404.html", Path::new("404.html"))?;
        self.render_all_pages(ctx)?;
        Ok(())
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
            .pages
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
            .pages
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
            .pages
            .featured
            .iter()
            .map(|slug| self.post_loader_by_slug(ctx, slug))
            .collect()
    }

    #[jobber::job]
    fn all_posts(&self, ctx: &mut JobCtx<'_>) -> Result<Vec<PostDetails>> {
        let site_config = self.site_config_loader(ctx)?;
        site_config
            .pages
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
        #[cfg(feature = "job_print")]
        eprintln!("Copyfile {src:?} -> {dst:?}");
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
        ty: &ImageConvertFormat,
        target_cover_size: (u32, u32),
        src: &Path,
        dst: &Path,
    ) -> Result<(u32, u32)> {
        let source = self.config.root_dir.join(src);
        let destination = self.config.output_dir.join(dst);
        ctx.depends_file(&source)?;
        #[cfg(feature = "job_print")]
        eprintln!("Convert image {ty:?} {src:?} -> {dst:?}");
        if let Some(dir) = destination.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut out = std::fs::File::create(destination)?;
        let img_fmt = match ty {
            ImageConvertFormat::Webp => image::ImageFormat::WebP,
        };
        let img = image::ImageReader::open(source)?.decode()?;
        let w = img.width();
        let h = img.height();
        let new_w = w.min(target_cover_size.0);
        let new_h = new_w * h / w;
        let new_new_h = h.min(new_h);
        let new_new_w = new_new_h * new_w / new_h;
        img.resize_to_fill(new_new_w, new_new_h, image::imageops::FilterType::Lanczos3)
            .write_to(&mut out, img_fmt)?;
        Ok((new_new_w, new_new_h))
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
        ctx.depends(jobber::Leaf::Glob(path.clone()))?;
        Ok(tera::Tera::new(&path)?)
    }

    #[jobber::job]
    fn img_tag_regex(&self, ctx: &mut JobCtx<'_>) -> Result<regex::Regex> {
        Ok(regex::Regex::new("<img[^>]* src=\"([^\"]+)\"[^>]*>")?)
    }

    fn render_template_html_common(
        &self,
        ctx: &mut JobCtx<'_>,
        templates: &tera::Tera,
        render_ctx: &tera::Context,
        src: &str,
        dst: &Path,
    ) -> Result<()> {
        let site_config = self.site_config_loader(ctx)?;
        let mut rendered = templates.render(src, render_ctx)?;
        let img_regex = self.img_tag_regex(ctx)?;
        rendered = img_regex
            .replace_all(&rendered, |cap: &regex::Captures<'_>| {
                let img = cap.get(0).unwrap().as_str();
                let path = cap.get(1).unwrap().as_str().replace("&#x2F;", "/");
                path.strip_prefix('/')
                    .map(Path::new)
                    .and_then(|src| {
                        self.replace_img(ctx, img, src, &site_config.convert_images)
                            .unwrap()
                    })
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

    #[jobber::job]
    fn img_class_regex(&self, ctx: &mut JobCtx<'_>) -> Result<regex::Regex> {
        Ok(regex::Regex::new("class=\"([^\"]*)\"")?)
    }

    fn replace_img(
        &self,
        ctx: &mut JobCtx<'_>,
        img: &str,
        src: &Path,
        convert: &Option<ImageConvertFormat>,
    ) -> Result<Option<String>> {
        self.copyfile(ctx, src, src)?;
        let Some(img_fmt) = convert else {
            return Ok(None);
        };
        if !img_fmt.is_supported_convert_extension(src.extension()) {
            return Ok(None);
        }
        let class_regex = self.img_class_regex(ctx)?;
        let mut target_cover_size = (800, 800);

        if let Some(class) = class_regex
            .captures(img)
            .and_then(|c| c.get(1))
            .map(|c| c.as_str())
        {
            if class.contains("thumb") {
                target_cover_size = (240, 130);
            }
        }
        let mut new_src = src.to_path_buf();
        new_src.set_file_name(format!(
            "{}_{}x{}",
            new_src.file_stem().unwrap().to_str().unwrap(),
            target_cover_size.0,
            target_cover_size.1
        ));
        new_src.set_extension(img_fmt.extension());
        let (width, height) = self.convert_image(ctx, img_fmt, target_cover_size, src, &new_src)?;
        let mime_type = img_fmt.mime_type();
        let new_path_str = new_src.display();
        Ok(Some(format!(
            "<picture><source srcset=\"/{new_path_str}\" type=\"{mime_type}\"/>{} width={width} height={height}></picture>",
            img.strip_suffix(">").unwrap(),
        )))
    }

    #[jobber::job]
    fn render_template_html(&self, ctx: &mut JobCtx<'_>, src: &str, dst: &Path) -> Result<()> {
        #[cfg(feature = "job_print")]
        eprintln!("Render {src}");
        let templates = self.template_loader(ctx)?;
        let info = self.all_info(ctx)?;
        let mut render_ctx = tera::Context::from_serialize(info)?;
        render_ctx.insert("hot_reload", &self.include_hot_reload);
        self.render_template_html_common(ctx, &templates, &render_ctx, src, dst)
    }

    #[jobber::job]
    fn render_post(&self, ctx: &mut JobCtx<'_>, post_config: &PostConfig) -> Result<()> {
        #[cfg(feature = "job_print")]
        eprintln!("Render post {}", post_config.slug);
        let post = self.post_loader(ctx, post_config)?;
        let templates = self.template_loader(ctx)?;
        let mut render_ctx = tera::Context::from_serialize(post)?;
        render_ctx.insert("hot_reload", &self.include_hot_reload);
        let dst = Path::new("posts")
            .join(&post_config.slug)
            .join("index.html");
        self.render_template_html_common(ctx, &templates, &render_ctx, "post.html", &dst)
    }

    #[jobber::job]
    fn render_all_posts(&self, ctx: &mut JobCtx<'_>) -> Result<()> {
        let site_config = self.site_config_loader(ctx)?;
        for post_config in &site_config.pages.posts {
            self.render_post(ctx, post_config)?;
        }
        Ok(())
    }

    #[jobber::job]
    fn render_all_pages(&self, ctx: &mut JobCtx<'_>) -> Result<()> {
        let site_config = self.site_config_loader(ctx)?;
        for page in &site_config.pages.pages {
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
        #[cfg(feature = "job_print")]
        eprintln!("Render {src}");
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
        #[cfg(feature = "job_print")]
        eprintln!("Render {src}");
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
