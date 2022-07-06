// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

//! A Leo program consists of import, circuit, and function definitions.
//! Each defined type consists of ast statements and expressions.

use crate::{Circuit, Function, FunctionInput, Identifier};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stores the Leo program abstract syntax tree.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Program {
    /// The name of the program.
    /// Empty after parsing.
    pub name: String,
    /// Expected main function inputs.
    /// Empty after parsing.
    pub expected_input: Vec<FunctionInput>,
    /// A map from function names to function definitions.
    pub functions: IndexMap<Identifier, Function>,
    /// A map from circuit names to circuit definitions.
    pub circuits: IndexMap<Identifier, Circuit>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (_, function) in self.functions.iter() {
            function.fmt(f)?;
            writeln!(f,)?;
        }
        for (_, circuit) in self.circuits.iter() {
            circuit.fmt(f)?;
            writeln!(f,)?;
        }
        write!(f, "")
    }
}

impl Program {
    /// Constructs an empty program with `name`.
    pub fn new(name: String) -> Self {
        Self {
            name,
            expected_input: vec![],
            functions: IndexMap::new(),
            circuits: IndexMap::new(),
        }
    }

    /// Extract the name of the program.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of the program.
    pub fn set_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}
