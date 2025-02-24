/*
 * author : Narcisse.
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
/* 
use crate::ast::*;
use crate::symbol::*;
use crate::interpreter::interpreter::*;

use super::domain::AbstractDomain;

#[derive(Clone, PartialEq, Eq)]
pub enum ConcreteDomain {
    Caca,
}

impl AbstractDomain for ConcreteDomain {
    fn bottom() -> Self {
        todo!()
    }

    fn is_bottom(&self) -> bool {
        todo!()
    }

    fn subset(&self, rhs : &Self) -> bool {
        todo!()
    }

    fn join_with(&mut self, rhs : Self) {
        todo!()
    }

    fn meet_with(&mut self, rhs : Self) {
        todo!()
    }

    fn widen_with(&mut self, rhs : Self) {
        todo!()
    }

    fn narrow_with(&mut self, rhs : Self) {
        todo!()
    }

    fn compare(&mut self, e1 : &IntExpr, cmp : &CompareOp, e2 : &IntExpr) -> Self {
        todo!()
    }

    fn assign(&mut self, v : &Symbol, e : &IntExpr) -> Result<Self, AnalysisError> {
        todo!()
    }

    fn add_variable(&mut self, v : &Symbol) -> Self {
        todo!()
    }

    fn remove_variable(&mut self, v : &Symbol) -> Self {
        todo!()
    }

    fn print(&mut self, symbol : Symbol) -> String {
        todo!()
    }
}
*/