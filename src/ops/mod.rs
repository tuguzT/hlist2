//! Provides operations defined for heterogenous lists.

pub use self::{
    append::Append,
    extend::Extend,
    fold::{Fold, FoldFn, Folder, RFold},
    get::Get,
    index::{Here, There},
    map::{Map, MapFn, Mapper},
    pop::Pop,
    pop_front::PopFront,
    prepend::Prepend,
    remove::Remove,
    reverse::Reverse,
    to_ref::ToRef,
    unzip::Unzip,
    zip::Zip,
};

mod append;
mod extend;
mod fold;
mod get;
mod index;
mod map;
mod pop;
mod pop_front;
mod prepend;
mod remove;
mod reverse;
mod to_ref;
mod unzip;
mod zip;
