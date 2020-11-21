
#![recursion_limit = "512"]

#[macro_use]
extern crate combine;
extern crate itertools;
#[macro_use]
extern crate quote;
extern crate rand;
extern crate djed_rsx_shared;
extern crate djed_self_tokenize_macro;
extern crate djed_self_tokenize_trait;

pub mod parse_attributes_types;
mod parse_attributes;
pub mod parse_children_types;
mod parse_children;
pub mod parse_elements_types;
mod parse_elements;
pub mod parse_external_placeholders;
pub mod parse_external_types;
mod parse_external;
pub mod parse_js_types;
mod parse_js;
mod parse_misc;
mod parse_rsx;
pub mod parse_rust_types;
mod parse_rust;
mod tokenize_attributes;
mod tokenize_children;
mod tokenize_elements;
mod tokenize_external;

#[cfg(test)]
mod test_helpers;

use combine::{ParseError, Parser};
use combine::combinator::parser;

pub fn parse(s: &str) -> Result<(parse_elements_types::RSXElement, &str), ParseError<&str>> {
    parser(parse_rsx::rsx_element_ignoring_ws).parse(s)
}
