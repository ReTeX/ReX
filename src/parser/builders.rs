use super::nodes::Scripts;
use super::ParseNode;

// Convenience macro: Opt<T> -> Option<Box<T>>
macro_rules! opt_box {
    ($expr:expr) => ({
        match $expr {
            None    => None,
            Some(e) => Some(Box::new(e)),
        }
    })
}

macro_rules! delimited {
    ($left:expr, $right:expr, $inner:expr) => (
        ParseNode::Delimited(
            Delimited {
                left: $left,
                right: $right,
                inner: $inner
            }
        )
    )
}

macro_rules! accent {
    ($sym:expr, $nucleus:expr) => (
        ParseNode::Accent(
            Accent {
                symbol: $sym,
                nucleus: $nucleus
            }
        )
    )
}

macro_rules! symbol {
    ($codepoint:expr, $atom:expr) => (
        ParseNode::Symbol(
            Symbol {
                unicode: $codepoint,
                atom_type: $atom,
            }
        )
    )
}

pub fn scripts(base: Option<ParseNode>,
               sup:  Option<ParseNode>,
               sub:  Option<ParseNode>) -> ParseNode
{
    ParseNode::Scripts(Scripts {
            base: opt_box!(base),
            superscript: opt_box!(sup),
            subscript: opt_box!(sub),
        })
}