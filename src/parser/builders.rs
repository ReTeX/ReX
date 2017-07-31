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
