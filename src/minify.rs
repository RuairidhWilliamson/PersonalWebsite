use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{ParserOptions, StyleSheet},
    traits::IntoOwned as _,
};
use oxc::{
    allocator::Allocator,
    codegen::{Codegen, CodegenOptions},
    minifier::{CompressOptions, MangleOptions, Minifier, MinifierOptions, MinifierReturn},
    parser::Parser,
    span::SourceType,
};

pub fn javascript(source: &str) -> anyhow::Result<String> {
    let allocator = Allocator::new();
    let parser = Parser::new(&allocator, source, SourceType::cjs());
    let parsed = parser.parse();
    if !parsed.errors.is_empty() {
        return Err(anyhow::anyhow!(
            "failed to parse javascript: {:?}",
            parsed.errors
        ));
    }
    let mut program = parsed.program;
    let minifier = Minifier::new(MinifierOptions {
        mangle: Some(MangleOptions {
            top_level: true,
            debug: false,
        }),
        compress: Some(CompressOptions {
            target: oxc::syntax::es_target::ESTarget::ES2022,
            drop_debugger: false,
            drop_console: false,
        }),
    });
    let MinifierReturn { scoping } = minifier.build(&allocator, &mut program);
    let js = Codegen::new()
        .with_options(CodegenOptions {
            minify: true,
            single_quote: true,
            comments: false,
            ..Default::default()
        })
        .with_scoping(scoping)
        .build(&program);
    Ok(js.code)
}

pub fn css(source: &str) -> anyhow::Result<String> {
    let stylesheet = StyleSheet::parse(source, ParserOptions::default()).map_err(|err| {
        lightningcss::error::Error {
            kind: err.kind.into_owned(),
            loc: err.loc,
        }
    })?;
    let result = stylesheet.to_css(PrinterOptions {
        minify: true,
        ..Default::default()
    })?;
    Ok(result.code)
}
