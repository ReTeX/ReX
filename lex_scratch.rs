#[macro_use]
extern crate nom;
use nom::alpha;
use nom::IResult;
use nom::Needed;
use nom::ErrorKind;
use nom::AsChar;

fn single_non_alpha(i:&[u8]) -> IResult<&[u8], &[u8]>{
  if i.len() < 1 {
    IResult::Incomplete(Needed::Size(1))
  } else {
    if ! i[0].is_alpha() {
        IResult::Done(&i[1..], &i[0..1])
    } else {
        IResult::Error(ErrorKind::Alt)
    }
  }
}

named!(command,
    do_parse!(
        tag!(r"\") >>
        cmd: alt!(
            single_non_alpha
            | alpha
        ) >>
        take_while!(nom::is_space) >>
        (cmd)
    )
);

fn main() {
    println!("{:?}", command(b"abc1"));
    println!("{:?}", command(b"\\abc1"));
    println!("{:?}", command(b"\\abc   1"));
    println!("{:?}", command(b"\\abc"));
    println!("{:?}", command(b"\\12"));
    println!("{:?}", command(b"\\1234"));
}