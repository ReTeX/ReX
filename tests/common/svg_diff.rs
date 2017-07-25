use super::debug_render::DebugRule;
use super::debug_render::DebugSymbol;
use super::debug_render::Equation;
use super::debug_render::Object;

use std::cmp::max;
use std::io::Write;
use std::path::Path;

const HEADER: &'static str =
r##"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Layout Tests</title>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.6.0/themes/prism-okaidia.min.css"/>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.6.0/prism.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.6.0/components/prism-latex.min.js"></script>
</head>
<body>"##;

const END: &'static str = r"</body></html>";

fn write_equations<W: Write>(f: &mut W, old: Equation, new: Equation) {
    writeln!(f, "<h2>{}</h2>", old.description).unwrap();
    writeln!(f,
             r#"<pre><code class="language-latex">{}</code></pre>"#,
             old.tex)
            .unwrap();

    let width = max(old.width, new.width);
    let height = max(old.height, new.height);

    let px_width = f64::from(width) / 1000.0 * 48.0;
    let px_height = f64::from(height) / 1000.0 * 48.0;

    writeln!(f,
             r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="{:2}" height="{:2}" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">
    <defs>
        <style type="text/css">
            @font-face{{font-family:rex; src:url('rex-xits.otf');}}
            .blend {{ mix-blend-mode: multiply; }}
            .old   {{ fill: #3A5BA6 }}
            .new   {{ fill: #EA312F }}
        </style>
    </defs>
    <g font-family="rex" font-size="1000">"#,
             px_width,
             px_height,
             width,
             height)
            .unwrap();

    writeln!(f, r##"        <g class="blend old">"##).unwrap();
    write_objects(f, &old.render);
    writeln!(f, r##"        </g>"##).unwrap();
    writeln!(f, r##"        <g class="blend new">"##).unwrap();
    write_objects(f, &new.render);
    writeln!(f, r##"        </g>"##).unwrap();
    writeln!(f, r##"    </g>"##).unwrap();
    writeln!(f, r##"</svg>"##).unwrap();
}

fn write_objects<W: Write>(f: &mut W, objects: &[Object]) {
    for object in objects {
        match *object {
            Object::Symbol(ref sym) => write_symbol(f, sym),
            Object::Rule(ref rule) => write_rule(f, rule),
            Object::Color(_, ref objects) => write_objects(f, objects),
        }
    }
}

fn write_rule<W: Write>(f: &mut W, rule: &DebugRule) {
    writeln!(f,
             r##"<rect x="{}" y ="{}" width="{}" height="{}"/>"##,
             rule.x,
             rule.y,
             rule.width,
             rule.height)
            .expect("Failed to write to buffer!");
}

fn write_symbol<W: Write>(f: &mut W, sym: &DebugSymbol) {
    use std::char::from_u32;

    if sym.scale != 1. {
        writeln!(f,
                 r#"<text transform="translate({}, {}) scale({:.2})">{}</text>"#,
                 sym.x,
                 sym.y,
                 sym.scale,
                 from_u32(sym.codepoint).expect("Unabale to decode utf8 code-point!"))
                .expect("Failed to write to buffer!");
    } else {
        writeln!(f,
                 r#"<text transform="translate({}, {})">{}</text>"#,
                 sym.x,
                 sym.y,
                 from_u32(sym.codepoint).expect("Unabale to decode utf8 code-point!"))
                .expect("Failed to write to buffer!");
    }
}

pub fn write_diff<P: AsRef<Path>>(path: P, diff: Vec<(Equation, Equation)>) {
    use std::fs::File;
    use std::io::BufWriter;

    let out = File::create(path.as_ref()).expect("failed to create html file for SVG diff");
    let mut writer = BufWriter::new(out);

    writer.write(HEADER.as_bytes()).unwrap();
    for (old, new) in diff {
        write_equations(&mut writer, old, new);
    }
    writer.write(END.as_bytes()).unwrap();
}