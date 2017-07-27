extern crate rex;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use rex::{SVGRenderer, RenderSettings, Renderer};
use std::fs::File;
use std::io::BufReader;
//use std::io::Sink;

type SvgSink<'a> = SVGRenderer<'a, Vec<u8>>;

#[derive(Debug, Serialize, Deserialize)]
struct Tests {
    #[serde(rename="Pass")]
    pass: Vec<String>,
    #[serde(rename="Fail")]
    fail: Vec<String>,
}

#[test]
fn pass_fail() {
    let file = File::open("tests/data/passfail.yaml").expect("failed to open passfail yaml");
    let reader = BufReader::new(file);
    let tests: Tests = serde_yaml::from_reader(reader).expect("failed to parse passfail.yaml");
    let settings = RenderSettings::default();
    let mut fail = 0;

    for test in tests.pass {
        match SvgSink::new(&settings).render(&test) {
            Ok(_) => continue,
            Err(err) => {
                println!("Tex: {}", test);
                println!("Should have passed, failed with: {}", err);
                fail += 1;
            }
        }
    }

    // TODO: We need to stop panicking and pass the errors properly.
    // TODO: We need to use io::Write instead of fmt::Write for the
    //       rendering traits.
    //
    // for test in tests.fail {
    //     match SvgSink::new(&settings).render(&test) {
    //         Err(_) => continue,
    //         Ok(_) => {
    //             println!("Tex: {}", test);
    //             println!("Should have failed");
    //             fail += 1;
    //         }
    //     }
    // }

    if fail > 0 {
        panic!("{} Pass/Fail tests failed.", fail);
    }
}
