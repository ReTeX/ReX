use super::dimensions::{ LayoutNode, HorizontalBox };
use parser::nodes::{ ParseNode, AtomType };
use font::GLYPHS;

/// This method takes the parsing nodes and reduces them to layout nodes.
#[allow(unconditional_recursion)]
#[allow(dead_code)]
pub fn reduce(nodes: &mut [ParseNode]) -> Vec<LayoutNode> {
    // Rule (5), pg 442.  If first item is a Bin atom, change it 
    // to an Ordinal item.
    use font::IsAtom;
    if let Some(mut node) = nodes.get_mut(0) {
        if node.atom_type() == Some(AtomType::Binary) {
            node.set_atom_type(AtomType::Ordinal)
        }
    }
    
    // Atom Changing Rules:
    //   Rule 5:
    //   - Current == Bin && Prev in {Bin,Op,Rel,Open,Punct}, Current -> Ord.
    //   Rule 6:
    //   - Current in {Rel,Close,Punct} && Prev == Bin => Prev -> Ord.
    for idx in 0..nodes.len() {
        if nodes[idx].atom_type() == Some(AtomType::Binary)
            && idx > 1 {
            match nodes[idx - 1].atom_type() {
                Some(AtomType::Binary) |
                Some(AtomType::Operator(_)) |
                Some(AtomType::Relation) |
                Some(AtomType::Open) |
                Some(AtomType::Punctuation) => {
                    nodes[idx].set_atom_type(AtomType::Alpha);
                    println!("Chaing binary to ordinal {:?}", nodes[idx]);
                },
                _ => (),
            }
        }

        if idx > 1
            && nodes[idx - 1].atom_type() == Some(AtomType::Binary) {
            match nodes[idx].atom_type() {
                Some(AtomType::Relation) |
                Some(AtomType::Close) |
                Some(AtomType::Punctuation) =>
                    nodes[idx - 1].set_atom_type(AtomType::Alpha),
                _ => (),
            }
        }
    }

    let mut prev_at: Option<AtomType> = None;
    let mut layout: Vec<LayoutNode> = Vec::with_capacity(nodes.len());
    for node in nodes {
        if let Some(p_at) = prev_at {
            if let Some(at) = node.atom_type() {
                use spacing::Spacing;
                use spacing::atom_spacing;
                let sp = atom_spacing(p_at, at);
                if sp != Spacing::None {
                    layout.push(LayoutNode::Space(sp));
                }
            }
        }

        // TODO: May need to ignore this if transparent atom_type.
        prev_at = node.atom_type();

        match *node {
            ParseNode::Symbol(sym) =>
                layout.push(LayoutNode::Glyph(GLYPHS[&sym.unicode].clone())),
            ParseNode::Spacing(sp) =>
                layout.push(LayoutNode::Space(sp)),
            ParseNode::Group(ref mut gp) =>
                layout.push(LayoutNode::HorizontalBox(HorizontalBox {
                    inner: reduce(&mut gp.clone()),
                })),
            ParseNode::Radical(ref rad) => {
                // Reference rule 11 from pg 443 of TeXBook
                // use font::SYMBOLS;
                // use font::CONSTANTS;

                // let rad_sym = &GLYPHS[&SYMBOLS["sqrt"].unicode];

                // // TODO: Change style to C'
                // // TODO: Select radical symbol large enough
                // layout.push(LayoutNode::VerticalBox(VerticalBox {
                //     inner: vec![
                //         LayoutNode::Rule(Rule {
                //             height: 
                //         })
                //     ]
                // }))

                unimplemented!()
            },
            _ => (),
       }
    }
    
    layout
}