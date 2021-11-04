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

use crate::{
    circuit::{Circuit, CircuitData},
    parser::Parsed,
    product_repeat::product_repeat,
};
use std::collections::HashMap;

pub struct TruthEntry {
    pub data: CircuitData,
    pub result: bool,
}

/// Returns a vector of CircuitData and result
pub fn truth(p: Parsed) -> Vec<TruthEntry> {
    let repeat = p.variables.len();
    let bool_arr = vec![true, false];

    let mut data = HashMap::new();

    let variable_list = p.variables.into_iter().collect::<Vec<String>>();

    let mut results = vec![];

    for i in product_repeat(bool_arr.iter(), repeat) {
        for (idx, var) in variable_list.iter().enumerate() {
            data.insert(var.clone(), *i[idx]);
        }

        let circuit = Circuit {
            data: data.clone(),
            gate: p.root_gate.clone(),
        };

        results.push(TruthEntry {
            data: data.clone(),
            result: circuit.execute(),
        });
    }
    results
}
