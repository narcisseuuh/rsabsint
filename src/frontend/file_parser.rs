use lrlex::lrlex_mod;
use lrpar::lrpar_mod;
use crate::error::SemanticError;
use crate::{
    frontend::parser_state::ParserState,
    ast::Program
};
use std::{cell::RefCell, env, error::Error, ffi::OsStr, 
    fs::File, io::Read, path::PathBuf};
use std::fmt;

#[derive(Debug)]
struct ParsingError {
    details : String
}

impl ParsingError {
    fn new(msg: &str) -> ParsingError {
        ParsingError{details: msg.to_string()}
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ParsingError {
    fn description(&self) -> &str {
        &self.details
    }
}

lrlex_mod!("frontend/lexer.l");
lrpar_mod!("frontend/parser.y");

fn get_input(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let mut fd = File::open(path)?;
    let mut s = String::new();
    fd.read_to_string(&mut s)?;
    Ok(s)
}

/// Function taking a filename as argument and returning the AST corresponding to the program
/// in case no error is raised at lexing/parsing time, the error corresponding to the parse 
/// failure otherwise.
pub fn parse_file(
    filename: String
) -> Result<Program, Box<(dyn std::error::Error + 'static)>> {
    let input_file = PathBuf::from(filename);

    match input_file.extension().and_then(OsStr::to_str) {
        Some("c") => {}
        _ => {
            return Err(Box::new(ParsingError::new("c file wasn\'t provided!")));
        }
    }

    let input = get_input(&input_file)?;

    let lexerdef = lexer_l::lexerdef();
    let lexer = lexerdef.lexer(&input);

    let p = RefCell::new(ParserState::default());
    let (res, errs) = parser_y::parse(&lexer, &p);

    if !errs.is_empty() {
        for e in errs {
            println!("{}", e.pp(&lexer, &parser_y::token_epp));
        }
        return Err(Box::new(ParsingError::new("Unable to analyze program!")));
    }

    let ret_val = 
        match res {
            Some(Ok(program)) => Ok(program),
            Some(Err(e)) => Err(Box::new(e)),
            None => Err(
                Box::new(SemanticError::new(None, "Unable to parse file."))
            ),
        };

    Ok(ret_val?)
}