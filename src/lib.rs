#![feature(plugin)]
#![plugin(phf_macros)]
extern crate phf;
//extern crate svg;

#[macro_use]
mod macros; 

#[macro_use]
extern crate lazy_static;

mod lexer;
pub mod parser;
mod functions;
pub mod render;
mod font;
mod spacing;
