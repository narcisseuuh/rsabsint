/*
 * author : Narcisse.
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
use lrpar::Span;
use crate::symbol::*;

/// binary operands for the type int.
#[derive(Debug, Clone, Copy)]
pub enum IntBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

/// unary operands for the type int.
#[derive(Debug, Clone, Copy)]
pub enum IntUnaryOp {
    AddUnary,
    SubUnary,
}

/// comparison operands.
#[derive(Debug, Clone, Copy)]
pub enum CompareOp {
    NE,
    GT,
    GE,
    LT,
    LE,
    EQ,
}

/// binary operands for the type bool.
#[derive(Debug, Clone, Copy)]
pub enum BoolBinaryOp {
    And,
    Or,
}

/// unary operands for the type bool.
#[derive(Debug, Clone, Copy)]
pub enum BoolUnaryOp {
    Not,
}

/// nodes inside an integer expression.
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
        lower: Box<IntExpr>,
        upper: Box<IntExpr>,
    }
}

impl IntExpr {
    /// getter for the span of an integer expression.
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

/// nodes inside a boolean expression.
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
    /// getter for the span of a boolean expression.
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

/// enumeration type for all the nodes of the
/// abstract syntax tree for the subset of C language
/// studied.
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
        then: Box<TNode>,
        otherwise: Option<Box<TNode>>,
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
    /// getter for the span of an AST node.
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

/// type for Programs analyzed by rsabsint.
pub type Program = Vec<TNode>;

fn display_symbol(symbol : Symbol, indent: usize) {
    let indentation = " ".repeat(indent);
    match symbol {
        Symbol::Variable { name, dtype }
            => println!("{}{:?} {};", indentation, dtype, name),
    }
}

fn format_intexpr(expr: &IntExpr) -> String {
    match expr {
        IntExpr::Unary { op, exp, .. }
            => format!("{:?}({})", op, format_intexpr(exp)),
        IntExpr::Binary { op, lhs, rhs, .. }
            => format!("({} {:?} {})", format_intexpr(lhs), op, format_intexpr(rhs)),
        IntExpr::Ident { var, .. }
            => format!("{}", var.get_name()),
        IntExpr::Const { cst, .. }
            => format!("{}", cst),
        IntExpr::Rand { lower, upper, .. }
            => format!("rand({}, {})", format_intexpr(lower), format_intexpr(upper)),
    }
}

fn format_boolexpr(expr: &BoolExpr) -> String {
    match expr {
        BoolExpr::Unary { op, exp, .. }
            => format!("{:?}({})", op, format_boolexpr(exp)),
        BoolExpr::Binary { op, lhs, rhs, .. }
            => format!("({} {:?} {})", format_boolexpr(lhs), op, format_boolexpr(rhs)),
        BoolExpr::Compare { op, lhs, rhs, .. }
            => format!("({} {:?} {})", format_intexpr(lhs), op, format_intexpr(rhs)),
        BoolExpr::Const { cst, .. }
            => format!("{}", cst),
    }
}

/// Pretty-prints a statement.
pub fn display_tnode(node: &TNode, indent: usize) {
    let indentation = " ".repeat(indent);
    match node {
        TNode::Block { decl, stmt } => {
            println!("{}{{", indentation);
            for d in decl {
                display_symbol(d.clone(), indent + 4);
            }
            for s in stmt {
                display_tnode(s, indent + 4);
            }
            println!("{}}}", indentation);
        }
        TNode::Assign { lhs, rhs } => {
            println!("{}{} = {};", indentation, lhs.get_name(), format_intexpr(rhs));
        }
        TNode::If { cond, then, otherwise } => {
            println!("{}if ({})", indentation, format_boolexpr(cond));
            display_tnode(then, indent + 4);
            if let Some(else_node) = otherwise {
                println!("{}else", indentation);
                display_tnode(else_node, indent + 4);
            }
        }
        TNode::While { cond, body } => {
            println!("{}while ({})", indentation, format_boolexpr(cond));
            display_tnode(body, indent + 4);
        }
        TNode::Halt => {
            println!("{}halt;", indentation);
        }
        TNode::Assert { cond } => {
            println!("{}assert({});", indentation, format_boolexpr(cond));
        }
        TNode::Print { vars } => {
            let var_names = 
                vars
                .iter()
                .map(|var| var.get_name())
                .collect::<Vec<_>>()
                .join(", ");
            println!("{}print({});", indentation, var_names);
        }
    }
}

/// Pretty-prints the program taken as argument.
pub fn display_program(program : Program)
{
    for node in program {
        display_tnode(&node, 0);
    }
}