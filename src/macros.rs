// ----------------------
// Parsing related Macros
// ----------------------

macro_rules! first_some {
    ($lex:ident, $locals:ident, $first:ident, $($expr:ident,)* )  => (
        {
            if let Some(res) = $first($lex, $locals)? {
                Some(res)
            }

            $(
                else if let Some(res) = $expr($lex, $locals)? {
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
            let l_res = $func(l);
            let r_res = $func(r);
            if l_res != r_res {
                $errs.push(format!("{:?} and {:?} - should have yielded the same results.\n\n\tLeft:  {:?}\n\n\tRight: {:?}",
                    l, r, l_res, r_res));
            }
        }
    })
}

macro_rules! should_differ {
    ($errs:ident, $func:ident, $iter:expr) => ({
        for &(l, r) in $iter.iter() {
            let l_res = $func(l);
            let r_res = $func(r);
            if l_res == r_res {
                $errs.push(format!("{:?} and {:?} - should have yielded different results.\n\n\tLeft:  {:?}\n\n\tRight: {:?}",
                    l, r, l_res, r_res));
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

macro_rules! fontunit {
    ($f:expr) => ($crate::dimensions::FontUnit::from_fp(
        $crate::dimensions::FixedFloat::from_float($f)
    ))
}
