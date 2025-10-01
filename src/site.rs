use std::{io::Cursor, path::Path, str, sync::Arc};

use anyhow::{Context as _, Result};
use harper_core::{
    DictWordMetadata, Document, Span,
    linting::{LintGroup, Linter as _},
    parsers::MarkdownOptions,
    spell::{FstDictionary, MergedDictionary},
};
use image::{DynamicImage, GenericImageView as _};
use jobber::{Cache, JobCtx, JobIdBuilder};
use serde::{Deserialize, Serialize};

use crate::{
    config::{BuildConfig, ImageConvertFormat, PostConfig, SiteConfig},
    post::PostDetails,
    progress::{DefaultSiteBuildProgress, NoSiteBuildProgress, SiteBuildProgress},
};

#[derive(Debug, Clone, Serialize)]
struct Info {
    details: crate::config::Details,
    posts: Vec<PostDetails>,
    featured: Vec<PostDetails>,
}

#[derive(Deserialize)]
pub struct AdditionalDictionary {
    pub words: Vec<AdditionalWord>,
}

#[derive(Deserialize)]
pub struct AdditionalWord {
    pub word: String,
    pub metadata: DictWordMetadata,
}

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
        if self.config.no_progress {
            self.build_site_with_cache_with_progress(cache, &NoSiteBuildProgress)
        } else {
            self.build_site_with_cache_with_progress(cache, &DefaultSiteBuildProgress)
        }
    }

    fn build_site_with_cache_with_progress<P: SiteBuildProgress>(
        &self,
        cache: &Cache,
        progress: &P,
    ) -> Result<u64> {
        let output = cache.root_job_with_progress(
            JobIdBuilder::new("build_site").build(),
            progress,
            |ctx| self.build_site(ctx),
        )?;
        progress.report_built(&output);

        Ok(output.hash)
    }

    fn build_site(&self, ctx: &mut JobCtx<'_>) -> Result<()> {
        self.download_third_party_asset(
            ctx,
            "https://cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.min.css",
            Path::new("thirdparty/normalize.min.css"),
        )?;
        self.download_third_party_asset(
            ctx,
            "https://cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.min.css.map",
            Path::new("thirdparty/normalize.min.css.map"),
        )?;
        self.download_third_party_asset(
            ctx,
            "https://cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.css",
            Path::new("thirdparty/normalize.css"),
        )?;
        self.download_third_party_asset(
            ctx,
            "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/default.min.css",
            Path::new("thirdparty/highlight.min.css"),
        )?;
        self.download_third_party_asset(
            ctx,
            "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/tokyo-night-dark.min.css",
            Path::new("thirdparty/highlight-tokyo-night-dark.min.css"),
        )?;
        self.download_third_party_asset(
            ctx,
            "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js",
            Path::new("thirdparty/highlight.min.js"),
        )?;
        self.copyfile(
            ctx,
            Path::new("assets/thirdparty/rubik-variablefont.ttf"),
            Path::new("thirdparty/rubik-variablefont.ttf"),
        )?;
        self.copyfile(
            ctx,
            Path::new("assets/favicon.ico"),
            Path::new("favicon.ico"),
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
        cfg.validate()?;
        Ok(cfg)
    }

    #[jobber::job]
    fn post_markdown(&self, ctx: &mut JobCtx<'_>, post_config: &PostConfig) -> Result<String> {
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
        Ok(contents)
    }

    #[jobber::job]
    fn post_loader(&self, ctx: &mut JobCtx<'_>, post_config: &PostConfig) -> Result<PostDetails> {
        let contents = self.post_markdown(ctx, post_config)?;
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
    fn download_third_party_asset(
        &self,
        ctx: &mut JobCtx<'_>,
        url: &str,
        dst: &Path,
    ) -> Result<()> {
        let response = reqwest::blocking::get(url)?;
        let mut bytes = Cursor::new(response.error_for_status()?.bytes()?);

        let destination = self.config.output_dir.join(dst);
        if let Some(dir) = destination.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut destination = std::fs::File::create(destination)?;
        std::io::copy(&mut bytes, &mut destination)?;
        Ok(())
    }

    #[jobber::job]
    fn copyfile(&self, ctx: &mut JobCtx<'_>, src: &Path, dst: &Path) -> Result<()> {
        log::info!("Copyfile {src:?} -> {dst:?}");
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
        let mut result = Result::Ok(());
        rendered = img_regex
            .replace_all(&rendered, |cap: &regex::Captures<'_>| {
                let img = cap
                    .get(0)
                    .expect("regex capture")
                    .as_str()
                    .replace("&#x2F;", "/");
                let path = cap
                    .get(1)
                    .expect("regex capture")
                    .as_str()
                    .replace("&#x2F;", "/");
                path.strip_prefix('/')
                    .map(Path::new)
                    .and_then(|src| {
                        self.replace_img(ctx, &img, src, &site_config.convert_images)
                            .unwrap_or_else(|err| {
                                if result.is_ok() {
                                    result = Err(err);
                                }
                                None
                            })
                    })
                    .unwrap_or_else(|| img.clone())
            })
            .to_string();
        let rendered_bytes = if self.config.minify {
            super::minify::html(&rendered)
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

    fn calculate_img_size(img: &DynamicImage, target_cover_size: (u32, u32)) -> (u32, u32) {
        let (w, h) = img.dimensions();
        let new_w = w.min(target_cover_size.0);
        let new_h = new_w * h / w;
        let new_new_h = h.min(new_h);
        let new_new_w = new_new_h * new_w / new_h;
        (new_new_w, new_new_h)
    }

    #[jobber::job]
    fn replace_img(
        &self,
        ctx: &mut JobCtx<'_>,
        img_html: &str,
        src: &Path,
        convert: &[ImageConvertFormat],
    ) -> Result<Option<String>> {
        self.copyfile(ctx, src, src)?;
        if convert.is_empty() {
            return Ok(None);
        }
        let class_regex = self.img_class_regex(ctx)?;
        let mut target_cover_size = (800, 800);

        if let Some(class) = class_regex
            .captures(img_html)
            .and_then(|c| c.get(1))
            .map(|c| c.as_str())
            && class.contains("thumb")
        {
            target_cover_size = (240, 130);
        }
        let source = self.config.root_dir.join(src);
        ctx.depends_file(&source)?;
        let img = image::ImageReader::open(&source)?.decode()?;
        let (width, height) = Self::calculate_img_size(&img, target_cover_size);

        let mut sources = Vec::new();
        for img_fmt in convert {
            if !img_fmt.is_supported_convert_extension(src.extension()) {
                continue;
            }
            let mut new_src = src.to_path_buf();
            new_src.set_file_name(format!(
                "{}_{}x{}",
                new_src.file_stem().context("file stem")?.to_string_lossy(),
                target_cover_size.0,
                target_cover_size.1
            ));
            new_src.set_extension(img_fmt.extension());
            self.convert_image(&img, (width, height), img_fmt, &new_src)?;
            let mime_type = img_fmt.mime_type();
            let new_path_str = new_src.display();
            sources.push(format!(
                "<source srcset=\"/{new_path_str}\" type=\"{mime_type}\"/>"
            ));
        }
        if sources.is_empty() {
            return Ok(None);
        }
        Ok(Some(format!(
            "<picture>{sources}{fallback} width={width} height={height}></picture>",
            sources = sources.join(""),
            fallback = img_html
                .strip_suffix(">")
                .context("strip suffx >")?
                .trim_end_matches('/')
                .trim(),
        )))
    }

    fn convert_image(
        &self,
        src: &DynamicImage,
        size: (u32, u32),
        ty: &ImageConvertFormat,
        dst: &Path,
    ) -> Result<()> {
        let destination = self.config.output_dir.join(dst);
        if let Some(dir) = destination.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut out = std::io::BufWriter::new(std::fs::File::create(destination)?);
        let img_fmt = match ty {
            ImageConvertFormat::Webp => image::ImageFormat::WebP,
            ImageConvertFormat::Avif => image::ImageFormat::Avif,
        };
        src.resize_to_fill(size.0, size.1, image::imageops::FilterType::Lanczos3)
            .write_to(&mut out, img_fmt)?;
        Ok(())
    }

    #[jobber::job]
    fn render_template_html(&self, ctx: &mut JobCtx<'_>, src: &str, dst: &Path) -> Result<()> {
        log::info!("Render {src}");
        let templates = self.template_loader(ctx)?;
        let info = self.all_info(ctx)?;
        let mut render_ctx = tera::Context::from_serialize(info)?;
        render_ctx.insert("hot_reload", &self.include_hot_reload);
        self.render_template_html_common(ctx, &templates, &render_ctx, src, dst)
    }

    #[jobber::job]
    fn render_post(&self, ctx: &mut JobCtx<'_>, post_config: &PostConfig) -> Result<()> {
        log::info!("Render post {}", post_config.slug);
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
            if self.config.grammar_check {
                self.spell_check_post(ctx, post_config)?;
            }
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
        log::info!("Render {src}");
        let templates = self.template_loader(ctx)?;
        let info = self.all_info(ctx)?;
        let mut render_ctx = tera::Context::from_serialize(info)?;
        render_ctx.insert("hot_reload", &self.include_hot_reload);
        let rendered = templates.render(src, &render_ctx)?;
        let rendered_bytes = if self.config.minify {
            // super::npm::minify_js(&rendered)?
            crate::minify::javascript(&rendered)?.into_bytes()
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
        log::info!("Render {src}");
        let templates = self.template_loader(ctx)?;
        let info = self.all_info(ctx)?;
        let mut render_ctx = tera::Context::from_serialize(info)?;
        render_ctx.insert("hot_reload", &self.include_hot_reload);
        let rendered = templates.render(src, &render_ctx)?;
        let rendered_bytes = if self.config.minify {
            super::minify::css(&rendered)?
        } else {
            rendered
        };
        let destination = self.config.output_dir.join(dst);
        if let Some(dir) = destination.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(destination, rendered_bytes)?;
        Ok(())
    }

    #[jobber::job]
    fn dictionary(&self, ctx: &mut JobCtx<'_>) -> Result<MergedDictionary> {
        let dictionary_path = self.config.root_dir.join("dictionary.toml");
        ctx.depends_file(&dictionary_path)?;
        let dictionary_contents = std::fs::read_to_string(dictionary_path)?;
        let additional_dict: AdditionalDictionary = toml::from_str(&dictionary_contents)?;

        let simple_dictionary_path = self.config.root_dir.join("dictionary.txt");
        ctx.depends_file(&simple_dictionary_path)?;
        let simple_dictionary_contents = std::fs::read_to_string(simple_dictionary_path)?;

        let mut dict: MergedDictionary = MergedDictionary::new();
        dict.add_dictionary(FstDictionary::curated());
        dict.add_dictionary(Arc::new(FstDictionary::new(
            additional_dict
                .words
                .into_iter()
                .map(|w| (w.word.chars().collect::<_>(), w.metadata))
                .collect(),
        )));
        dict.add_dictionary(Arc::new(FstDictionary::new(
            simple_dictionary_contents
                .lines()
                .map(|l| (l.chars().collect::<_>(), DictWordMetadata::default()))
                .collect(),
        )));
        Ok(dict)
    }

    #[jobber::job]
    fn spell_ignore_list(&self, ctx: &mut JobCtx<'_>) -> Result<Vec<String>> {
        let spell_ignore_path = self.config.root_dir.join("spell_ignore.txt");
        ctx.depends_file(&spell_ignore_path)?;
        let spell_ignore_list = std::fs::read_to_string(spell_ignore_path)?;
        Ok(spell_ignore_list
            .lines()
            .map(str::to_owned)
            .collect::<Vec<String>>())
    }

    #[jobber::job]
    fn spell_check_post(&self, ctx: &mut JobCtx<'_>, post_config: &PostConfig) -> Result<()> {
        let dict = self.dictionary(ctx)?;
        let spell_ignore_list = self.spell_ignore_list(ctx)?;
        let contents = self.post_markdown(ctx, post_config)?;
        let mut md_options = MarkdownOptions::default();
        md_options.ignore_link_title = true;
        let document = Document::new_markdown(&contents, md_options, &dict);

        let mut linter = LintGroup::new_curated(Arc::new(dict), harper_core::Dialect::British);
        linter.config.set_rule_enabled("OxfordComma", false);
        linter.config.set_rule_enabled("NoOxfordComma", true);
        linter.config.set_rule_enabled("CompoundNouns", false);
        let lints = linter.lint(&document);
        let mut count = 0;
        for l in lints {
            let expanded_span = Span::new(
                l.span.start.saturating_sub(10),
                (l.span.end + 10).min(document.get_source().len() - 1),
            );
            let target = document.get_span_content_str(&expanded_span);
            if spell_ignore_list.iter().any(|s| target.contains(s)) {
                continue;
            }
            if l.message.starts_with("Vocabulary enhancement") {
                continue;
            }
            log::error!(
                "{}\n {target}\nConsider one of\n{}",
                l.message,
                l.suggestions
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join("\n")
            );
            count += 1;
        }
        if count == 0 {
            return Ok(());
        }
        let slug = &post_config.slug;
        return Err(anyhow::anyhow!("post {slug} has spelling errors"));
    }
}
