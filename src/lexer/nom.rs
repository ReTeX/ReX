use regex::bytes::Regex;

use dimensions::Unit;
use nom;
use nom::space;
use nom::alpha;
use nom::digit;
use nom::IResult;
use nom::Needed;
use nom::ErrorKind;
use nom::AsChar;
use std::f64;
use std::num::ParseFloatError;
use std::str::FromStr;
use std::str;

fn single_non_alpha(i :&[u8]) -> IResult<&[u8], &[u8]> {
  if i.len() == 0 {
    IResult::Incomplete(Needed::Size(1))
  } else {
    if ! i[0].is_alpha() {
        IResult::Done(&i[1..], &i[0..1])
    } else {
        IResult::Error(ErrorKind::Alt)
    }
  }
}

fn floating_point(i: &[u8]) -> IResult<&[u8], f64> {
    lazy_static! {
    static ref FLOAT_RE: Regex =
        Regex::new(r#"^[-]?[0-9]+\.?[0-9]*(?:[eE][-+]?[0-9]+)?"#)
            .expect("Unable to compile regular expression for\
                    floating point detection"); };

    if let Some((0, idx)) = FLOAT_RE.find(i) {
        let f = f64::from_str(
            unsafe { str::from_utf8_unchecked(&i[0..idx]) })
            .expect("Failed to convert float!? WTF?");
        return IResult::Done(&i[idx..], f);
    } else {
        return IResult::Error(ErrorKind::Digit);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Command(&'a [u8]),
    Symbol(char),
    WhiteSpace,
    EOF,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LexState {
    Math,       // Math mode
    Text,       // Text mode
}

#[derive(Debug)]
pub struct Lexer<'a, 'b> {
    input:       &'a [u8],
    cursor:      &'b [u8],
    pub state:   LexState,
    pub current: Token<'a>,
}

named!(command, do_parse!(
    tag!(r"\") >>
    cmd: alt!(single_non_alpha | alpha) >>
    opt!(space) >>

    (cmd)
));

named!(dimension <Unit>, do_parse!(
    opt!(space) >>
    number: floating_point >>
    opt!(space) >>

    (Unit::Font(number))
));

#[test]
fn nom_lex() {
    println!("{:?}", command(b"abc1"));
    println!("{:?}", command(b"\\abc1"));
    println!("{:?}", command(b"\\abc   1"));
    println!("{:?}", command(b"\\abc"));
    println!("{:?}", command(b"\\12"));
    println!("{:?}", command(b"\\1234"));

    println!("{:?}", dimension(b"-1"));
    println!("{:?}", dimension(b"-123a"));
    println!("{:?}", dimension(b"+321b"));
    println!("{:?}", dimension(b"-1.23e+10"));
    println!("{:?}", dimension(b"-1.23e-10"));
    println!("{:?}", dimension(b"-1.23e"));

    // TODO: We probably need better error handling for
    // floats?  -1.23e should probably fail?
}