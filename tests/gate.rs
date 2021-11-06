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

use positron::{circuit::Circuit, gate::Gate};
use std::collections::HashMap;

#[test]
fn and() {
    let gate = Gate::And(vec![
        Gate::Value("a".to_string()),
        Gate::Value("b".to_string()),
    ]);
    let mut data = HashMap::new();
    data.insert("a".to_string(), false);
    data.insert("b".to_string(), true);

    let circuit = Circuit { gate, data };

    assert_eq!(circuit.execute().unwrap(), false);
}
#[test]
fn or() {
    let gate = Gate::Or(vec![
        Gate::Value("a".to_string()),
        Gate::Value("b".to_string()),
    ]);
    let mut data = HashMap::new();
    data.insert("a".to_string(), false);
    data.insert("b".to_string(), true);

    let circuit = Circuit { gate, data };

    assert_eq!(circuit.execute().unwrap(), true);
}
#[test]
fn not() {
    let gate = Gate::Not(vec![Gate::Value("a".to_string())]);
    let mut data = HashMap::new();
    data.insert("a".to_string(), false);

    let circuit = Circuit { gate, data };

    assert_eq!(circuit.execute().unwrap(), true);
}
