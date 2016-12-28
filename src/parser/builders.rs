use super::nodes::Scripts;
use super::ParseNode;
use super::AtomType;
use font::Symbol;

// Convenience macro: Opt<T> -> Option<Box<T>>
macro_rules! opt_box {
    ($expr:expr) => ({
        match $expr {
            None    => None,
            Some(e) => Some(Box::new(e)),
        }
    })
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