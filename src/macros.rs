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