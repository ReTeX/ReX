#![feature(plugin)]
#![plugin(phf_macros)]
extern crate phf;
//extern crate svg;

#[macro_use]
mod macros; 

mod lexer;
mod parser;
mod functions;
mod render;
mod font;