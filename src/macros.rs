macro_rules! first_some {
    ($lex:ident, $first:ident, $($expr:ident,)* )  => (
        {
            if let Some(res) = $first($lex)? {
                Some(res)
            }

            $(
                else if let Some(res) = $expr($lex)? {
                    Some(res) 
                }
            )*

            else { None }
        }
    )
}

// ----------------------
// Testing related Macros
// ----------------------

macro_rules! should_fail {
    ($errs:ident, $func:ident, $iter:expr) => ({
        for item in $iter.iter() {
            if let Ok(_) = $func(item) {
                $errs.push(format!("{:?} - should have errored.\n", item));
            }
        } 
    })
}

macro_rules! should_pass {
    ($errs:ident, $func:ident, $iter:expr) => ({
        for item in $iter.iter() {
            if let Err(s) = $func(item) {
                $errs.push(format!("{:?} - should have passed.\n\tError: {:?}\n", item, s));
            }
        }
    })
}

macro_rules! should_equate {
    ($errs:ident, $func:ident, $iter:expr) => ({
        for &(l, r) in $iter.iter() {
            if $func(l) != $func(r) {
                $errs.push(format!("{:?} and {:?} - should have yielded the same results.", l, r));
            }
        }
    })    
}

macro_rules! should_differ {
    ($errs:ident, $func:ident, $iter:expr) => ({
        for &(l, r) in $iter.iter() {
            if $func(l) == $func(r) {
                $errs.push(format!("{:?} and {:?} - should have yielded different results.", l, r));
            }
        }
    })    
}

macro_rules! display_errors {
    ($errs:ident) => (
        if $errs.len() > 0 {
            for err in $errs {
                println!("\n{}", err);
            }
            panic!();
        }            
    )
}