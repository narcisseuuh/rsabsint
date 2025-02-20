/*
 * author : Narcisse.
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
use std::error::Error;
use std::fmt;
use lrlex::DefaultLexerTypes;
use lrpar::{NonStreamingLexer, Span};

/// Used to denote semantic errors at parsing phase.
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

    pub fn display(&self, lexer: &dyn NonStreamingLexer<DefaultLexerTypes<u32>>) {
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

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for SemanticError {}