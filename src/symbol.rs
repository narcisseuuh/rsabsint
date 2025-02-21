/*
 * author : Narcisse.
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
use lrlex::DefaultLexerTypes;
use lrpar::{NonStreamingLexer, Span};

use crate::typing::*;
use std::collections::HashMap;

/// Symbol table, containing a hashmap mapping String names
/// to Symbols.
#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    table: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.table.get(name)
    }

    pub fn insert_builder(
        &mut self,
        s: SymbolBuilder,
        lexer: &dyn NonStreamingLexer<DefaultLexerTypes>,
    ) -> Result<(), String> {
        let name = lexer.span_str(s.get_name());
        if self.table.contains_key(name) {
            return Err(format!("Variable declared multiple times"));
        }
        let symb = s.build(lexer).unwrap();
        self.table.insert(name.to_string(), symb.clone());
        Ok(())
    }

    pub fn insert_symbol(&mut self, s: Symbol, check: bool) -> Result<(), String> {
        if check && self.table.contains_key(s.get_name()) {
            return Err(format!("Multiple variables with same name defined"));
        }
        self.table.insert(s.get_name().to_string(), s);
        Ok(())
    }
}

/// Enumeration storing data about all the symbols,
/// currently, only variables are supported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    Variable {
        name: String,
        dtype: Type,
    },
}

impl Symbol {
    pub fn get_name(&self) -> &str {
        match self {
            Self::Variable { name, .. } => name,
        }
    }

    pub fn get_type(&self) -> &Type {
        match self {
            Self::Variable { dtype, .. } => dtype,
        }
    }
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Symbol {
    // we compare symbols per lexical order
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Symbol::Variable { name : n1, dtype : _ } => {
                match other {
                    Symbol::Variable { name : n2, dtype : _ }
                        => n1.cmp(n2)
                }
            },
        }
    }
    
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }
    
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }
    
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

/// Structure to build Symbols.
#[derive(Clone)]
pub struct SymbolBuilder {
    name: Span,
    dtype: TypeBuilder,
}

impl SymbolBuilder {
    pub fn new(name: Span) -> SymbolBuilder {
        SymbolBuilder {
            name,
            dtype: TypeBuilder::default(),
        }
    }

    pub fn get_name(&self) -> Span {
        self.name
    }

    pub fn dtype(&mut self, inner_type: Type) -> &mut SymbolBuilder {
        self.dtype.dtype(inner_type);
        self
    }

    pub fn build(
        self,
        lexer: &dyn NonStreamingLexer<DefaultLexerTypes>,
    ) -> Result<Symbol, String> {
        Ok(Symbol::Variable {
            name: lexer.span_str(self.name).to_string(),
            dtype: self.dtype.build()?,
        })
    }
}