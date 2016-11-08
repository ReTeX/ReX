#![feature(plugin)]
#![plugin(phf_macros)]
extern crate phf;
//extern crate svg;

#[macro_use]
mod macros; 

mod lexer;
pub mod parser;
mod functions;
pub mod render;
mod font;
mod spacing;
