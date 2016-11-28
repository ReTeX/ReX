#![feature(try_from)]
#![feature(plugin)]
#![plugin(phf_macros)]
extern crate phf;
//extern crate svg;

#[macro_use]
mod macros;

#[macro_use]
extern crate lazy_static;

pub mod lexer;
pub mod parser;
mod functions;
pub mod render;
mod font;
pub mod layout;
pub mod dimensions;
