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
    let MinifierReturn { symbol_table } = minifier.build(&allocator, &mut program);
    let js = Codegen::new()
        .with_options(CodegenOptions {
            minify: true,
            single_quote: true,
            comments: false,
            ..Default::default()
        })
        .with_symbol_table(symbol_table)
        .build(&program);
    Ok(js.code)
}
