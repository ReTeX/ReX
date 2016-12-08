#![feature(try_from)]
#![feature(plugin)]
#![plugin(phf_macros)]
#![plugin(quickcheck_macros)]

#[cfg(test)]
extern crate quickcheck;

#[macro_use]
extern crate nom;
extern crate regex;
extern crate phf;

#[macro_use]
mod macros;

#[macro_use]
extern crate lazy_static;

pub mod dimensions;
pub mod layout;
pub mod lexer;
pub mod parser;
pub mod render;

mod font;
mod functions;
