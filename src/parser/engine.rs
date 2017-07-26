#![allow(dead_code)]
use error::{Error, Result};
use font::Style;
use font::style::style_symbol;
use font::Symbol;
use font::symbols::SYMBOLS;
use lexer::{Lexer, Token};
use parser::nodes::{Delimited, ParseNode, Accent};
use font::AtomType;
use functions::COMMANDS;
use super::builders as build;

/// This function is served as an entry point to parsing the input.
/// It can also be used to parse sub-expressions (or more formally known
/// as `mathlists`) which occur when parsing groups.

pub fn expression(lex: &mut Lexer, local: Style) -> Result<Vec<ParseNode>> {
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

            // At this point, if the current `Token` is a Command,
            // then it must be an unrecognized Command.
            if let Token::Command(cmd) = lex.current {
                return Err(Error::UnrecognizedCommand(cmd.into()))
            }
        }

        // Like state changing functions, post-fix operators are handled
        // as a special case since they need access to the currently
        // processed node.
        let node = postfix(lex, local, node)?;

        // If at this point, we still haven't processed a node then we must have
        // an unrecognized symbol (perhaps from non-english, non-greek).
        // TODO: We should allow for more dialects.
        match node {
            Some(n) => ml.push(n),
            None => {
                match lex.current {
                    Token::Symbol(c) => return Err(Error::UnrecognizedSymbol(c)),
                    _ => unreachable!(),
                }
            }
        }
    }
    Ok(ml)
}

fn postfix(lex: &mut Lexer, local: Style, mut prev: Option<ParseNode>) -> Result<Option<ParseNode>>
{
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
                    return Err(Error::ExcessiveSubscripts);
                }
                subscript = Some(math_field(lex, local)?);
            }
            Token::Symbol('^') => {
                lex.next();
                // If we already have a superscript, bail.
                if superscript.is_some() {
                    return Err(Error::ExcessiveSuperscripts);
                }
                superscript = Some(math_field(lex, local)?);
            }
            Token::Command("limits") => {
                lex.next();
                let op = prev.as_mut().ok_or(Error::LimitsMustFollowOperator)?;
                if let AtomType::Operator(_) = op.atom_type() {
                    op.set_atom_type(AtomType::Operator(true));
                } else {
                    return Err(Error::LimitsMustFollowOperator);
                }
            }
            Token::Command("nolimits") => {
                lex.next();
                let op = prev.as_mut().ok_or(Error::LimitsMustFollowOperator)?;
                if let AtomType::Operator(_) = op.atom_type() {
                    op.set_atom_type(AtomType::Operator(false));
                } else {
                    return Err(Error::LimitsMustFollowOperator);
                }
            }
            _ => break,
        }
    }

    if superscript.is_some() || subscript.is_some() {
        Ok(Some(build::scripts(prev, superscript, subscript)))
    } else {
        Ok(prev)
    }
}

/// Theses commands may change the state of the parser, and so will
/// be handled in a special manner.  Changinge the state of the parser
/// may also require direct access to the current list of parse nodes.

