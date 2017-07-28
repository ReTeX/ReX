extern crate rex;
use rex::Renderer;

use std::io::Write;
use std::path::Path;
use super::debug_render::Equation;


const HEADER: &'static str =
r##"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Render Tests</title>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.6.0/themes/prism-okaidia.min.css"/>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.6.0/prism.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.6.0/components/prism-latex.min.js"></script>
</head>
<body>"##;

const END: &'static str = r"</body></html>";

fn write_equation<W: Write>(f: &mut W, eq: &Equation) {
    writeln!(f, "<h2>{}</h2>", eq.description).unwrap();
    writeln!(f,
             r#"<pre><code class="language-latex">{}</code></pre>"#,
             eq.tex)
            .unwrap();

    let settings = rex::RenderSettings::default()
        .font_src("rex-xits.otf")
        .font_size(48);

    writeln!(f,
             "{}",
             rex::render::svg::render_to_string(&settings, &eq.tex).unwrap())
            .unwrap();
}

pub fn write<P: AsRef<Path>>(path: P, eqs: &[Equation]) {
    use std::fs::File;
    use std::io::BufWriter;

    let out = File::create(path.as_ref()).expect("failed to create html file for SVG diff");
    let mut writer = BufWriter::new(out);

    writer.write(HEADER.as_bytes()).unwrap();
    for eq in eqs {
        write_equation(&mut writer, eq);
    }
    writer.write(END.as_bytes()).unwrap();
}