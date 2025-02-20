/*
 * author : Narcisse.
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
use crate::domains::domain::AbstractDomain;
use crate::ast::*;

#[derive(Debug)]
pub enum AnalysisError {
    DeadCode,
    FailedAssert,
}

impl std::fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::DeadCode => { write!(f, "DeadCode") },
            Self::FailedAssert => { write!(f, "FailedAssert") },
        }
    }
}

impl std::error::Error for AnalysisError {}

#[derive(Clone)]
pub struct AnalysisResults {
    msg : String,
    node : TNode,
}

impl AnalysisResults {
    pub fn new(msg : String, node : TNode) -> Self {
        AnalysisResults {
            msg,
            node,
        }
    }

    pub fn show(&mut self) {
        println!("{} in statement :", self.msg);
        display_tnode(&self.node, 0);
    }
}

fn fixpoint<D : AbstractDomain, T : FnMut(D, BoolExpr) -> Result<D, AnalysisError>>
(mut f : T, x : &D, cond : BoolExpr) -> Result<D, AnalysisError> {
    let fx = f(x.clone(), cond.clone())?;
    if fx.subset(&x) {
        Ok(fx)
    }
    else {
        // to change...xswdq
        fixpoint(f, &fx.join(x.clone()), cond)
    }
}

fn eval_boolexpr<D : AbstractDomain>(mut ctx : D, be : BoolExpr, should_satisfy : bool) -> D {
    match be {
        BoolExpr::Unary { span : _, op, exp } => {
            match op {
                BoolUnaryOp::Not => eval_boolexpr(ctx.clone(), *exp, !should_satisfy)
            }
        },
        BoolExpr::Binary { span : _, op, lhs, rhs } => {
            let eval_lhs = eval_boolexpr(ctx.clone(), *lhs, should_satisfy);
            let eval_rhs = eval_boolexpr(ctx.clone(), *rhs, should_satisfy);
            match op {
                BoolBinaryOp::And => eval_lhs.meet(eval_rhs),
                BoolBinaryOp::Or => eval_lhs.join(eval_rhs),
            }
        },
        BoolExpr::Compare { span : _, op, lhs, rhs } => {
            ctx.compare(lhs, op, rhs)
        },
        BoolExpr::Const { span : _, cst } => {
            if cst && should_satisfy {
                ctx
            }
            else {
                D::bottom()
            }
        },
    }
}

pub struct MonotonicFixpointIterator<D : AbstractDomain> {
    base : D,
    next_nodes : Vec<TNode>,
    unroll : u32,
}

impl<D> MonotonicFixpointIterator<D>
where D : AbstractDomain {
    pub fn new(next_nodes : Program, unroll : u32) -> Self {
        Self {
            base : D::bottom(),
            next_nodes,
            unroll,
        }
    }

    fn show_results(&mut self, msgs : Vec<AnalysisResults>) -> () {
        for msg in msgs {
            msg.clone().show();
        }
    }

    fn eval_stmt(&mut self, stmt : TNode, ctx : D) -> Result<D, AnalysisError> {
        // todo : take into account scope
        match stmt {
            TNode::Assert { cond } => {
                let res = eval_boolexpr(ctx.clone(), cond, true);
                if res.is_bottom() {
                    Err(AnalysisError::FailedAssert)
                }
                else {
                    Ok(res)
                }
            },
            TNode::Assign { lhs, rhs } => {
                Ok(self.base.assign(lhs, rhs))
            },  
            TNode::Block { decl, stmt } => {
                // todo : modify to take into account the scope
                let mut new_ctx = ctx;
                let _ = decl
                    .iter()
                    .map(|x| new_ctx.add_variable(x.clone()));
                self.eval_stmt_list(stmt, new_ctx)
            },
            TNode::Halt => {
                // todo : change this to raise an error if dead code after halt.
                Ok(ctx.clone())
            },
            TNode::If { cond, then, otherwise } => {
                // todo : modify to analyze the body of the then and else branches
                let then_domain = eval_boolexpr(ctx.clone(), cond.clone(), true);
                if let Some(otherwise) = otherwise {
                    let else_domain = eval_boolexpr(ctx.clone(), cond.clone(), false);
                    Ok(D::join(
                        self.eval_stmt(*then, then_domain)?,
                        self.eval_stmt(*otherwise, else_domain)?
                    ))
                }
                else {
                    Ok(then_domain)
                }

            },
            TNode::Print { vars } => {
                let fmt = vars
                    .iter()
                    .map(|x| -> String { 
                        format!("{} : {}", x.get_name(), self.base.print(x.clone()))
                    })
                    .fold(String::from(""),
                        |acc, x| -> String { format!("{}, {}", acc, x) });
                println!("{}", fmt);
                Ok(ctx.clone())
            },
            TNode::While { cond, body } => {
                // todo : implement loop unrolling and widening for fixpoint in while loop.
                let mut in_loop = eval_boolexpr(ctx.clone(),cond.clone(), true);
                for _ in 1..self.unroll {
                    in_loop = D::join(
                        in_loop.clone(),
                        self.eval_stmt(*body.clone(), in_loop)?
                    );
                }
                in_loop = fixpoint(
                    |x, cond| -> Result<D, AnalysisError> {
                        let filtered_comp = eval_boolexpr(x.clone(), cond, true);
                        let result = self.eval_stmt(*body.clone(), filtered_comp)?;
                        Ok(D::join(x, result))
                    },
                    &in_loop.clone(),
                    cond
                )?;
                Ok(in_loop)
            },
        }
    }

    fn eval_stmt_list(&mut self, stmt_list : Vec<TNode>, ctx : D) -> Result<D, AnalysisError> {
        self.base = ctx;
        for stmt in stmt_list {
            self.base = D::join(
                self.base.clone(),
                self.eval_stmt(stmt.clone(), self.base.clone())?
            );
        }
        Ok(self.base.clone())
    }

    pub fn eval_prog(&mut self) -> Result<(), AnalysisError> {
        let mut res : Vec<AnalysisResults> = Vec::new();
        for stmt in self.next_nodes.clone() {
            let curr_res = self.eval_stmt(stmt.clone(), D::bottom());
            if let Err(e) = curr_res {
                res.push(AnalysisResults::new(
                    e.to_string(),
                    stmt,
                ));
            }
            else {
                self.base = curr_res?;
            }
        }
        self.show_results(res);
        Ok(())
    }
}