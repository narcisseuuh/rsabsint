use cfgrammar::yacc::YaccKind;
use lrlex::{CTLexerBuilder, DefaultLexerTypes};
use lrpar::{unstable_api::UnstableApi, CTParser, CTParserBuilder};

/// function building the .rs files corresponding to the parser/lexer.
/// it is precised in `Cargo.toml` that it should be called before building.
/// the if let pattern is shown as irrefutable, even if it is not...
#[allow(irrefutable_let_patterns)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser : CTParser =
        CTParserBuilder::<DefaultLexerTypes>::new()
        .error_on_conflicts(false)
        .grammar_path("src/frontend/parser.y")
        .yacckind(YaccKind::Grmtools)
        .output_path("conflicts.rs")
        .build()?;

    /*
    Pretty-printing conflicts at build time.
    */
    if let Some(grammar) = parser.conflicts(UnstableApi) {
        println!("grammar conflicts detected : {:?}", grammar.3);
    };

    CTLexerBuilder::new()
        .lrpar_config(|ctp| {
            ctp.yacckind(YaccKind::Grmtools)
                .grammar_in_src_dir("frontend/parser.y")
                .unwrap()
        })
        .lexer_in_src_dir("frontend/lexer.l")?
        .build()?;

    Ok(())
}