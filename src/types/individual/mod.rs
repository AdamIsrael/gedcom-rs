// Allow inception for now, until this can be refactored. It's because the parent
// and child modules (individual.Individual) have the same name.
#![allow(clippy::module_inception)]

mod adoption;
mod birth;
mod christening;
mod death;
mod event;
mod gender;
mod individual;
mod name;
mod note;
mod residence;
mod source;

pub use adoption::Adoption;
pub use birth::Birth;
pub use christening::Christening;
pub use death::Death;
pub use event::IndividualEventDetail;
pub use gender::*;
pub use individual::*;
pub use name::*;
pub use note::*;
pub use residence::Residence;
// pub use source::*;
