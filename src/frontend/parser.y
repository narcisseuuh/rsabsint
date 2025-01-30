%start PROG
%parse-param p: &RefCell<ParserState>

%avoid_insert "INT_T"
%avoid_insert "IDENT"
%avoid_insert "INT"

%epp TRUEE    "true"
%epp FALSEE   "false" 
%epp IF       "if"      
%epp ELSE     "else"    
%epp WHILE    "while"   
%epp ASSERT   "assert"  
%epp PRINT    "print"   
%epp HALT     "halt"    
%epp RAND     "rand"

%token "(" ")" "," ";" "{" "}"
%left "&&" "||"
%left "==" "!="
%left "<" ">" "<=" ">="
%left "+" "-"
%left "*" "/" "%"
%left "!"

%nonassoc IF
%nonassoc ELSE

%%
// Statements
PROG -> Result<Program, SemanticError>:
      STMT PROG  { insert_vec($2, $1) }
    | STMT       { Ok(vec![$1?]) }
    ;

STMT -> Result<TNode, SemanticError>:
      BLOCK                        { $1 }
    | Id "=" IE ";"                       
        {
            let parser = &mut *p.borrow_mut();
            let fname = $lexer.span_str($1?.span()).to_string();
            let symbol = parser.sym_table
                .get(&fname)
                .unwrap()
                .clone();
            Ok(TNode::Assign { lhs: symbol, rhs: $3? })
        }
    | "IF" "(" BE ")" STMT %prec IF
        { Ok(TNode::If { cond: $3?, then: Box::new($5?), otherwise: None }) }
    | "IF" "(" BE ")" STMT "ELSE" STMT %prec ELSE
        { Ok(TNode::If { cond: $3?, then: Box::new($5?), otherwise: Some(Box::new($7?)) }) }
    | "WHILE" "(" BE ")" STMT      { Ok(TNode::While { cond: $3?, body: Box::new($5?) }) }
    | "ASSERT" "(" BE ")" ";"      { Ok(TNode::Assert { cond: $3? }) }
    | "PRINT" "(" ID_LIST ")" ";"  { Ok(TNode::Print { vars: $3? }) }
    | "HALT" ";"                   { Ok(TNode::Halt) }
    ;

STMT_LIST -> Result<Vec<TNode>, SemanticError>:
      STMT              { Ok(vec![$1?]) }
    | STMT STMT_LIST    { insert_vec($2, $1) }
    ;

ID_LIST -> Result<Vec<Symbol>, SemanticError>:
       Id                  
        { 
            let parser = &mut *p.borrow_mut();
            let fname = $lexer.span_str($1?.span()).to_string();
            let symbol = parser.sym_table
                .get(&fname)
                .unwrap()
                .clone();
            Ok(vec![symbol])
        }
    | Id "," ID_LIST       
        {
            let parser = &mut *p.borrow_mut();
            let fname = $lexer.span_str($1?.span()).to_string();
            let symbol = parser.sym_table
                .get(&fname)
                .unwrap()
                .clone();
            insert_vec($3, Ok(symbol))
        }
    ;

// blocks
BLOCK -> Result<TNode, SemanticError>:
    "{" DECL_LIST STMT_LIST "}"  { Ok(TNode::Block { decl : $2?, stmt : $3? }) }
    ;

DECL -> Result<Symbol, SemanticError>:
    TYPE SYMDEF ";"         
        {
            let parser = &mut *p.borrow_mut();
            let _ = insert($2.clone()?, $1?, &mut parser.sym_table, $lexer);
            let fname = $lexer.span_str($2?.get_name()).to_string();
            let symbol = parser.sym_table
                .get(&fname)
                .unwrap()
                .clone();
            Ok(symbol)
        }
    ;

DECL_LIST -> Result<Vec<Symbol>, SemanticError>:
      DECL              { Ok(vec![$1?]) }
    | DECL DECL_LIST    { insert_vec($2, $1) }
    ;

TYPE -> Result<Type, SemanticError>:
    "INT_T"             { Ok(Type::Int) }
    ;

