#![feature(test)]
extern crate test;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

#[macro_use]
extern crate log;

extern crate fp;
extern crate stix;
extern crate font_types;

#[macro_use]
extern crate static_map;
#[macro_use]
extern crate static_map_macros;

#[macro_use]
mod macros;

pub mod error;
pub mod dimensions;
pub mod layout;
pub mod lexer;
pub mod parser;
pub mod render;

mod font;
mod functions;

pub use render::*;