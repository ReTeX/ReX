pub mod engine;
pub mod nodes;
pub mod atoms;

pub use self::engine::*;
pub use self::nodes::ParseNode;
pub use self::atoms::AtomType;

/// This struct contains many of the local variables that we will be passing
/// around while we are recursing.  It is designed to make it easier to
/// change variables while recursing, but keep the original variables
/// once we are done.
use font::fontselection::{ Family, Weight };

#[derive(Copy, Clone, Debug)]
pub struct Locals {
    family: Family,
    weight: Weight,
}

impl Locals {
    fn new() -> Locals {
        Locals {
            family: Family::Normal,
            weight: Weight::None,
        }
    }

    fn with_family(&self, fam: Family) -> Locals {
        Locals {
            weight: self.weight,
            family: fam,
        }
    }

    fn with_weight(&self, weight: Weight) -> Locals {
        Locals {
            family: self.family,
            weight: weight,
        }
    }
}