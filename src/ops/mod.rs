//! Provides operations defined for heterogenous lists.

pub use self::{
    append::Append,
    extend::Extend,
    fold::{Fold, FoldFn, Folder, RFold},
    get::{Get, Here, There},
    map::Map,
    pop::Pop,
    pop_front::PopFront,
    prepend::Prepend,
    reverse::Reverse,
    to_ref::ToRef,
    unzip::Unzip,
    zip::Zip,
};

mod append;
mod extend;
mod fold;
mod get;
mod map;
mod pop;
mod pop_front;
mod prepend;
mod reverse;
mod to_ref;
mod unzip;
mod zip;
