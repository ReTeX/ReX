#![allow(dead_code)]
use error::{ParseError, ParseResult};
use font::Style;
use font::{SYMBOLS, Symbol, OptionalAtom};
use lexer::{Lexer, Token};
use parser::nodes::{Delimited, ParseNode};
use parser::atoms::IsAtom;
use parser::AtomType;
use functions::COMMANDS;
use super::builders as build;

/// This method is served as an entry point to parsing the input.
/// It can also be used to parse sub-expressions (or more formally known)
/// as `mathlists` which can be found from parsing groups.

pub fn expression<'l>(lex: &'l mut Lexer, local: Style) -> Result<Vec<ParseNode>, String> {
    let mut ml: Vec<ParseNode> = Vec::new();

    loop {
        // TODO: Handle INFIX operators here, once we support them.
        lex.consume_whitespace();
        if lex.current.ends_expression() {
            break;
        }

        let node = alt!(
            command(lex, local),
            group(lex, local),
            symbol(lex, local),
            implicit_group(lex, local));

        // Handle commands that may change the state of the parser
        // ie: fontstyle changes.
        if node.is_none() {
            if let Some(mut nodes) = state_change(lex, local)? {
                ml.append(&mut nodes);
                continue;
            }
        }

        let node = postfix(lex, local, node)?;

        // If at this point we still haven't parsed a node, then
        // it's something we don't know how to handle.
        match node {
            Some(n) => ml.push(n),
            None => return Err(format!("unable to parse {}", lex.current)),
        }
    }

    Ok(ml)
}

fn postfix(lex: &mut Lexer,
           local: Style,
           mut prev: Option<ParseNode>)
           -> Result<Option<ParseNode>, String>
{
    const LIMITS_ERR: &'static str = "Limits must follow an operator";
    let mut superscript: Option<ParseNode> = None;
    let mut subscript: Option<ParseNode> = None;

    loop {
        lex.consume_whitespace();
        let token = lex.current;

        match token {
            Token::Symbol('_') => {
                lex.next();
                // If we already have a subscript, bail.
                if subscript.is_some() {
                    return Err("Excessive subscripts".into());
                }
                subscript = Some(math_field(lex, local)?);
            }
            Token::Symbol('^') => {
                lex.next();
                // If we already have a superscript, bail.
                if superscript.is_some() {
                    return Err("Excessive superscripts".into());
                }
                superscript = Some(math_field(lex, local)?);
            }
            Token::Command("limits") => {
                lex.next();
                let op = prev.as_mut().ok_or(LIMITS_ERR.to_string())?;
                if let AtomType::Operator(_) = op.atom_type() {
                    op.set_atom_type(AtomType::Operator(true));
                } else {
                    return Err(LIMITS_ERR.into());
                }
            }
            Token::Command("nolimits") => {
                lex.next();
                let op = prev.as_mut().ok_or(LIMITS_ERR.to_string())?;
                if let AtomType::Operator(_) = op.atom_type() {
                    op.set_atom_type(AtomType::Operator(false));
                } else {
                    return Err(LIMITS_ERR.into());
                }
            }
            _ => break,
        } // End match
    } // End loop

    if superscript.is_some() || subscript.is_some() {
        Ok(Some(build::scripts(prev, superscript, subscript)))
    } else {
        Ok(prev)
    }
}

/// Theses commands may change the state of the parser, and so will
/// be handled in a special manner.  Changinge the state of the parser
/// may also require direct access to the current list of parse nodes.

