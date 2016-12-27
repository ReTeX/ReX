enum ParseErrorType {
    UnknownCommand(Token::ControlSequence),
    UnclosedBrace,
    UnsupportedSymbol(Token::Symbol),
    LimitsMustFollowOperator,
    MultipleScripts,
    RequiredArgument,
    Expected,
}

// Add a span element to lex?

ParseError {
    // \left -> red{\left}
    // \rule{123} -> red{\rule{123}}
    // Happens at the end of a statment?
    MissingArgument(cmd),

    // \left+ ->
    // rule{asdfad}{12.3} -> Expected number. 
    Expected("Open delimiter", "")
}

MissingArgument(cmd) {
    print command and given arguments.
    cmd  = Token::Command,
    expected = 3,
    found    = 2,
    args = "{  }{  }"
}

NoArgument(cmd) {
    print command
}

// Symbol should return Option<ParseNode>
// Accents should be handled in functions.rs

struct ParseError {
    lenient: Vec<ParseNode>, // Result
    cause:   ParseErrorType,
}



// \left:
//   Strict:
//   -    
//
//   Non-Strict:
//   - If missing argument, add \red{\left}
//   - If argument is not open-type, add \red{\left} <parse expression> \red{\right}
//   - If expreesion has an error:
//      - raise if strict
//      - otherwise there shouldn't be an error (so raise anyway)
//   - If missing \right, add \rec{\left}, continue with expression
//   - If \right is missing arguments, \red{\left} <expreesion> \red{\right}
//

ParseError {
    lenient: vec![error!("\left")],
    cause:   MissingArguments(
}

fn implicit_group() -> Result<Option<ParseNode>, String> {
    let token = lex.current;

    if token == Token::ControlSequence("left") {
        lex.next();
        let left = symbol(lex, local)
            .ok_or(ErrMissingArguments! {
                    
                })?;

        if left.atom_type != AtomType::Open
            && left.atom_type != AtomType::Fence
            && left.unicode != b'.' {
            return Err(token.expected(
                "Open delimiter or .",
                left.atom_type.humanize());
        }

        let inner = expression(lex, local)?;
        lex.current.expect(Token::Command("right"))
            .ok_or(ParseError::Expected(
                "\right",
                lex.current.humanize())?

        lex.next();
        let right = symbol(lex, local)
            .ok_or(ParseError::NoArgument("right"))?;

        if right.atom_type != AtomType::Close
            && right.atop_type != AtomType::Fence
            && right.unicode != b'.' {
            return Err(token.expected(
                "Close delimiter or .",
                left.atom_type.humanize());
        }

        Ok(Some(ParseNode::Delimited(Delimited {
            left:  left,
            right: right,
            inner: inner,
        })))
    } else if token == Token::Command("right") {
        
    } else {
        Ok(None)
    }
}

// For backtracking purposes, we can set a mark position.
// which will rewind back to the previous mark.
// It may be possible to have multiple mark positions for
// a rewind.  This would require manually resetting the mark 
// position then.

// 0 argument macros won't fail, unless symbol?
fn text_operator( ) -> Result<ParseNode, Err> {
    ..
}

// 1 argument macros, 
