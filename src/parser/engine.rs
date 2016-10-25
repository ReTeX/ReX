#![allow(dead_code)]
// TODO: Figure out how to handle functions which are in Symbols table.

use lexer::{Lexer, Token};
use symbols::{SYMBOLS, Symbol, IsSymbol, FontMode};
use parser::nodes::{ AtomType, Delimited, ParseNode };

use functions::COMMANDS;

/// This method is served as an entry point to parsing the input.
/// It can also but used to parse sub-expressions (or more formally known)
/// as `mathlists` which can be found from parsing groups.
///
/// This method will always return something, though it may be an emptylist.
/// This method itself will not fail, but it is possible that expressions
/// inside this method will fail and raise and error. 

fn expression(lex: &mut Lexer) -> Result<Vec<ParseNode>, String> {
    let mut ml: Vec<ParseNode> = Vec::new();

    loop {
        // TODO: We need to check parsing mode here for properly handling spaces.
        // TODO: Handle INFIX operators here.
        lex.consume_whitespace();
        if lex.current.ends_expression() { break; }

        let node = first_some!(lex, command, group, symbol, implicit_group,)
            .expect(&format!("Unable to parse token: {:?}", lex.current));

        // TODO: We need to handle script parsing here.

        ml.push(node);
    }

    Ok(ml)
}

/// Parse a `<Math Field>`.  A math field is defined by
///
/// ```bnf,ignore
/// <Math_Field> = <filler><Symbol> | <filler>{<mathmode material>}
/// ```
///
/// See page 289 of the TeX book for more details.
/// This method will result in an error if either the `Symbol` or
/// `<mathmode material>` contains an error, or if no match is found.

pub fn math_field(lex: &mut Lexer) -> Result<ParseNode, String> {
    while lex.current == Token::WhiteSpace {
        lex.next();
    }

    let result = first_some!(lex, group, symbol,);
    result.ok_or("Expected a math field, but no matches were found.".to_string())
}

/// Parse a TeX command. These commands define the "primitive" commands for our
/// typesetting system.  It (should) include a large portion of the TeX primitives,
/// along with the most useful primitives you find from amsmath and LaTeX.
/// If no matching command is found, this will return `Ok(None)`.  This method
/// can fail while parsing parameters for a TeX command.

pub fn command(lex: &mut Lexer) -> Result<Option<ParseNode>, String> {
    // TODO: We need to build a framework, that will match commands 
    let cmd = if let Token::ControlSequence(cmd) = lex.current {
        match COMMANDS.get(cmd).cloned() {
            Some(command) => command,
            None => return Ok(None),
        }
    } else {
        return Ok(None)
    };

    // A command has been found.  Consume the token and parse for arguments. 
    lex.next();
    cmd.parse(lex)
}

/// Parse an implicit group.  An implicit group is often defined by a command
/// that implicitly has a `\bgroup` or `{` somewhere inside it's definition.  This is one
/// point where we will deviate from TeX a little bit.  We won't characterize every
/// command that will start a new implicit group (for instance, `\frac`).
///
/// This should be used almost anywhere `group()` is used.

pub fn implicit_group(lex: &mut Lexer) -> Result<Option<ParseNode>, String> {
    let token = lex.current;

    if token == Token::ControlSequence("left") {
        lex.next(); // consume the `\left` token`
        let left  = expect_type(lex, AtomType::Open)?;
        let inner = expression(lex)?;
        lex.current.expect(Token::ControlSequence("right"))?;
        lex.next();
        let right = expect_type(lex, AtomType::Close)?;

        Ok(Some(ParseNode::Delimited(Delimited{
            left: left,
            right: right,
            inner: inner,
        })))
    } else {
        Ok(None)
    }
}

/// Parse a group.  Which is defined by `{<mathmode material>}`.
/// This function will return `Ok(None)` if it does not find a `{`,
/// and will `Err` if it finds a `{` with no terminating `}`, or if
/// there is a syntax error from within `<mathmode material>`.

// TODO: This should also recognize `\bgroup` if we decide to go that route.

pub fn group(lex: &mut Lexer) -> Result<Option<ParseNode>, String> {
    if lex.current == Token::Symbol('{') {
        lex.next();
        let inner = expression(lex)?;
        lex.current.expect(Token::Symbol('}'))?;
        lex.next();
        Ok(Some(ParseNode::Group(inner)))
    } else {
        Ok(None)
    }
}

/// Parse a symbol.  Symbols can be found from a TeX command (like `\infty`)
/// or from a unicode character input.  This function will return `Ok(None)`
/// if the current token is a TeX command which is not found in the symbols
/// table. If there is no defined representation for the given `Token::Symbol`
/// then this function will return with an error.
///
/// Note, there are some `char` inputs that no work here.  For instance,
/// the `{` will not be recognized here and will therefore result in an `Err`.
/// So in general, you should always parse for a group before parsing for a symbol.

pub fn symbol(lex: &mut Lexer) -> Result<Option<ParseNode>, String> {
    match lex.current {
        Token::ControlSequence(cs) => {
            match SYMBOLS.get(cs).cloned() {
                None => Ok(None),
                Some(sym) => { lex.next(); Ok(Some(ParseNode::Symbol(sym))) },
            }
        },
        Token::Symbol(c) => {
            // TODO: Properly handle fontmode here.
            match c.atom_type(FontMode::Italic) {
                None => Err(format!("Unable to find symbol representation for {}", c)),
                Some(sym) => { lex.next(); Ok(Some(ParseNode::Symbol(sym))) },
            }
        },
        _ => Ok(None),
    }
}