// Expression grammar
IE -> Result<IntExpr, SemanticError>:
      IE "+" IE                  { create_int_binop(IntBinaryOp::Add, $span, $1?, $3?) }
    | IE "-" IE                  { create_int_binop(IntBinaryOp::Sub, $span, $1?, $3?) }
    | IE "*" IE                  { create_int_binop(IntBinaryOp::Mul, $span, $1?, $3?) }
    | IE "/" IE                  { create_int_binop(IntBinaryOp::Div, $span, $1?, $3?) }
    | IE "%" IE                  { create_int_binop(IntBinaryOp::Mod, $span, $1?, $3?) }
    | "-" IE                     { create_int_unop(IntUnaryOp::SubUnary, $span, $2?) }
    | "+" IE                     { create_int_unop(IntUnaryOp::AddUnary, $span, $2?) }
    | "RAND" "(" Num "," Num ")" 
        {
            let num1 = $lexer.span_str($3?.span()).to_string();
            let num2 = $lexer.span_str($5?.span()).to_string();
            Ok(IntExpr::Rand { span: $span, lower: num1, upper: num2 })
        }
    | Num                        { Ok(IntExpr::Const { span: $span, cst: $lexer.span_str($1?.span()).to_string() }) }
    | Id                         
        {
            let parser = &mut *p.borrow_mut();
            let fname = $lexer.span_str($1?.span()).to_string();
            let maybe_symbol = parser.sym_table.get(&fname);
            if let Some(symbol) = maybe_symbol {
                Ok(IntExpr::Ident { span : $span, var : symbol.clone() })
            }
            else {
                Err(SemanticError::new(
                    Some($span),
                    format!("unknown variable {}", fname).as_str()
                ))
            }
        }
    ;

BE -> Result<BoolExpr, SemanticError>:
      IE "==" IE                { create_bool_compare(CompareOp::EQ, $span, $1?, $3?) }
    | IE "!=" IE                { create_bool_compare(CompareOp::NE, $span, $1?, $3?) }
    | IE ">=" IE                { create_bool_compare(CompareOp::GE, $span, $1?, $3?) }
    | IE ">" IE                 { create_bool_compare(CompareOp::GT, $span, $1?, $3?) }
    | IE "<=" IE                { create_bool_compare(CompareOp::LE, $span, $1?, $3?) }
    | IE "<" IE                 { create_bool_compare(CompareOp::LT, $span, $1?, $3?) }
    | BE "&&" BE                { create_bool_binop(BoolBinaryOp::And, $span, $1?, $3?) }
    | BE "||" BE                { create_bool_binop(BoolBinaryOp::Or, $span, $1?, $3?) }
    | "!" BE                    { create_bool_unop(BoolUnaryOp::Not, $span, $2?) }
    | "(" BE ")"                { $2 }
    | "TRUEE"                   { Ok(BoolExpr::Const { span: $span, cst: true }) }
    | "FALSEE"                  { Ok(BoolExpr::Const { span: $span, cst: false }) }
    ;

// Symbol definition
SYMDEF -> Result<SymbolBuilder, SemanticError>:
    Id               { let s = SymbolBuilder::new($1?.span()); Ok(s) }
    ;

// LEXEME RESOLVE
Id -> Result<DefaultLexeme<u32>, SemanticError>:
    "IDENT"          { $1.map_err(|e| SemanticError::new(Some(e.span()), "Faulty lexeme")) }
    ;

Num -> Result<DefaultLexeme<u32>, SemanticError>:
    "INT"            { $1.map_err(|e| SemanticError::new(Some(e.span()), "Faulty lexeme")) }
    ;
%%

// Any functions here are in scope for all the grammar actions above.

use lrlex::DefaultLexeme;
use crate::{
    ast::*,
    symbol::*,
    error::SemanticError,
    frontend::semantics::*,
    frontend::parser_state::*,
    typing::*,
};
use crate::frontend::file_parser::RefCell;

fn insert_vec<T>(
    rhs: Result<Vec<T>, SemanticError>,
    lhs: Result<T, SemanticError>,
) -> Result<Vec<T>, SemanticError> {
    let mut flt : Vec<T> = rhs?;
    flt.push(lhs?);
    Ok(flt)
}