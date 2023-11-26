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
    pub contents: String,
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
        let contents = markdown::to_html(contents);
        Ok(PostDetails {
            slug,
            title,
            image,
            date,
            tags,
            // TODO: Extract description
            description: String::default(),
            contents,
        })
    }
}

fn find_map_ast<F, O>(node: &Node, checker: F) -> Option<O>
where
    F: Fn(&Node) -> Option<O> + Clone,
{
    if let Some(output) = checker(node) {
        Some(output)
    } else {
        node.children()?
            .iter()
            .find_map(|child| find_map_ast(child, checker.clone()))
    }
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
