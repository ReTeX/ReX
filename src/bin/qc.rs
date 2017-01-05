use std::env;

extern crate rex;

// let svg = rex::SVGRenderer::new();
//
// let svg = rex::SVGRenderer::new()
//      .style(Style::Display)
//      .font_size(48)
//      .horz_padding(12)
//      .vert_padding(12)
//      .strict(true)
//      .gzip(true);
//
// // Redner to file
// let _ = svg.render_to_file("temp.svg", r"\frac{-b \pm \sqrt{b^2 - 4ac}}{2a}");
//
// // Render to String
// let result = svg.render(r"\frac{-b \pm \sqrt{b^2 - 4ac}}{2a}")
//      .expect("Unable render svg!");
//

fn main() {
    let input = env::args().skip(1).collect::<String>();
    if input.len() == 0 {
        println!("Provide a TeX argument");
        return
    }

    let svg = rex::SVGRenderer::new().font_size(96.0).debug(false);
    svg.render_to_file("test.svg", &input);
}