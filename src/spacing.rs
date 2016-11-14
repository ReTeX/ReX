// Spaces used for kerning between symbols
use parser::nodes::AtomType;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Spacing {
  None,
  Thin,
  Medium,
  Thick,
}



#[allow(unused)]
pub fn atom_spacing(left: AtomType, right: AtomType) -> Spacing {
    match (left, right) {
        (AtomType::Alpha,       AtomType::Operator(_)) => Spacing::Thin,
        (AtomType::Alpha,       AtomType::Binary)      => Spacing::Medium,
        (AtomType::Alpha,       AtomType::Relation)    => Spacing::Thick,
        (AtomType::Alpha,       AtomType::Inner)       => Spacing::Thin,
        (AtomType::Operator(_), AtomType::Alpha)       => Spacing::Thin,
        (AtomType::Operator(_), AtomType::Operator(_)) => Spacing::Thin,
        (AtomType::Operator(_), AtomType::Relation)    => Spacing::Thick,
        (AtomType::Operator(_), AtomType::Inner)       => Spacing::Thin,
        (AtomType::Binary,      AtomType::Alpha)       => Spacing::Medium,
        (AtomType::Binary,      AtomType::Operator(_)) => Spacing::Medium,
        (AtomType::Binary,      AtomType::Inner)       => Spacing::Medium,
        (AtomType::Relation,    AtomType::Alpha)       => Spacing::Thick,
        (AtomType::Relation,    AtomType::Operator(_)) => Spacing::Thick,
        (AtomType::Relation,    AtomType::Inner)       => Spacing::Thick,
        (AtomType::Close,       AtomType::Operator(_)) => Spacing::Thin,
        (AtomType::Close,       AtomType::Binary)      => Spacing::Medium,
        (AtomType::Close,       AtomType::Relation)    => Spacing::Thick,
        (AtomType::Close,       AtomType::Inner)       => Spacing::Thin,
        
        // Here it is better to list everything but Spacing::Thin
        (AtomType::Inner, AtomType::Binary)   => Spacing::Medium,
        (AtomType::Inner, AtomType::Relation) => Spacing::Thick,
        (AtomType::Inner, AtomType::Close)    => Spacing::None,
        (AtomType::Inner, _)                  => Spacing::Thin,

        // Every valid (punct, _) pair is undefined or Thin
        (AtomType::Punctuation, _) => Spacing::Thin,
        _ => Spacing::None,
    }
}

// Atom Changing Rules:
//   Rule 5:
//   - If first item is a Bin, change current to Ord.
//   - Current == Bin && Prev in {Bin,Op,Rel,Open,Punct}, Current -> Ord.
//   Rule 6:
//   - Current in {Rel,Close,Punct} && Prev == Bin => Prev -> Ord.