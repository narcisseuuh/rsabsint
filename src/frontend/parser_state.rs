/*
 * author : Narcisse.
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
use crate::symbol::{SymbolTable, Symbol};
use crate::typing::TypeTable;

pub struct ParserState {
    pub sym_table: SymbolTable, // table of symbols
    pub type_table: TypeTable, // table for userdef types (upcoming)
}

impl Default for ParserState {
    fn default() -> Self {
        ParserState {
            sym_table: SymbolTable::default(),
            type_table: TypeTable::default(),
        }
    }
}

impl ParserState {
    pub fn get_var(&self, name: &str) -> Result<Symbol, String> {
        self.sym_table
            .get(name)
            .cloned()
            .ok_or("Variable was not declared".to_owned())
    }

    pub fn update_state(&mut self) -> Result<(), String> {
        self.sym_table = SymbolTable::default();
        Ok(())
    }
}