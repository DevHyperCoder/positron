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

use std::{collections::HashSet, str::FromStr};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{
    error::{Error, Result},
    gate::Gate,
};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ExprParser;

#[derive(Clone)]
pub struct Parsed {
    pub root_gate: Gate,
    pub variables: HashSet<String>,
}

impl FromStr for Parsed {
    type Err = Error;
    fn from_str(s: &str) -> Result<Parsed> {
        let p = match ExprParser::parse(Rule::main, s) {
            Ok(mut pair) => {
                pair.next().unwrap() // This call is safe.
            }
            Err(e) => return Err(Error::UnableToParse(e.to_string())),
        };

        let mut variables = HashSet::new();
        let root_gate = get_gate_tree_with_variables(p, &mut variables);

        Ok(Parsed {
            root_gate,
            variables,
        })
    }
}

fn get_gate_tree_with_variables(expr: Pair<Rule>, values: &mut HashSet<String>) -> Gate {
    match expr.as_rule() {
        Rule::var_name => {
            let val: String = expr.as_span().as_str().into();
            values.insert(val.clone());
            Gate::Value(val)
        }
        Rule::not_gate => Gate::Not(vec![get_gate_tree_with_variables(
            expr.into_inner().next().unwrap(),
            values,
        )]), // This unwrap is safe.
        Rule::and_gate => {
            let mut params = vec![];
            for p in expr.into_inner() {
                params.push(get_gate_tree_with_variables(p, values))
            }
            Gate::And(params)
        }
        Rule::or_gate => {
            let mut params = vec![];
            for p in expr.into_inner() {
                params.push(get_gate_tree_with_variables(p, values))
            }
            Gate::Or(params)
        }
        _ => unreachable!(),
    }
}
