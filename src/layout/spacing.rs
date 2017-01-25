use parser::AtomType;
use dimensions::FontUnit;
use layout::Style;

#[allow(unused)]
pub fn atom_spacing(left: AtomType, right: AtomType, style: Style) -> Spacing {
    if style >= Style::TextCramped {
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
            (AtomType::Binary,      AtomType::Open)        => Spacing::Medium,
            (AtomType::Binary,      AtomType::Inner)       => Spacing::Medium,
            (AtomType::Relation,    AtomType::Alpha)       => Spacing::Thick,
            (AtomType::Relation,    AtomType::Operator(_)) => Spacing::Thick,
            (AtomType::Relation,    AtomType::Open)        => Spacing::Thick,
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
    } else {
        match (left, right) {
            (AtomType::Alpha, AtomType::Operator(_)) => Spacing::Thin,
            (AtomType::Operator(_), AtomType::Alpha) => Spacing::Thin,
            (AtomType::Operator(_), AtomType::Operator(_)) => Spacing::Thin,
            (AtomType::Close, AtomType::Operator(_)) => Spacing::Thin,
            (AtomType::Inner, AtomType::Operator(_)) => Spacing::Thin,
            _ => Spacing::None,
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Spacing {
  None,
  Thin,
  Medium,
  Thick,
}

impl Spacing {
    pub fn to_unit(self) -> FontUnit {
        match self {
            Spacing::None   => FontUnit::from(0),
            Spacing::Thin   => FontUnit::from(1) / 6,
            Spacing::Medium => FontUnit::from(2) / 9,
            Spacing::Thick  => FontUnit::from(3) / 9,
        }
    }
}