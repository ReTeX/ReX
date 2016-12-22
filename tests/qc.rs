use std::fs::File;
use std::io::Write;
use std::fmt;

extern crate rex;

use rex::parser::parse;
use rex::render::Renderer;
use rex::layout::engine::layout;
use rex::layout::Style;

const HEADER: &'static str =
r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Testing Things</title>
    <style>
        @font-face {
            font-family: rex;
            src: url('out/rex-xits.otf');
        }
    </style>
</head>

<body>"#;

const END: &'static str = r"</body></html>";

// We will group up tests into categories.  Each category
// will contain a description, along with a list of tests,
// each of which contains a description of their test.
struct Tests(Vec<Categories>);

struct Categories {
    description: &'static str,
    tests:       Vec<Test>,
}

struct Test {
    description: &'static str,
    tests:       Vec<&'static str>,
}

impl fmt::Display for Tests {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "<h1>Tests</h1>")?;

        for test in &self.0 {
            write!(f, "{}", test)?;
        }

        Ok(())
    }
}

impl fmt::Display for Categories {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "<h2>{}</h2>", self.description)?;

        for sub_test in &self.tests {
            write!(f, "{}", sub_test)?;
        }

        Ok(())
    }
}

impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "<h3>{}</h3>", self.description)?;

        for test in &self.tests {
            let mut p = parse(test).unwrap();
            let r = layout(&mut p, Style::Display);
            let output = Renderer::new(r).render();

            writeln!(f, r#"<p class="tex">{}</p><p>{}</p>"#, test, output)?;
        }

        Ok(())
    }
}

macro_rules! cat {
    ($desc:expr => $($test:expr),* ) => ({
        Categories {
            description: $desc,
            tests:       {
                let mut v = vec![];
                $( v.push($test); )*
                v
            },
        }
    })
}

macro_rules! test {
    ($desc:expr, $( $test:expr ),*) => ({
        Test {
            description: $desc,
            tests: {
                let mut v = vec![];
                $( v.push($test); )*
                v
            }
        }
    })
}

#[test]
fn test_images() {
    let results = Tests(vec![
    cat!("Scripts" =>
      test!("Properly scale with standard algorithm",
        r"a^2\textstyle a^2\scriptstyle a^2 \scriptscriptstyle a^2",
        r"a_2\textstyle a_2\scriptstyle a_2 \scriptscriptstyle a_2",
        r"a_2^2\textstyle a_2^2\scriptstyle a_2^2 \scriptscriptstyle a_2^2"),

      test!("Properly scale with nolimits",
        r"\int_0^1\textstyle\int_0^1\scriptstyle\int_0^1\scriptscriptstyle\int_0^1"),

      test!("Properly scale operator with limits",
        r"\sum_0^k\textstyle\sum_0^k\scriptstyle\sum_0^k\scriptscriptstyle\sum_0^k")
    ),

    cat!("Fractions" =>
      test!("Should properly scale",
        r"\frac12\textstyle\frac12\scriptstyle\frac12\scriptscriptstyle\frac12"),

      test!("Should center",
        r"\frac{1}{x+y}\frac{x+y}{1}",
        r"\textstyle\frac{1}{x+y}\frac{x+y}{1}",
        r"\scriptstyle\frac{1}{x+y}\frac{x+y}{1}",
        r"\scriptscriptstyle\frac{1}{x+y}\frac{x+y}{1}"),

      test!("Should handle tall symbols",
        r"\frac{x}{\int x}",
        r"\frac{\int x}{x}",
        r"\frac{\int x}{\int x}"),

      test!("Should handle depth",
        r"\frac{g}{x}\frac{x}{x}", r"\frac{x}{g}\frac{x}{x}")
    ),

    ]);

    let output = format!("{}\n{}\n{}", HEADER, results, END);

    let mut f = File::create("test.html").unwrap();
    f.write_all(output.as_bytes()).unwrap();
}