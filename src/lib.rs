#![feature(try_from)]
#![feature(plugin)]
#![plugin(phf_macros)]

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

pub use render::SVGRenderer;