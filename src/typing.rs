use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct TypeTable {
    table: HashMap<String, Type>,
}

impl TypeTable {
    pub fn get(&self, tname: &str) -> Option<&Type> {
        self.table.get(tname)
    }
}

// This ds is used to store information about types
#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Bool => write!(f, "Bool"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct TypeBuilder {
    dtype: Option<Type>,
}

impl TypeBuilder {
    pub fn dtype(&mut self, inner_type: Type) -> &mut Self {
        self.dtype = Some(inner_type);
        self
    }

    pub fn build(self) -> Result<Type, String> {
        match self.dtype {
            Some(t) => {
                Ok(t)
            }
            None => Err("Inner type was not set!".to_owned()),
        }
    }
}