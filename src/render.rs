use parser::nodes::{ ParseNode };
use symbols::Symbol;
use std::char;

pub fn render(nodes: Vec<ParseNode>) {
    use svg;    
    use svg::Document;
    use svg::node::Text;
    use svg::node::element::Path;
    use svg::node::element::path::Data;

    let mut content = String::with_capacity(nodes.len());

    for node in nodes {
        match node {
            ParseNode::Symbol(sym) => content.push(char::from_u32(sym.code).unwrap()),
            _ => continue,
        }
    }

    let text = Text::new(content);
    let document = Document::new()
                            .set("viewBox", (0, 0, 70, 70))
                            .add(text);

    svg::save("image.svg", &document).unwrap();
}