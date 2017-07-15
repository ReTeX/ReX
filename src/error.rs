use lexer::Token;
use std::error;
use std::fmt;
use parser::AtomType;
use font::Symbol;

pub type ParseResult<'a, T> = ::std::result::Result<T, ParseError<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError<'a> {
    UnrecognizedCommand(&'a str),
    FailedToParse(Token<'a>),
    ExcessiveSubscripts,
    ExcessiveSuperscripts,
    LimitsMustFollowOperator,
    ExpectedMathField(Token<'a>),
    MissingSymbolAfterDelimiter,
    MissingSymbolAfterAccent,
    ExpectedAtomType(AtomType, AtomType),
    ExpectedSymbol(Token<'a>),
    RequiredMacroArg,
    ExpectedTokenFound(Token<'a>, Token<'a>),
    ExpectedOpen(Symbol),
    ExpectedClose(Symbol),
    ExpectedOpenGroup,
    NoClosingBracket,
    StackMustFollowGroup,
}

impl<'a> error::Error for ParseError<'a> {
    fn description(&self) -> &'static str {
        use self::ParseError::*;
        match *self {
            UnrecognizedCommand(_) => "unrecogized tex command",
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
        }
    }
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;
        match *self {
            UnrecognizedCommand(cmd) =>
                write!(f, "Unreocngized command: `{}`", cmd),
            FailedToParse(tok) =>
                write!(f, "Failed to parse `{}`", tok),
            ExcessiveSubscripts =>
                write!(f, "An excessive number of subscripts"),
            ExcessiveSuperscripts =>
                write!(f, "excessive number of superscripts"),
            LimitsMustFollowOperator =>
                write!(f, "limit commands must follow an operator"),
            ExpectedMathField(field) =>
                write!(f, "expected math field, found `{}`", field),
            MissingSymbolAfterDelimiter =>
                write!(f, "missing symbol following delimiter"),
            MissingSymbolAfterAccent =>
                write!(f, "missing symbol following accent"),
            ExpectedAtomType(left, right) =>
                write!(f, "expected atom type `{:?}` found `{:?}`", left, right),
            ExpectedSymbol(sym) =>
                write!(f, "expected symbol, found `{:?}`", sym),
            RequiredMacroArg =>
                write!(f, "missing required macro argument"),
            ExpectedTokenFound(expected, found) =>
                write!(f, "expected `{:?}` found `{:?}`", expected, found),
            ExpectedOpen(sym) =>
                write!(f, "expected Open, Fence, or period after `\\left`, found `{:?}`", sym),
            ExpectedClose(sym) =>
                write!(f, "expected Open, Fence, or period after `\\right`, found `{:?}`", sym),
            ExpectedOpenGroup =>
                write!(f, "expected an open group symbol"),
            NoClosingBracket =>
                write!(f, "failed to find a closing bracket"),
            StackMustFollowGroup =>
                write!(f, "stack commands must follow a group"),
        }
    }
}