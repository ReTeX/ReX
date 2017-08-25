#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate log;

extern crate stix2 as stix;
extern crate font_types;

#[macro_use]
extern crate static_map;
#[macro_use]
extern crate static_map_macros;

#[macro_use]
mod macros;

pub mod environments;
pub mod error;
pub mod dimensions;
pub mod layout;
pub mod lexer;
pub mod parser;
pub mod render;

mod font;
mod functions;

pub use render::*;
pub use font_types::fp;

