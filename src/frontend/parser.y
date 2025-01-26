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

%token "(" ")" "," ";" "&" "{" "}"
%left "&&" "||"
%left "==" "!="
%left "<" ">" "<=" ">="
%left "+" "-"
%left "*" "/" "%"

%%
// Statements
PROG -> Result<Program, SemanticError>:
    STMT PROG    { insert($2, $1) }
    | STMT       { Ok(Vec::new($1?)) }
    ;

STMT -> Result<TNode, SemanticError>:
      BLOCK                               { $1 }
    | Id "=" IE ";"                       { Ok(TNode::Assign { lhs: $1?, rhs: $3? }) }
    | "IF" "(" BE ")" STMT                { Ok(TNode::If { cond: $3?, then: $5?, otherwise: None }) }
    | "IF" "(" BE ")" STMT "ELSE" STMT    { Ok(TNode::If { cond: $3?, then: $5?, otherwise: Some($7?) }) }
    | "WHILE" "(" BE ")" STMT             { Ok(TNode::While { cond: $3?, body: Box::new($5?) }) }
    | "ASSERT" "(" BE ")" ";"             { Ok(TNode::Assert { cond: $3? }) }
    | "PRINT" "(" Id_list ")" ";"         { Ok(TNode::Print { vars: $3? }) }
    | "HALT" ";"                          { Ok(TNode::Halt) }
    ;

STMT_LIST -> Result<Vec<TNode>, SemanticError>:
      STMT              { Ok(Vec::new($1?)) }
    | STMT STMT_LIST    { insert($2, $1) }
    ;

ID_LIST -> Result<Vec<Symbol>, SemanticError>:
       Id                {  }
    | Id "," ID_LIST     {  }
    ;

// blocks
BLOCK -> Result<TNode, SemanticError>:
    "{" DECL_LIST STMT_LIST "}"  { Ok(TNode::Block($2?, $3?)) }
    ;

DECL -> Result<Symbol, SemanticError>:
    TYPE Id ";"         {  }
    ;

DECL_LIST -> Result<Vec<Symbol>, SemanticError>:
      DECL              { Ok(Vec::new($1?)) }
    | DECL DECL_LIST    { insert($2, $1) }
    ;

TYPE -> Type:
    "INT_T"             { Ok(Type::Int) }
    ;

// Expression grammar
IE -> Result<IntExpr, SemanticError>:
      IE "+" IE                 { create_int(IntBinaryOp::Add, $span, $1?, $3?) }
    | IE "-" IE                 { create_int_binop(IntBinaryOp::Sub, $span, $1?, $3?) }
    | IE "*" IE                 { create_int_binop(IntBinaryOp::Mul, $span, $1?, $3?) }
    | IE "/" IE                 { create_int_binop(IntBinaryOp::Div, $span, $1?, $3?) }
    | IE "%" IE                 { create_int_binop(IntBinaryOp::Mod, $span, $1?, $3?) }
    | "-" IE                    { create_int_unop(IntUnaryOp::SubUnary, $span, $2?) }
    | "+" IE                    { create_int_unop(IntUnaryOp::AddUnary, $span, $2) }
    | Num                       {  }
    | Id                        {  }
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
    | TRUEE                     { Ok(BoolExpr::Const { span: $span, cst: true }) }
    | FALSEE                    { Ok(BoolExpr::Const { span: $span, cst: false }) }
    ;

// LEXEME RESOLVE
Id -> Result<DefaultLexeme<u32>, SemanticError>:
    "IDENT"          { $1.map_err(|e| SemanticError::new(Some(e.span()), "Faulty lexeme")) }
    ;

Num -> Result<DefaultLexeme<u32>, SemanticError>:
    "INT"            { $1.map_err(|e| SemanticError::new(Some(e.span()), "Faulty lexeme")) }
    ;

Unmatched -> ():
    "UNMATCHED" { }
    ;
%%

// Any functions here are in scope for all the grammar actions above.

use lrlex::DefaultLexeme;
use lrpar::{NonStreamingLexer, Span};
use rsabsint::{
    ast::*,
    symbol::*,
    error::SemanticError,
    frontend::semantics,
};

fn insert<T>(
    rhs: Result<Vec<T>, SemanticError>,
    lhs: Result<T, SemanticError>,
) -> Result<Vec<T>, SemanticError> {
    let mut flt : Vec<T> = rhs?;
    flt.push(lhs?);
    Ok(flt)
}