use lrpar::Span;
use crate::symbol::*;

#[derive(Debug, Clone, Copy)]
pub enum IntBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, Copy)]
pub enum IntUnaryOp {
    AddUnary,
    SubUnary,
}

#[derive(Debug, Clone, Copy)]
pub enum CompareOp {
    NE,
    GT,
    GE,
    LT,
    LE,
    EQ,
}

#[derive(Debug, Clone, Copy)]
pub enum BoolBinaryOp {
    And,
    Or,
}

#[derive(Debug, Clone, Copy)]
pub enum BoolUnaryOp {
    Not,
}

#[derive(Debug, Clone, Copy)]
pub enum RefType {
    RHS,
    LHS,
}

#[derive(Debug, Clone)]
pub enum IntExpr {
    Unary {
        span: Span,
        op: IntUnaryOp,
        exp: Box<IntExpr>,
    },
    Binary {
        span: Span,
        op: IntBinaryOp,
        lhs: Box<IntExpr>,
        rhs: Box<IntExpr>,
    },
    Ident {
        span: Span,
        var: Symbol,
    },
    Const {
        span: Span,
        cst: String,
    },
    Rand {
        span: Span,
        lower: String,
        upper: String,
    }
}

impl IntExpr {
    pub fn get_span(&self) -> &Span {
        match self {
            IntExpr::Unary { span, .. }
            | IntExpr::Binary { span, .. }
            | IntExpr::Ident { span, .. }
            | IntExpr::Const { span, .. }
            | IntExpr::Rand { span, .. }
                => span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BoolExpr {
    Unary {
        span: Span,
        op: BoolUnaryOp,
        exp: Box<BoolExpr>,
    },
    Binary {
        span: Span,
        op: BoolBinaryOp,
        lhs: Box<BoolExpr>,
        rhs: Box<BoolExpr>,
    },
    Compare {
        span: Span,
        op: CompareOp,
        lhs: IntExpr,
        rhs: IntExpr,
    },
    Const {
        span: Span,
        cst: bool,
    }
}

impl BoolExpr {
    pub fn get_span(&self) -> &Span {
        match self {
            BoolExpr::Unary { span, .. }
            | BoolExpr::Binary { span, .. }
            | BoolExpr::Compare { span, .. }
            | BoolExpr::Const { span, .. }
                => span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TNode {
    Block {
        decl: Vec<Symbol>,
        stmt: Vec<TNode>,
    },
    Assign {
        lhs: Symbol,
        rhs: IntExpr,
    },
    If {
        cond: BoolExpr,
        then: Vec<TNode>,
        otherwise: Option<Vec<TNode>>,
    },
    While {
        cond: BoolExpr,
        body: Box<TNode>,
    },
    Halt,
    Assert {
        cond: BoolExpr,
    },
    Print {
        vars: Vec<Symbol>,
    }
}

impl TNode {
    pub fn get_span(&self) -> Option<&Span> {
        match self {
            TNode::Assign { lhs: _, rhs }
                => Some(rhs.get_span()),
            TNode::If { cond, then: _, otherwise: _ }
                => Some(cond.get_span()),
            TNode::While { cond, body: _ }
                => Some(cond.get_span()),
            TNode::Assert { cond }
                => Some(cond.get_span()),
            _ => None
        }
    }
}

type Program = Vec<TNode>;