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

use crate::{error::Result, gate::Gate};

/// Variable name to variable value mapping
pub type CircuitData = HashMap<String, bool>;

/// Main "Circuit". Contains the data and the root gate.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Circuit {
    /// Root gate
    pub gate: Gate,
    /// Data to execute the root gate
    pub data: CircuitData,
}

impl<'a> Circuit {
    /// Execute the root gate with CircuitData
    pub fn execute(self) -> Result<bool> {
        self.gate.execute(&self.data)
    }
}
