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

use positron::{circuit::Circuit, parser::Parsed, truth::truth};
use std::{collections::HashMap, io, str::FromStr};

fn main() {
    println!("positron");
    println!("Enter expression: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parsed = Parsed::from_str(input.trim()).unwrap();

    println!("Should show truth or will you enter stuff ? true / false");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let should_show_truth: bool = input.trim().parse().unwrap();
    input.clear();

    if should_show_truth {
        // calculating truth table
        let truth_entires = truth(parsed);
        //println!("{:#?}",a);

        for truth_entry in truth_entires {
            println!("--");
            for (_key, val) in truth_entry.data {
                print!(" {} ", val);
            }
            print!(" {} ", truth_entry.result);
            println!("--");
        }

        return;
    }
    let mut data = HashMap::new();

    for var in parsed.variables {
        println!("Enter value for \"{}\" in true / false: ", var);
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        data.insert(var, input.trim().parse().unwrap());
    }

    let circuit = Circuit {
        gate: parsed.root_gate,
        data,
    };

    let res = circuit.execute().unwrap();

    println!("Result: {}", res);
}
