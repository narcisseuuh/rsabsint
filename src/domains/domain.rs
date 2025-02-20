/*
 * author : Narcisse.
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
/// General trait for an abstract domain.
/// You need to precise top, bottom, and basic operators
/// in order for the interpreter to manipulate it.
use crate::ast::*;
use crate::interpreter::interpreter::AnalysisError;
use crate::symbol::*;

pub trait AbstractDomain : Clone + Eq {
    /// empty set
    fn bottom() -> Self;

    /// wether an element represents the empty set
    fn is_bottom(&self) -> bool;
    /// wether an abstract element is included in another
    fn subset(&self, rhs : &Self) -> bool;

    /// abstract join
    fn join(mut self, rhs : Self) -> Self {
        self.join_with(rhs);
        self
    }

    /// abstract intersection
    fn meet(mut self, rhs : Self) -> Self {
        self.meet_with(rhs);
        self
    }

    /// widening : loose precision to ensure soundness
    fn widen(mut self, rhs : Self) -> Self {
        self.widen_with(rhs);
        self
    }

    /// narrowing : gain precision without loosing soundness
    fn narrow(mut self, rhs : Self) -> Self {
        self.narrow_with(rhs);
        self
    }

    /// helper function for the join operator
    fn join_with(&mut self, rhs : Self);
    /// helper function for the meet operator
    fn meet_with(&mut self, rhs : Self);
    /// helper function for the widening operator
    fn widen_with(&mut self, rhs : Self);
    /// helper function for the narrowing operator
    fn narrow_with(&mut self, rhs : Self);

    /// helper function to compare expressions inside a domain
    fn compare(&mut self, e1 : IntExpr, cmp : CompareOp, e2 : IntExpr) -> Self;
    /// helper function to represent the assignment
    fn assign(&mut self, v : Symbol, e : IntExpr) -> Result<Self, AnalysisError>;
    /// helper function to add variable from scope
    fn add_variable(&mut self, v : Symbol) -> Self;
    /// helper function to remove variable from scope
    fn remove_variable(&mut self, v : Symbol) -> Self;

    /// pretty printer
    fn print(&mut self, symbol : Symbol) -> String;
}