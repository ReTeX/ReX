// Spaces used for kerning between symbols
use parser::nodes::AtomType;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MathSpace {
  None,
  Thin,
  Medium,
  Thick,
}

#[allow(unused)]
impl MathSpace {
    pub fn to_em(self) -> f64 {
        match self {
            MathSpace::None => 0_f64,
            MathSpace::Thin => 1_f64/6_f64,
            MathSpace::Medium => 2_f64/9_f64,
            MathSpace::Thick => 3_f64/9_f64,
        }
    }
}

#[allow(unused)]
pub fn atom_spacing(left: AtomType, right: AtomType) -> MathSpace {
    match (left, right) {
        (AtomType::Alpha,   AtomType::Operator(_))    => MathSpace::Thin,
        (AtomType::Alpha,   AtomType::Binary)   => MathSpace::Medium,
        (AtomType::Alpha,   AtomType::Relation)   => MathSpace::Thick,
        (AtomType::Alpha,   AtomType::Inner) => MathSpace::Thin,
        (AtomType::Operator(_),    AtomType::Alpha)   => MathSpace::Thin,
        (AtomType::Operator(_),    AtomType::Operator(_))    => MathSpace::Thin,
        (AtomType::Operator(_),    AtomType::Relation)   => MathSpace::Thick,
        (AtomType::Operator(_),    AtomType::Inner) => MathSpace::Thin,
        (AtomType::Binary,   AtomType::Alpha)   => MathSpace::Medium,
        (AtomType::Binary,   AtomType::Operator(_))    => MathSpace::Medium,
        (AtomType::Binary,   AtomType::Inner) => MathSpace::Medium,
        (AtomType::Relation,   AtomType::Alpha)   => MathSpace::Thick,
        (AtomType::Relation,   AtomType::Operator(_))    => MathSpace::Thick,
        (AtomType::Relation,   AtomType::Inner) => MathSpace::Thick,
        (AtomType::Close, AtomType::Operator(_))    => MathSpace::Thin,
        (AtomType::Close, AtomType::Binary)   => MathSpace::Medium,
        (AtomType::Close, AtomType::Relation)   => MathSpace::Thick,
        (AtomType::Close, AtomType::Inner) => MathSpace::Thin,
        
        // Here it is better to list everything but MathSpace::Thin
        (AtomType::Inner, AtomType::Binary)   => MathSpace::Medium,
        (AtomType::Inner, AtomType::Relation)   => MathSpace::Thick,
        (AtomType::Inner, AtomType::Close) => MathSpace::None,
        (AtomType::Inner, _)               => MathSpace::Thin,

        // Every valid (punct, _) pair is undefined or Thin
        (AtomType::Punctuation, _)               => MathSpace::Thin,
        _ => MathSpace::None,
    }
}
