use anyhow::{Context, Result};
use markdown::mdast::{Heading, Image, Node, Text};
use serde::Serialize;

use crate::config::PostConfig;

#[derive(Debug, Clone, Default, Serialize)]
pub struct PostDetails {
    pub slug: String,
    pub title: String,
    pub image: String,
    pub date: String,
    pub tags: Vec<String>,
    pub description: String,
    pub headings: Vec<PostHeading>,
    pub contents: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostHeading {
    label: String,
    kebab_label: String,
    depth: u8,
}

impl PostDetails {
    pub fn extract(post_config: &PostConfig, contents: &str) -> Result<PostDetails> {
        let node = markdown::to_mdast(contents, &markdown::ParseOptions::default())
            .expect("parse markdown");
        let slug = post_config.slug.to_owned();
        let title = extract_title(&node).context("extract title")?.to_owned();
        let image = extract_image(&node).context("extract image")?.to_owned();
        let date = extract_date(&node).context("extract date")?.to_owned();
        let tags = extract_tags(&node).context("extract tags")?.to_owned();
        let headings = extract_headings(&node).to_owned();
        let contents = add_heading_ids(
            markdown::to_html_with_options(contents, &markdown::Options::gfm())
                .map_err(|err| MarkdownToHtmlError { msg: err })?,
        );
        Ok(PostDetails {
            slug,
            title,
            image,
            date,
            tags,
            // TODO: Extract description
            description: String::default(),
            headings,
            contents,
        })
    }
}

#[derive(Debug)]
struct MarkdownToHtmlError {
    msg: String,
}

impl std::fmt::Display for MarkdownToHtmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl std::error::Error for MarkdownToHtmlError {}

fn find_map_ast<F, O>(node: &Node, checker: F) -> Option<O>
where
    F: Fn(&Node) -> Option<O> + Clone,
{
    fn helper<F, O>(node: &Node, checker: &F) -> Option<O>
    where
        F: Fn(&Node) -> Option<O>,
    {
        if let Some(output) = checker(node) {
            Some(output)
        } else {
            node.children()?
                .iter()
                .find_map(|child| helper(child, checker))
        }
    }
    helper(node, &checker)
}

fn filter_map_ast<F, O>(node: &Node, checker: F) -> Vec<O>
where
    F: Fn(&Node) -> Option<O>,
{
    let mut acc = Vec::default();
    fn helper<F, O>(node: &Node, checker: &F, acc: &mut Vec<O>)
    where
        F: Fn(&Node) -> Option<O>,
    {
        if let Some(o) = checker(node) {
            acc.push(o);
        }
        if let Some(children) = node.children() {
            for node in children {
                helper(node, checker, acc);
            }
        }
    }
    helper(node, &checker, &mut acc);
    acc
}

fn extract_text(node: &Node) -> Option<String> {
    find_map_ast(node, |node| {
        if let Node::Text(Text { value, .. }) = node {
            Some(value.to_owned())
        } else {
            None
        }
    })
}

fn extract_title(node: &Node) -> Option<String> {
    find_map_ast(node, |node| {
        if let Node::Heading(Heading { depth: 1, .. }) = node {
            extract_text(node)
        } else {
            None
        }
    })
}

fn extract_image(node: &Node) -> Option<String> {
    find_map_ast(node, |node| {
        if let Node::Image(Image { url, .. }) = node {
            Some(url.to_owned())
        } else {
            None
        }
    })
}

fn extract_date(node: &Node) -> Option<String> {
    find_map_ast(node, |node| {
        if let Node::Text(Text { value, .. }) = node {
            const MONTHS: &[&str] = &[
                "january",
                "february",
                "march",
                "april",
                "may",
                "june",
                "july",
                "august",
                "september",
                "october",
                "november",
                "december",
            ];
            if MONTHS.iter().any(|m| value.to_lowercase().contains(m)) {
                Some(value.to_owned())
            } else {
                None
            }
        } else {
            None
        }
    })
}

fn extract_tags(node: &Node) -> Option<Vec<String>> {
    Some(
        extract_text(node.children()?.get(2)?)?
            .split(',')
            .map(|s| s.to_owned())
            .collect(),
    )
}

fn extract_headings(node: &Node) -> Vec<PostHeading> {
    filter_map_ast(node, |node| {
        if let Node::Heading(Heading { depth, .. }) = node {
            let label = extract_text(node)?;
            Some(PostHeading {
                kebab_label: kebab(&label),
                label,
                depth: *depth,
            })
        } else {
            None
        }
    })
}

fn add_heading_ids(contents: String) -> String {
    let header_pattern = regex::Regex::new("<h([1-6])>([^<]+)</h").unwrap();
    header_pattern
        .replace_all(&contents, |cap: &regex::Captures<'_>| {
            let rank = cap.get(1).unwrap().as_str();
            let inner = cap.get(2).unwrap().as_str();
            let kebab_inner = kebab(inner);
            format!("<h{rank} id=\"{kebab_inner}\">{inner}</h")
        })
        .to_string()
}

fn kebab(s: &str) -> String {
    s.to_lowercase().replace(' ', "-")
}
