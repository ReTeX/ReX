#![feature(plugin)]
#![plugin(phf_macros)]

#![feature(test)]
extern crate test;

#[macro_use]
extern crate log;

extern crate phf;
extern crate fnv;
extern crate fp;

#[macro_use]
mod macros;

#[macro_use]
extern crate lazy_static;

pub mod error;
pub mod dimensions;
pub mod layout;
pub mod lexer;
pub mod parser;
pub mod render;

mod font;
mod functions;

pub use render::*;