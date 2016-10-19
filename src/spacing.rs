// Spaces used for kerning between symbols
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ScaledPoint(u32);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum MathSpace {
  None,
  Thin,
  Medium,
  Thick,
}

enum AtomType {
    Ord,
    Op,
    Bin,
    Rel,
    Open,
    Close,
    Punct,
    Inner,
}

pub fn atom_spacing(left: MathSpace, right: MathSpace) -> MathSpace:: {
    match (left, right) {
        (AtomType::Ord,   AtomType::Op)    => MathSpace::Thin,
        (AtomType::Ord,   AtomType::Bin)   => MathSpace::Medium
        (AtomType::Ord,   AtomType::Rel)   => MathSpace::Thick
        (AtomType::Ord,   AtomType::Inner) => MathSpace::Thin,
        (AtomType::Op,    AtomType::Ord)   => MathSpace::Thin,
        (AtomType::Op,    AtomType::Op)    => MathSpace::Thin,
        (AtomType::Op,    AtomType::Rel)   => MathSpace::Thick,
        (AtomType::Op,    AtomType::Inner) => MathSpace::Thin,
        (AtomType::Bin,   AtomType::Ord)   => MathSpace::Medium,
        (AtomType::Bin,   AtomType::Op)    => MathSpace::Medium,
        (AtomType::Bin,   AtomType::Open)  => MathSpace::Medium,
        (AtomType::Bin,   AtomType::Inner) => MathSpace::Medium,
        (AtomType::Rel,   AtomType::Ord)   => MathSpace::Thick,
        (AtomType::Rel,   AtomType::Op)    => MathSpace::Thick,
        (AtomType::Rel,   AtomType::Open)  => MathSpace::Thick,
        (AtomType::Rel,   AtomType::Inner) => MathSpace::Thick,
        (AtomType::Close, AtomType::Op)    => MathSpace::Thin,
        (AtomType::Close, AtomType::Bin)   => MathSpace::Medium,
        (AtomType::Close, AtomType::Rel)   => MathSpace::Thick,
        (AtomType::Close, AtomType::Inner) => MathSpace::Thin,
        
        // Here it is better to list everything but MathSpace::Thin
        (AtomType::Inner, AtomType::Bin)   => MathSpace::Medium,
        (AtomType::Inner, AtomType::Rel)   => MathSpace::Thick,
        (AtomType::Inner, AtomType::Close) => MathSpace::None,
        (AtomType::Inner, _)               => MathSpace::Thin,

        // Every valid (punct, _) pair is undefined or Thin
        (AtomType::Punct, _)               => MathSpace::Thin,
        _ => MathSpace::None,
    }
}

pub fn scripts_atom_spacing(left: AtomType, right: AtomType) -> MathSpace {
    match (left, right) {
        (AtomType::Ord,   AtomType::Op) -> MathSpace::Thin,
        (AtomType::Op,    AtomType::Op) -> MathSpace::Thin,
        (AtomType::Close, AtomType::Op) -> MathSpace::Thin,
        (AtomType::Inner, AtomType::Op) -> MathSpace::Thin,
        _ -> MathSpace::None,
    }
}