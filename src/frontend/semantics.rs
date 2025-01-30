use crate::{ast::*, error::SemanticError};
use crate::symbol::*;
use crate::typing::Type;
use lrlex::DefaultLexerTypes;
use lrpar::{NonStreamingLexer, Span};

pub fn insert(
    mut id : SymbolBuilder,
    typ : Type,
    sym_table: &mut SymbolTable,
    lexer: &dyn NonStreamingLexer<DefaultLexerTypes>,
) -> Result<(), SemanticError> {
    id.dtype(typ.clone());
    let span = id.get_name();

    sym_table
        .insert_builder(id, lexer)
        .map_err(|msg| SemanticError::new(Some(span), &msg))?;

    Ok(())
}

pub fn create_int_binop(
    op: IntBinaryOp,
    span: Span,
    left: IntExpr,
    right: IntExpr
) -> Result<IntExpr, SemanticError> {
    Ok(IntExpr::Binary {
        op,
        span,
        lhs: Box::new(left),
        rhs: Box::new(right),
    })
}

pub fn create_int_unop(
    op: IntUnaryOp,
    span: Span,
    exp: IntExpr
) -> Result<IntExpr, SemanticError> {
    Ok(IntExpr::Unary {
        op,
        span,
        exp: Box::new(exp),
    })
}

pub fn create_bool_binop(
    op: BoolBinaryOp,
    span: Span,
    left: BoolExpr,
    right: BoolExpr
) -> Result<BoolExpr, SemanticError> {
    Ok(BoolExpr::Binary {
        op,
        span,
        lhs: Box::new(left),
        rhs: Box::new(right),
    })
}

pub fn create_bool_unop(
    op: BoolUnaryOp,
    span: Span,
    exp: BoolExpr
) -> Result<BoolExpr, SemanticError> {
    Ok(BoolExpr::Unary {
        op,
        span,
        exp: Box::new(exp),
    })
}

pub fn create_bool_compare(
    op: CompareOp,
    span: Span,
    lhs: IntExpr,
    rhs: IntExpr
) -> Result<BoolExpr, SemanticError> {
    Ok(BoolExpr::Compare {
        op,
        span,
        lhs,
        rhs,
    })
}

pub fn create_while(
    cond: BoolExpr,
    body: TNode
) -> Result<TNode, SemanticError> {
    Ok(TNode::While { 
        cond,
        body: Box::new(body),
    })
}

pub fn create_if(
    cond: BoolExpr,
    then: Box<TNode>,
    otherwise: Option<Box<TNode>>
) -> Result<TNode, SemanticError> {
    Ok(TNode::If {
        cond,
        then,
        otherwise
    })
}
