# Trait Assisted Parsing 

typedef Expression  = Vec<ParseNode>;
typedef ParseResult = Option<ParseNode>;

struct Delimited {
    left:  Symbol,
    right: Symbol,
    inner: Expression,
}

pub enum ParseNode {
    Symbol    (Symbol),
    Delimited (Delimited),
}

struct ParseError {
    lenient: Expression,
    cause:   ParseErrorType,
}

trait Parse {
    fn parse(&mut Lexer, Locals) -> Result<ParseNode>;
}

impl Parse for Delimited {
    fn parse(lex: &mut Lexer, local: Locals) -> Result<ParseNode> {
        lex.next();
        let left = parse.symbol()
            .ok_or(MissingArguments)?;

        if left.atom != AtomType::Open
            && left.atom != AtomType::Fence
            && left.unicode != b'.' {
            return Err(Expected("open delimiter or .", left));
        }

        let inner = parse.expression()?;
        
        if lex.current != Token::Command("right") {
            return Err(Expected(r"closing \right", lex.current))
        }
        
        lex.next();
        let right = parse.symbol()
            .ok_or(MissingArguments)?;
            
        if right.atom != AtomType::Close
            && right.atom != AtomType::Fence
            && right.unicode != b'.' {
            return Err(Expected("closing delimiter or .", right));
        }
        
        Ok(Some(ParseNode::Delimited(Delimited {
            left:  left,
            right: right,
            inner: inner,
        })))
    }
}

/// Handle postfix commands, include:
///   superscripts, subscripts, limit modifications 
/// The limit modifications can _modify_ the previous
/// element from the parse node.
fn postfix(prev: mut Option<ParseNode>) -> Result<ParseNode> {
    let supscript: Option<Expression> = None;
    let subscript: Option<Expression> = None;

    loop {
        lex.consume_whitespace();
        let token = lex.current;
        
        match token {
            Token::Symbol('_') => {
                // If we have already found a subscript, fail.
                if subscript.is_some() {
                    return Err(ExcessiveSubscripts);
                }
                subscript = Some(parse.mathfield()?);
            },
            Token::Symbol('^') => {
                if supscript.is_some() {
                    return Err(ExcessiveSuperscripts);
                }
                supscript = Some(parse.mathfield()?);
            },
            Token::Command("limits") => {
                let op = prev.as_mut()
                    .unwrap_or(Err(LimitsMustFollowOperator))?;
                    
                if AtomType::Operator(limits) = op.atom {
                    op.atom = AtomType::Operator(true);
                } else {
                    return Err(LimitsMustFollowOperator);
                }
            },
            Token::Command("nolimits") => {
                let op = prev.as_mut()
                    .unwrap_of(Err(LimitsMustFollowOperator))?;
                    
                if AtomType::Operator(limits) = op.atom {
                    op.atom = AtomType::Operator(false);
                } else {
                    return Err(LimitsMustFollowOperator);
                }
            }
        } // End match
    } // End loop
    
    Ok(if supscript.is_some() || subscript.is_some() {
        Some(ParseNode::Scripts(Scripts {
            subscript:   subscript,
            superscript: superscript,
            base:        prev,        
        })))
    } else {
        prev
    })
}

// Upon error:
//   - Missing Argument: \red{\left} <continue>
//   - Expected (atom):  \red{\left} <continue>
//   - Missing Symbol:   \red{\left} <continue>
//   - \right erros:
//      - \red{\left} <parsed expression> \red{\right} <continue>
//   NBB: \right errors will return a vector, appended or an
//        error message printed.

fn implicit_group() -> Result<ParseResult> {
    let token = lex.current;

    match token {
        Token::Command("left")  => Delimited::parse(lex, locals).as_node();
        Token::Command("begin") => {
            // determine the type of enviornment
            let env = lex.raw_group();

            match env {
                "matrix"  => Matrix::parse(lex, locals, MatrixDelim::None);
                "pmatrix" => Matrix::parse(lex, locals, MatrixDelim::Paren);
                "bmatrix" => Matrix::parse(lex, locals, MatrixDelim::Bracket);
                "Bmatrix" => Matrix::parse(lex, locals, MatrixDelim::Brace);
                "vmatrix" => Matrix::parse(lex, locals, MatrixDelim::Vert);
                "Vmatrix" => Matrix::parse(lex, locals, MatrixDelim::VertVert);

                "cases"   => 
                "aligned" =>
                "align"   =>
                _         => Err(UnrecognizedEnvironment(env)),
            }
        },
        Token::Command("end")   => Error::Unexpected(..);
        Token::Command("right") => Error::Unexpected(..);
        _ => Ok(None),
    }
}

Result<ParseResult>
