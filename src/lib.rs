#![feature(str_escape)]
#![feature(plugin)]
#![plugin(phf_macros)]
extern crate phf;

#[macro_use]
mod macros; 

mod lexer;
mod parser;
mod symbols;
mod functions;