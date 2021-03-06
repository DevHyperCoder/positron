/*
 * positron - parse and execute boolean expressions
 * Copyright (C) 2021 DevHyperCoder
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::collections::HashMap;

use crate::error::{Error, Result};

/// Gate enum
/// Each Gate type (And, Or and Not) take in a Vec of other gates.
/// Value Gate is a simple passthrough gate
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Gate {
    /// Does boolean AND operation
    And(Vec<Gate>),
    /// Does boolean OR operation
    Or(Vec<Gate>),
    /// Does boolean NOT operation on the first elem
    Not(Vec<Gate>),
    /// Evaluates to the content of the variable specified by inner string
    Value(String),
}

impl Gate {
    /// Executes the Gate, recursively if required.
    ///
    /// Error: If variable not found, only happens in Value gate.
    pub fn execute(self, data: &HashMap<String, bool>) -> Result<bool> {
        match self {
            Gate::Value(s) => {
                let var_val = data.get(s.as_str());
                if var_val.is_none() {
                    return Err(Error::VariableNotFound(s));
                }
                Ok(var_val.unwrap().to_owned()) // This is safe
            }
            Gate::Not(s) => {
                let val = s[0].clone().execute(data)?;
                Ok(!val)
            }
            Gate::Or(s) => {
                let mut acc = false;
                for val in s {
                    let value = val.execute(data)?;
                    acc |= value;
                }
                Ok(acc)
            }
            Gate::And(s) => {
                let mut acc = true;
                for gate in s {
                    let value = gate.execute(data)?;
                    acc &= value;
                }
                Ok(acc)
            }
        }
    }
}
