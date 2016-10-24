"""
enum MathStyle {
    Display,
    Text,
    Script,
    ScriptScript,
    NoChange,
}

enum TexCommand {
    Radical,
    GenFraction {
        pub left_delimiter: Option<Symbol>,
        pub right_delimiter: Option<Symbol>,
        pub bar_thickness: u32,
        pub math_style: Option<MathStyle>,
    },
}
"""

def x(name, *args):
    t = template[name]
    r = defaults[name]
    a = list(*args)
    for (n, item) in enumerate(a):
        if item == 0x9e:
            a[n] = r[n]
    return t.format(*a)

#Used for default variable substitution
d = 0x9e
defaults = {
    genfrac: ("None", "None", "4", "MathStyle::NoChange"),
}

template = {
    genfrac: """\
TexCommand::GenFraction {
    pub left_delimiter: {},
    pub right_delimiter: {},
    pub bar_thickness: {},
    pub math_style: {},   
}
"""
}