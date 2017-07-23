use lexer::OwnedToken;
use std::error;
use std::fmt;
use font::AtomType;
use font::Symbol;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    UnrecognizedCommand(String),
    UnrecognizedSymbol(char),
    FailedToParse(OwnedToken),
    ExcessiveSubscripts,
    ExcessiveSuperscripts,
    LimitsMustFollowOperator,
    ExpectedMathField(OwnedToken),
    MissingSymbolAfterDelimiter,
    MissingSymbolAfterAccent,
    ExpectedAtomType(AtomType, AtomType),
    ExpectedSymbol(OwnedToken),
    RequiredMacroArg,
    ExpectedTokenFound(OwnedToken, OwnedToken),
    ExpectedOpen(Symbol),
    ExpectedClose(Symbol),
    ExpectedOpenGroup,
    NoClosingBracket,
    StackMustFollowGroup,
    AccentMissingArg(String),
    UnexpectedEof,
    UnrecognizedDimension,
}

impl error::Error for Error {
    fn description(&self) -> &'static str {
        use self::Error::*;
        match *self {
            UnrecognizedCommand(_) => "unrecognized tex command",
            UnrecognizedSymbol(_) => "unrecognized symbol",
            FailedToParse(_) => "failed to parse",
            ExcessiveSubscripts => "excessive number of subscripts",
            ExcessiveSuperscripts => "excessive number of superscripts",
            LimitsMustFollowOperator => "limit commands must follow an operator",
            ExpectedMathField(_) => "expected a mathfield",
            MissingSymbolAfterDelimiter => "missing symbol following delimiter",
            MissingSymbolAfterAccent => "missing symbol following accent",
            ExpectedAtomType(_, _) => "expected atom type",
            ExpectedSymbol(_) => "expected symbol",
            RequiredMacroArg => "required macro argument",
            ExpectedTokenFound(_, _) => "unexpected token",
            ExpectedOpen(_) => "expected Open, Fence, or period after `\\left`",
            ExpectedClose(_) => "expected Open, Fence, or period after `\\right`",
            ExpectedOpenGroup => "expected an open group symbol",
            NoClosingBracket => "failed to find a closing bracket",
            StackMustFollowGroup => "stack commands must follow a group",
            AccentMissingArg(_) => "an argument must follow accent commands",
            UnexpectedEof => "unexpected end of parsing",
            UnrecognizedDimension => "failed to parse dimension",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            UnrecognizedCommand(ref cmd) =>
                write!(f, "unrecognized command: \\{}`", cmd),
            UnrecognizedSymbol(c) =>
                write!(f, "unrecognized symbol '{}'", c),
            FailedToParse(ref tok) =>
                write!(f, "failed to parse `{}`", tok),
            ExcessiveSubscripts =>
                write!(f, "an excessive number of subscripts"),
            ExcessiveSuperscripts =>
                write!(f, "excessive number of superscripts"),
            LimitsMustFollowOperator =>
                write!(f, "limit commands must follow an operator"),
            ExpectedMathField(ref field) =>
                write!(f, "expected math field, found `{}`", field),
            MissingSymbolAfterDelimiter =>
                write!(f, "missing symbol following delimiter"),
            MissingSymbolAfterAccent =>
                write!(f, "missing symbol following accent"),
            ExpectedAtomType(left, right) =>
                write!(f, "expected atom type {:?} found {:?}", left, right),
            ExpectedSymbol(ref sym) =>
                write!(f, "expected symbol, found {}", sym),
            RequiredMacroArg =>
                write!(f, "missing required macro argument"),
            ExpectedTokenFound(ref expected, ref found) =>
                write!(f, "expected {} found {}", expected, found),
            ExpectedOpen(sym) =>
                write!(f, "expected Open, Fence, or period after '\\left', found `{:?}`", sym),
            ExpectedClose(sym) =>
                write!(f, "expected Open, Fence, or period after '\\right', found `{:?}`", sym),
            ExpectedOpenGroup =>
                write!(f, "expected an open group symbol"),
            NoClosingBracket =>
                write!(f, "failed to find a closing bracket"),
            StackMustFollowGroup =>
                write!(f, "stack commands must follow a group"),
            AccentMissingArg(ref acc) =>
                write!(f, "the accent '\\{}' must have an argument", acc),
            UnexpectedEof =>
                write!(f, "unexpected EOF"),
            UnrecognizedDimension =>
                write!(f, "failed to parse dimension"),
        }
    }
}