/// This method expects to parse a single macro argument.  Whitespace will not be consumed
/// while parsing this argument, unless the argument is a command.
/// A macro argument will consume a single token, unless there is a group found { }.
/// In which case, a macro_argument will strip the surrounding { }.  Because of this,
/// the result may be either a single ParseNode, or a vector of ParseNodes.
///
/// Open questions:
///   - How to properly inline a vector of parsenodes?
///   - When can this possible fail?
///   - How to handle custom validators/parsers for arguments. ie: Argument is a color.

pub fn macro_argument(lex: &mut Lexer) -> Result<Option<Vec<ParseNode>>, String> {
    // Must figure out how to properly handle implicit groups here.
    match first_some!(lex, group, symbol,) {
        Some(ParseNode::Symbol(sym)) => Ok(Some(vec![ParseNode::Symbol(sym)])),
        Some(ParseNode::Group(inner)) => Ok(Some(inner)),
        _ => Ok(None),
    }
}

/// This method is like `macro_argument` except that it requires an argument to be present.

pub fn required_macro_argument(lex: &mut Lexer) -> Result<Vec<ParseNode>, String> {
    let arg = macro_argument(lex)?;
    match arg {
        None => Err(format!("Expected a required macro argument! {:?}", arg)),
        Some(res) => Ok(res),
    }
}

/// DOCUMENT ME

#[allow(unused_variables)]
pub fn optional_macro_argument(lex: &mut Lexer) -> Result<Option<Vec<ParseNode>>, String> {
    unimplemented!()
}

/// This method will be used to allow for customized macro argument parsing?

#[allow(unused_variables)]
pub fn special_macro_argument(lex: &mut Lexer) -> () {
    unimplemented!()
}

/// This method expects that the current token has a given atom type.  This method
/// will frist strip all whitespaces first before inspecting the current token.
/// This function will Err if the expected symbol doesn't have the given type,
/// otherwise it will return `Ok`.
///
/// This function _will_ advance the lexer.

pub fn expect_type(lex: &mut Lexer, expected: AtomType) -> Result<Symbol, String> {
    lex.consume_whitespace();

    if let Some(ParseNode::Symbol(sym)) = symbol(lex)? {
        if sym.atom_type == expected {
            Ok(sym)
        } else {
            Err(format!("Expected a symbol of type {:?}, got a symbol of type {:?}",
                expected, sym.atom_type))
        }
    } else {
        Err(format!("Expected a symbol of type {:?}, got a {:?}", expected, lex.current))
    }
}

/// This function is the API entry point for parsing a macro.  For now, it takes a `&str`
/// and outputs a vector of parsing nodes, or an error message.

pub fn parse(input: &str) -> Result<Vec<ParseNode>, String> {
    let mut lexer = Lexer::new(input);
    expression(&mut lexer)
}


// --------------
//     TESTS      
// --------------

#[cfg(test)]
mod tests {
    use parser::nodes::{ ParseNode, AtomType, Radical, Delimited };
    use parser::parse;
    use symbols::Symbol;

    #[test]
    fn parser() {
        assert_eq!(parse(r"").unwrap(), vec![]);
        
        assert_eq!(parse(r" 1 + \sqrt   2").unwrap(), parse(r"1+\sqrt2").unwrap());
        assert_eq!(parse(r"\sqrt  {  \sqrt  2 }").unwrap(), parse(r"\sqrt{\sqrt2}").unwrap());

        assert_eq!(parse(r"1 + {2 + 3}").unwrap(),
            vec![ParseNode::Symbol(Symbol { code: 120803, atom_type: AtomType::Alpha }), 
                ParseNode::Symbol(Symbol { code: 43, atom_type: AtomType::Binary }), 
                ParseNode::Group(vec![ParseNode::Symbol(Symbol { code: 120804, atom_type: AtomType::Alpha }), 
                    ParseNode::Symbol(Symbol { code: 43, atom_type: AtomType::Binary }), 
                    ParseNode::Symbol(Symbol { code: 120805, atom_type: AtomType::Alpha })
            ])]);

        assert_eq!(parse(r"1+\left(3+2\right)=6").unwrap(),
            vec![ParseNode::Symbol(Symbol { code: 120803, atom_type: AtomType::Alpha }), 
                ParseNode::Symbol(Symbol { code: 43, atom_type: AtomType::Binary }), 
                ParseNode::Delimited(Delimited { 
                    left: Symbol { code: 40, atom_type: AtomType::Open }, 
                    right: Symbol { code: 41, atom_type: AtomType::Close }, 
                    inner: vec![ParseNode::Symbol(Symbol { code: 120805, atom_type: AtomType::Alpha }), 
                       ParseNode::Symbol(Symbol { code: 43, atom_type: AtomType::Binary }), 
                       ParseNode::Symbol(Symbol { code: 120804, atom_type: AtomType::Alpha })],
                }), 
                ParseNode::Symbol(Symbol { code: 61, atom_type: AtomType::Relation }), 
                ParseNode::Symbol(Symbol { code: 120808, atom_type: AtomType::Alpha })]);
        
        assert_eq!(parse(r"1+\sqrt2").unwrap(),
            vec![ParseNode::Symbol(Symbol { code: 120803, atom_type: AtomType::Alpha }), 
                 ParseNode::Symbol(Symbol { code: 43, atom_type: AtomType::Binary }), 
                 ParseNode::Radical(Radical { 
                    inner: vec![ParseNode::Symbol(Symbol { code: 120804, atom_type: AtomType::Alpha })] 
                 })]);

        assert_eq!(parse(r"1+\sqrt2"), parse(r"1+\sqrt{2}"));
        assert_ne!(parse(r"1+\sqrt2 + 3"), parse(r"1+\sqrt{2 + 3}"));
        assert_eq!(parse(r"\frac12"), parse(r"\frac{1}{2}"));
        assert_eq!(parse(r"\binom{2}1"), parse(r"\binom2{1}"));
    }
}