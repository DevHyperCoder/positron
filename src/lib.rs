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
//! positron - parse and execute boolean expressions
//!
//! ## Examples
//! Basic Or Gate
//! ```rust
//! use std::collections::HashMap;
//! use positron::{circuit::Circuit,gate::Gate};
//!
//! // a + b
//! let gate =  Gate::Or(vec![Gate::Value("a".to_string()),Gate::Value("b".to_string())]);
//!
//! let mut data = HashMap::new();
//! data.insert("a".to_string(),true);
//! data.insert("b".to_string(),true);
//!
//! let circuit = Circuit {
//!     gate,
//!     data
//! };
//!
//! assert_eq!(circuit.execute().unwrap(),true)
//! ```
//!
//! Parsing example
//!
//! ```rust
//! use std::{collections::HashMap,str::FromStr};
//! use positron::{circuit::Circuit,parser::Parsed};
//!
//! let input = "(a+b).(a.b)";
//!
//! let parsed = Parsed::from_str(input).unwrap();
//!
//! assert!(parsed.variables.contains("a"));
//! assert!(parsed.variables.contains("b"));
//!
//! let mut data = HashMap::new();
//! data.insert("a".to_string(),true);
//! data.insert("b".to_string(),true);
//!
//! let circuit = Circuit {
//!     gate:parsed.root_gate,
//!     data
//! };
//!
//! assert_eq!(circuit.execute().unwrap(),true)

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[warn(missing_docs)]

/// Circuit Struct and impls
pub mod circuit;
/// Error handling
pub mod error;
/// Gate enum with impls
pub mod gate;
/// Expr parser
pub mod parser;
/// itertools.product with repeat
pub mod product_repeat;
/// Generate truth table entries
pub mod truth;
