header = """\
use phf;
use symbols::Symbol;
use parser::nodes::{ AtomType, ParseNode, Radical, GenFraction };
use lexer::Lexer;
use parser;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MathStyle {
    Display,
    Text,
    Script,
    ScriptScript,
    NoChange,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TexCommand {
    Radical,
    GenFraction {
        left_delimiter: Option<Symbol>,
        right_delimiter: Option<Symbol>,
        bar_thickness: u8,
        math_style: MathStyle,
    },
    DelimiterSize {
        atom_type: AtomType,
        size: u8,
    }
}

macro_rules! default {
    ($t:ident) => ({
        if $t.is_none 
    })
}

impl TexCommand {
    #[allow(dead_code, unused_variables)]
    pub fn parse(self, lex: &mut Lexer) -> Result<Option<ParseNode>, String> {
        Ok(match self {
            TexCommand::Radical =>
                Some(ParseNode::Radical(Radical {
                    inner: parser::required_macro_argument(lex)?,
                })),
            TexCommand::GenFraction { 
                left_delimiter: ld, 
                right_delimiter: rd, 
                bar_thickness: bt, 
                math_style: ms 
            } =>
                // TODO: Change math style here.
                Some(ParseNode::GenFraction(GenFraction{
                    left_delimiter: ld,
                    right_delimiter: rd,
                    bar_thickness: bt,
                    numerator: parser::required_macro_argument(lex)?,
                    denominator: parser::required_macro_argument(lex)?,
                })),
            TexCommand::DelimiterSize {
                size: s,
                atom_type: at,
            } =>
                Some(ParseNode::Symbol(parser::expect_type(lex, at)?)),
        })
    }
}
"""
def sym(c, t):
    return "Some(Symbol {{ code: '{}' as u32, atom_type: AtomType::{} }})"\
        .format(c, t)
def sty(s):
    return "MathStyle::{}".format(s)   

# Give name, and list of defaults
# Output string for value of hashmap
def sub(name, *args):
    tem = template[name]
    res = defaults[name]
    a   = list(args)
    for (n, item) in enumerate(a):
        if item == 0x9e:
            a[n] = res[n-1]
    return tem.format(*a[1:])


def phf(f, *args):
    string = ""
    for item in args:
        key = item[0]
        for thing in item[1]:
            f.write('    "' + thing[0] + '" => ' + sub(key, *thing) + ',\n')
    

#Used for default variable substitution
d = 0x9e
defaults = {
    "genfrac": ("None", "None", "4", "MathStyle::NoChange"),
    "sqrt": (),
    "delim_size": (),
}

template = {
    "genfrac": """\
TexCommand::GenFraction {{\
 left_delimiter: {},\
 right_delimiter: {},\
 bar_thickness: {},\
 math_style: {} }}""",
    "sqrt": "TexCommand::Radical",
    "delim_size": """\
TexCommand::DelimiterSize {{\
 size: {},\
 atom_type: AtomType::{} }}"""
}

with open('../src/functions.rs', 'w', newline='\n') as f:
    f.write(header)
    f.write("\npub static COMMANDS: phf::Map<&'static str, TexCommand> = phf_map! {\n")
    phf(f,
    ("genfrac",
        [("frac", d, d, d, d),
        ("tfrac", d, d, d, sty("Text")),
        ("dfrac", d, d, d, sty("Display")),
        ("binom", sym('{', "Open"), sym('}', "Close"), 0, d),
        ("tbinom", sym('{', "Open"), sym('}', "Close"), 0, sty("Text")),
        ("dbinom", sym('{', "Open"), sym('}', "Close"), 0, sty("Display")),
        ]),
    ("sqrt",
        [("sqrt",)]),
    ("delim_size",
        [("bigl", 1, "Open"), ("Bigl", 2, "Open"), ("biggl", 3, "Open"), ("Biggl", 4, "Open"),
        ("bigr", 1, "Close"), ("Bigr", 2, "Close"), ("biggr", 3, "Close"), ("Biggr", 4, "Close"),
        ("bigm", 1, "Relation"), ("Bigm", 2, "Relation"), ("biggm", 3, "Relation"), ("Biggm", 4, "Relation"),
        ("big", 1, "Ordinal"), ("Big", 2, "Ordinal"), ("bigg", 3, "Ordinal"), ("Bigg", 4, "Ordinal"),
        ]),
    )
    f.write("};")