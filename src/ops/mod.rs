//! Provides operations defined for heterogenous lists.

pub use self::{append::Append, extend::Extend, prepend::Prepend, reverse::Reverse};

mod append;
mod extend;
mod prepend;
mod reverse;
