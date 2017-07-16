#![feature(test)]
extern crate test;

#[macro_use]
extern crate log;

extern crate fnv;
extern crate fp;

#[macro_use]
extern crate staticmap;
#[macro_use]
extern crate staticmap_macros;

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