pub fn state_change(lex: &mut Lexer, style: Style) -> Result<Option<Vec<ParseNode>>> {
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
        return required_argument(lex, new_style).map(Some)
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

pub fn math_field(lex: &mut Lexer, local: Style) -> Result<ParseNode> {
    alt!(
        command(lex, local),
        group(lex, local),
        symbol(lex, local))
            .ok_or(Error::ExpectedMathField(lex.current.into()))
}

/// Parse a TeX command. These commands define the "primitive" commands for our
/// typesetting system.  It (should) include a large portion of the TeX primitives,
/// along with the most useful primitives you find from amsmath and LaTeX.
/// If no matching command is found, this will return `Ok(None)`.  This method
/// can fail while parsing parameters for a TeX command.

pub fn command(lex: &mut Lexer, local: Style) -> Result<Option<ParseNode>> {
    if let Token::Command(cmd) = lex.current {
        match COMMANDS.get(cmd) {
            Some(ref cmd) => {
                lex.next();
                cmd.parse(lex, local)
            },
            None => Ok(None),
        }
    } else {
        Ok(None)
    }
}

/// Parse an implicit group.  An implicit group is often defined by a command
/// that implicitly has a `\bgroup` or `{` somewhere inside it's definition.  This is one
/// point where we will deviate from TeX a little bit.  We won't characterize every
/// command that will start a new implicit group (for instance, `\frac`).
///
/// This should be used almost anywhere `group()` is used.

pub fn implicit_group(lex: &mut Lexer, local: Style) -> Result<Option<ParseNode>> {
    let token = lex.current;

    if token == Token::Command("left") {
        lex.next();
        let left = symbol(lex, local)?
            .ok_or(Error::ExpectedSymbol(lex.current.into()))?
            .expect_left()?;

        let inner = expression(lex, local)?;
        lex.current.expect_command("right")?;
        lex.next();
        let right = symbol(lex, local)?
            .ok_or(Error::ExpectedSymbol(lex.current.into()))?
            .expect_right()?;

        Ok(Some(delimited!(left, right, inner)))
    } else {
        Ok(None)
    }
}

/// Parse a group.  Which is defined by `{<mathmode material>}`.
/// This function will return `Ok(None)` if it does not find a `{`,
/// and will `Err` if it finds a `{` with no terminating `}`, or if
/// there is a syntax error from within `<mathmode material>`.

pub fn group(lex: &mut Lexer, local: Style) -> Result<Option<ParseNode>> {
    if lex.current == Token::Symbol('{') {
        lex.next();
        let inner = expression(lex, local)?;
        lex.current.expect_symbol('}')?;
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

pub fn symbol(lex: &mut Lexer, local: Style) -> Result<Option<ParseNode>> {
    match lex.current {
        Token::Command(cs) => {
            if let Some(sym) = SYMBOLS.get(cs).cloned() {
                lex.next();

                if sym.atom_type == AtomType::Accent {
                    let nucleus = math_field(lex, local)
                        .map_err(|_| Error::AccentMissingArg(cs.into()))?;

                    Ok(Some(accent!(sym, nucleus)))
                } else {
                    Ok(Some(ParseNode::Symbol(Symbol {
                        unicode: style_symbol(sym.unicode, local),
                        ..sym
                    })))
                }
            } else {
                Ok(None)
            }
        }
        Token::Symbol(c) => {
            match codepoint_atom_type(c) {
                None => Ok(None),
                Some(sym) => {
                    lex.next();
                    Ok(Some(symbol!(style_symbol(c as u32, local), sym)))
               }
            }
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
/// This method is like `macro_argument` except that it requires an argument to be present.

pub fn required_argument(lex: &mut Lexer, local: Style) -> Result<Vec<ParseNode>> {
    lex.consume_whitespace();

    let opt_node = alt!(
        group(lex, local),
        command(lex, local),
        symbol(lex, local));

    match opt_node {
        Some(ParseNode::Group(inner)) => Ok(inner),
        Some(node) => Ok(vec![node]),
        _ => Err(Error::RequiredMacroArg),
    }
}

pub fn required_argument_with<F, O>(lex: &mut Lexer, local: Style, f: F) -> Result<O>
    where F: FnOnce(&mut Lexer, Style) -> Result<O>
{
    lex.consume_whitespace();

    if lex.current == Token::Symbol('{') {
        lex.next();
        let parsed = f(lex, local)?;
        lex.current.expect_symbol('}')?;
        lex.next();
        Ok(parsed)
    } else {
        f(lex, local)
    }
}

/// This method expects that the current token has a given atom type.  This method
/// will frist strip all whitespaces first before inspecting the current token.
/// This function will Err if the expected symbol doesn't have the given type,
/// otherwise it will return `Ok`.
///
/// This function _will_ advance the lexer.

pub fn expect_type(lex: &mut Lexer, local: Style, expected: AtomType) -> Result<Symbol> {
    lex.consume_whitespace();

    if let Some(ParseNode::Symbol(sym)) = symbol(lex, local)? {
        if sym.atom_type == expected {
            Ok(sym)
        } else {
            Err(Error::ExpectedAtomType(expected, sym.atom_type))
        }
    } else {
        Err(Error::ExpectedSymbol(lex.current.into()))
    }
}

/// This function is the API entry point for parsing a macro.  For now, it takes a `&str`
/// and outputs a vector of parsing nodes, or an error message.

// TODO: This should return a result.
pub fn parse(input: &str) -> Result<Vec<ParseNode>> {
    let mut lexer = Lexer::new(input);
    let local = Style::new();

    let result = expression(&mut lexer, local)?;

    if lexer.current != Token::EOF {
        panic!("Unexpectedly ended parsing; \
                unmatched end of expression? \
                Stoped parsing at {}",
                lexer.current);
    }

    Ok(result)
}

/// Helper function for determining an atomtype based on a given codepoint.
/// This is primarily used for characters while processing, so may give false
/// negatives when used for other things.
fn codepoint_atom_type(codepoint: char) -> Option<AtomType> {
    Some(match codepoint {
        'a'...'z' | 'A'...'Z' |
        '0'...'9' | 'Α'...'Ω' | 'α'...'ω'  => AtomType::Alpha,

        '*' | '+' | '-' => AtomType::Binary,
        '[' | '(' => AtomType::Open,
        ']' | ')' | '?' | '!' => AtomType::Close,
        '=' | '<' | '>' | ':' => AtomType::Relation,
        ',' | ';' => AtomType::Punctuation,
        '|' |  '/' | '@' | '.' | '"' => AtomType::Alpha,
        _ => return None,
    })
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
