// Allow inception for now, until this can be refactored. It's because the parent
// and child modules (individual.Individual) have the same name.
#![allow(clippy::module_inception)]

mod birth;
mod death;
mod gender;
mod individual;
mod name;
mod note;
mod residence;
mod source;

pub use birth::*;
pub use death::Death;
pub use gender::*;
pub use individual::*;
pub use name::*;
pub use note::*;
pub use residence::Residence;
pub use source::*;
