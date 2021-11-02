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

use std::str::FromStr;

use positron::{gate::Gate, parser::Parsed};

#[test]
fn parse() {
    let mut test_str = "a.b";

    let parsed = Parsed::from_str(test_str).unwrap();

    assert_eq!(
        parsed.root_gate,
        Gate::And(vec![
            Gate::Value("a".to_string()),
            Gate::Value("b".to_string())
        ])
    );

    assert!(parsed.variables.contains("a"));
    assert!(parsed.variables.contains("b"));

    // Testing OR Gate

    test_str = "a+b";
    let parsed = Parsed::from_str(test_str).unwrap();

    assert_eq!(
        parsed.root_gate,
        Gate::Or(vec![
            Gate::Value("a".to_string()),
            Gate::Value("b".to_string())
        ])
    );

    assert!(parsed.variables.contains("a"));
    assert!(parsed.variables.contains("b"));

    // Combination

    test_str = "(a+b)'.(c+d)";
    let parsed = Parsed::from_str(test_str).unwrap();

    assert_eq!(
        parsed.root_gate,
        Gate::And(vec![
            Gate::Not(vec![Gate::Or(vec![
                Gate::Value("a".to_string()),
                Gate::Value("b".to_string())
            ])]),
            Gate::Or(vec![
                Gate::Value("c".to_string()),
                Gate::Value("d".to_string())
            ])
        ])
    );

    assert!(parsed.variables.contains("a"));
    assert!(parsed.variables.contains("b"));
    assert!(parsed.variables.contains("c"));
    assert!(parsed.variables.contains("d"));
}
