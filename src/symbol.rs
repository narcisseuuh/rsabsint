use lrlex::DefaultLexerTypes;
use lrpar::{NonStreamingLexer, Span};

use crate::typing::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct LabelGenerator {
    counter: u16,
}
impl LabelGenerator {
    pub fn default() -> Self {
        Self { counter: 1 }
    }
    pub fn get(&mut self) -> u16 {
        let label = self.counter;
        self.counter += 1;
        label
    }
}

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

// This stored data about each symbol
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