pub fn state_change(lex: &mut Lexer, style: Style) -> Result<Option<Vec<ParseNode>>, String> {
    use font::Family;
    if let Token::Command(cmd) = lex.current {
        let new_style = match cmd {
            "mathbf" =>   style.with_bold(),
            "mathit" =>   style.with_italics(),
            "mathrm" =>   style.with_family(Family::Roman),
            "mathscr" =>  style.with_family(Family::Script),
            "mathfrak" => style.with_family(Family::Fraktur),
            "mathbb" =>   style.with_family(Family::Blackboard),
            "mathsf" =>   style.with_family(Family::SansSerif),
            "mathtt" =>   style.with_family(Family::Monospace),
            "mathcal" =>  style.with_family(Family::Script),
            _ => return Ok(None),
        };

        lex.next();
        return macro_argument(lex, new_style)
    }

    Ok(None)
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

pub fn math_field(lex: &mut Lexer, local: Style) -> Result<ParseNode, String> {
    alt!(
        command(lex, local),
        group(lex, local),
        symbol(lex, local))
            .ok_or(format!("Expected a mathfield following: {:?}", lex.current))
}

/// Parse a TeX command. These commands define the "primitive" commands for our
/// typesetting system.  It (should) include a large portion of the TeX primitives,
/// along with the most useful primitives you find from amsmath and LaTeX.
/// If no matching command is found, this will return `Ok(None)`.  This method
/// can fail while parsing parameters for a TeX command.

pub fn command(lex: &mut Lexer, local: Style) -> Result<Option<ParseNode>, String> {
    // TODO: We need to build a framework, that will match commands
    let cmd = if let Token::Command(cmd) = lex.current {
        match COMMANDS.get(cmd).cloned() {
            Some(command) => command,
            None => return Ok(None),
        }
    } else {
        return Ok(None);
    };

    // A command has been found.  Consume the token and parse for arguments.
    lex.next();
    cmd.parse(lex, local)
}

/// Parse an implicit group.  An implicit group is often defined by a command
/// that implicitly has a `\bgroup` or `{` somewhere inside it's definition.  This is one
/// point where we will deviate from TeX a little bit.  We won't characterize every
/// command that will start a new implicit group (for instance, `\frac`).
///
/// This should be used almost anywhere `group()` is used.

pub fn implicit_group(lex: &mut Lexer, local: Style) -> Result<Option<ParseNode>, String> {
    let token = lex.current;

    if token == Token::Command("left") {
        lex.next(); // consume the `\left` token`
        let left = symbol(lex, local)?
            .ok_or(String::from(r#"No symbol found after `\left`"#))?
            .expect_left()?;

        let inner = expression(lex, local)?;
        lex.current.expect(Token::Command("right"))?;
        lex.next();
        let right = symbol(lex, local)?
            .ok_or(String::from(r#"No symbol found after '\right'"#))?
            .expect_right()?;

        Ok(Some(ParseNode::Delimited(Delimited {
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

pub fn group(lex: &mut Lexer, local: Style) -> Result<Option<ParseNode>, String> {
    if lex.current == Token::Symbol('{') {
        lex.next();
        let inner = expression(lex, local)?;
        lex.current.expect(Token::Symbol('}'))?;
        lex.next();
        Ok(Some(ParseNode::Group(inner)))
    } else {
        Ok(None)
    }
}

/// Parse a symbol.  Symbols can be found from a TeX command (like `\infty`)
/// or from a uniid character input.  This function will return `Ok(None)`
/// if the current token is a TeX command which is not found in the symbols
/// table. If there is no defined representation for the given `Token::Symbol`
/// then this function will return with an error.
///
/// Note, there are some `char` inputs that no work here.  For instance,
/// the `{` will not be recognized here and will therefore result in an `Err`.
/// So in general, you should always parse for a group before parsing for a symbol.

pub fn symbol(lex: &mut Lexer, local: Style) -> Result<Option<ParseNode>, String> {
    match lex.current {
        Token::Command(cs) => {
            match SYMBOLS.get(cs).cloned() {
                None => Ok(None),
                Some(sym) => {
                    lex.next();
                    use parser::nodes::Accent;
                    // If this symbol is an accent, we need to consume the next math field.
                    if sym.atom_type == AtomType::Accent {
                        let nucleus = math_field(lex, local)
                            .expect("No symbol following an accent!");
                        Ok(Some(ParseNode::Accent(Accent {
                                                      symbol: sym,
                                                      nucleus: Box::new(nucleus),
                                                  })))
                    } else {
                        Ok(Some(ParseNode::Symbol(Symbol {
                                                      unicode: local.style_symbol(sym.unicode),
                                                      ..sym
                                                  })))
                    }
                }
            }
        }
        Token::Symbol(c) => {
            Ok(match c.atom_type() {
                   None => None,
                   Some(sym) => {
                       lex.next();
                       Some(ParseNode::Symbol(Symbol {
                                                  unicode: local.style_symbol(c as u32),
                                                  atom_type: sym,
                                              }))
                   }
               })
        }
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

pub fn macro_argument(lex: &mut Lexer, local: Style) -> Result<Option<Vec<ParseNode>>, String> {
    // Must figure out how to properly handle implicit groups here.
    lex.consume_whitespace();

    let opt_node = alt!(
        group(lex, local),
        command(lex, local),
        symbol(lex, local));

    match opt_node {
        Some(ParseNode::Group(inner)) => Ok(Some(inner)),
        Some(node) => Ok(Some(vec![node])),
        _ => Ok(None),
    }
}

/// This method is like `macro_argument` except that it requires an argument to be present.

pub fn required_macro_argument(lex: &mut Lexer, local: Style) -> Result<Vec<ParseNode>, String> {
    let arg = macro_argument(lex, local)?;
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

pub fn expect_type(lex: &mut Lexer, local: Style, expected: AtomType) -> Result<Symbol, String> {
    lex.consume_whitespace();

    if let Some(ParseNode::Symbol(sym)) = symbol(lex, local)? {
        if sym.atom_type == expected {
            Ok(sym)
        } else {
            Err(format!("Expected a symbol of type {:?}, got a symbol of type {:?}",
                        expected,
                        sym.atom_type))
        }
    } else {
        Err(format!("Expected a symbol of type {:?}, got a {:?}",
                    expected,
                    lex.current))
    }
}

/// This function is the API entry point for parsing a macro.  For now, it takes a `&str`
/// and outputs a vector of parsing nodes, or an error message.

pub fn parse(input: &str) -> Result<Vec<ParseNode>, String> {
    let mut lexer = Lexer::new(input);
    let local = Style::new();

    let result = expression(&mut lexer, local);
    if lexer.current != Token::EOF {
        println!("Unexpectedly ended parsing; \
                  unmatched end of expression? \
                  Stoped parsing at {}",
                 lexer.current);
    }

    result
}


// --------------
//     TESTS
// --------------

#[cfg(test)]
mod tests {
    use parser::parse;

    #[test]
    fn fractions() {
        let mut errs: Vec<String> = Vec::new();
        should_pass!(errs, parse, [r"\frac\alpha\beta", r"\frac\int2"]);
        should_fail!(errs, parse, [r"\frac \left(1 + 2\right) 3"]);
        should_equate!(errs,
                       parse,
                       [(r"\frac12", r"\frac{1}{2}"),
                        (r"\frac \sqrt2 3", r"\frac{\sqrt2}{3}"),
                        (r"\frac \frac 1 2 3", r"\frac{\frac12}{3}"),
                        (r"\frac 1 \sqrt2", r"\frac{1}{\sqrt2}")]);
        display_errors!(errs);
    }

    #[test]
    fn radicals() {
        let mut errs: Vec<String> = Vec::new();
        // TODO: Add optional paramaters for radicals
        should_pass!(errs,
                     parse,
                     [r"\sqrt{x}",
                      r"\sqrt2",
                      r"\sqrt\alpha",
                      r"1^\sqrt2",
                      r"\alpha_\sqrt{1+2}",
                      r"\sqrt\sqrt2"]);
        should_fail!(errs, parse, [r"\sqrt", r"\sqrt_2", r"\sqrt^2"]);
        should_equate!(errs, parse, [(r"\sqrt2", r"\sqrt{2}")]);
        should_differ!(errs, parse, [(r"\sqrt2_3", r"\sqrt{2_3}")]);
        display_errors!(errs);
    }

    #[test]
    fn scripts() {
        let mut errs: Vec<String> = Vec::new();
        should_pass!(errs,
                     parse,
                     [r"1_2^3",
                      r"_1",
                      r"^\alpha",
                      r"_2^\alpha",
                      r"1_\frac12",
                      r"2^\alpha",
                      r"x_{1+2}",
                      r"x^{2+3}",
                      r"x^{1+2}_{2+3}",
                      r"a^{b^c}",
                      r"{a^b}^c",
                      r"a_{b^c}",
                      r"{a_b}^c",
                      r"a^{b_c}",
                      r"{a^b}_c",
                      r"a_{b_c}",
                      r"{a_b}_c"]);
        should_fail!(errs,
                     parse,
                     [r"1_", r"1^", r"x_x_x", r"x^x_x^x", r"x^x^x", r"x_x^x_x"]);
        should_equate!(errs,
                       parse,
                       [(r"x_\alpha^\beta", r"x^\beta_\alpha"), (r"_2^3", r"^3_2")]);
        display_errors!(errs);
    }

    #[test]
    fn delimited() {
        let mut errs: Vec<String> = Vec::new();
        should_pass!(errs,
                     parse,
                     [r"\left(\right)",
                      r"\left.\right)",
                      r"\left(\right.",
                      r"\left\vert\right)",
                      r"\left(\right\vert"]);
        should_fail!(errs,
                     parse,
                     [r"\left1\right)",
                      r"\left.\right1",
                      r"\left",
                      r"\left.{1 \right."]);
        display_errors!(errs);
    }
}
