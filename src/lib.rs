#![feature(plugin)]
#![plugin(phf_macros)]
extern crate phf;
 
//use phf;

mod lexer;
mod parser;
mod symbols;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
