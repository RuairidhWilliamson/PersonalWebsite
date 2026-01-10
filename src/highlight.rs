use tree_sitter_highlight::HighlightConfiguration;

const HIGHLIGHT_NAMES: &[&str; 52] = &[
    "attribute",
    "boolean",
    "carriage-return",
    "comment",
    "comment.documentation",
    "constant",
    "constant.builtin",
    "constructor",
    "constructor.builtin",
    "embedded",
    "error",
    "escape",
    "function",
    "function.builtin",
    "keyword",
    "markup",
    "markup.bold",
    "markup.heading",
    "markup.italic",
    "markup.link",
    "markup.link.url",
    "markup.list",
    "markup.list.checked",
    "markup.list.numbered",
    "markup.list.unchecked",
    "markup.list.unnumbered",
    "markup.quote",
    "markup.raw",
    "markup.raw.block",
    "markup.raw.inline",
    "markup.strikethrough",
    "module",
    "number",
    "operator",
    "property",
    "property.builtin",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "string",
    "string.escape",
    "string.regexp",
    "string.special",
    "string.special.symbol",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.member",
    "variable.parameter",
];

fn language_config(language_name: &str) -> Option<HighlightConfiguration> {
    match language_name {
        "rust" => Some(
            HighlightConfiguration::new(
                tree_sitter_rust::LANGUAGE.into(),
                "rust",
                tree_sitter_rust::HIGHLIGHTS_QUERY,
                "",
                "",
            )
            .unwrap(),
        ),
        "rain" => Some(
            HighlightConfiguration::new(
                tree_sitter_rain::LANGUAGE.into(),
                "rain",
                tree_sitter_rain::HIGHLIGHTS_QUERY,
                "",
                "",
            )
            .unwrap(),
        ),
        _ => None,
    }
}

pub fn src_to_highlight_html(language_name: &str, src: &str) -> Option<String> {
    let mut highlight_config: HighlightConfiguration = language_config(language_name)?;
    highlight_config.configure(HIGHLIGHT_NAMES);
    let mut highlighter = tree_sitter_highlight::Highlighter::new();
    let events = highlighter
        .highlight(&highlight_config, src.as_bytes(), None, |_| None)
        .unwrap();
    let mut renderer = tree_sitter_highlight::HtmlRenderer::new();
    renderer
        .render(events, &src.as_bytes(), &move |highlight, output| {
            output.extend(b"class='");

            let mut parts = HIGHLIGHT_NAMES[highlight.0].split('.').peekable();
            while let Some(part) = parts.next() {
                output.extend(part.as_bytes());
                if parts.peek().is_some() {
                    output.extend(b" ");
                }
            }
            output.extend(b"'");
        })
        .unwrap();

    let highlighted_html = std::str::from_utf8(&renderer.html).unwrap();
    Some(format!(
        "<pre><code class=\"nohighlight tree-sitter\">{highlighted_html}</code></pre>"
    ))
}
