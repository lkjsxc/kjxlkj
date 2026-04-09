//! Documentation validation commands

mod links;
mod output;
mod terms;
mod topology;

pub use links::validate_links;
pub use terms::validate_terms;
pub use topology::validate_topology;
