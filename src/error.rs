use lrlex::{LRLexError, DefaultLexeme};
use lrpar::{NonStreamingLexer, Span, LexerTypes};

#[derive(Debug, Clone)]
pub struct U32lexerType;

impl LexerTypes for U32lexerType {
    type StorageT = u32;
    type LexemeT = DefaultLexeme;
    type LexErrorT = LRLexError;
}

#[derive(Debug, Clone)]
pub struct SemanticError {
    span: Option<Span>,
    msg: String,
}

impl SemanticError {
    pub fn new(span: Option<Span>, msg: &str) -> Self {
        Self {
            span,
            msg: msg.to_owned(),
        }
    }

    pub fn display(&self, lexer: &dyn NonStreamingLexer<U32lexerType>) {
        match self.span {
            Some(s) => {
                let ((line, col), _) = lexer.line_col(s);
                eprintln!(
                    "Evaluation error at line {} column {}\n'{}'\n{}.",
                    line,
                    col,
                    lexer.span_str(s),
                    self.msg
                )
            }
            None => eprintln!("{}\nEvaluation error!", self.msg),
        }
    }
}
