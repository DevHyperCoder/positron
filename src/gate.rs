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

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Gate {
    And(Vec<Gate>),
    Or(Vec<Gate>),
    Not(Vec<Gate>),
    Value(String),
}

impl Gate {
    pub fn execute(self, data: &HashMap<String, bool>) -> bool {
        match self {
            Gate::Value(s) => return data.get(s.as_str()).unwrap().to_owned(),
            Gate::Not(s) => {
                let values = s
                    .into_iter()
                    .map(|gate| gate.execute(data))
                    .collect::<Vec<bool>>();
                !values[0]
            }
            Gate::Or(s) => {
                let values = s
                    .into_iter()
                    .map(|gate| gate.execute(data))
                    .collect::<Vec<bool>>();

                let mut acc = false;
                for value in values {
                    acc |= value;
                }
                acc
            }
            Gate::And(s) => {
                let values = s
                    .into_iter()
                    .map(|gate| gate.execute(data))
                    .collect::<Vec<bool>>();
                let mut acc = true;
                for value in values {
                    acc &= value;
                }
                acc
            }
        }
    }
}
