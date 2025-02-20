/*
 * author : Narcisse.
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
use std::fmt;
use std::collections::HashMap;

/// Public structure containing the hashtable for all types of
/// the program's symbols.
#[derive(Debug, Clone, Default)]
pub struct TypeTable {
    table: HashMap<String, Type>,
}

impl TypeTable {
    pub fn get(&self, tname: &str) -> Option<&Type> {
        self.table.get(tname)
    }
}

/// Public enumeration used to store information about types.
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

/// Builder for user made custom types.
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