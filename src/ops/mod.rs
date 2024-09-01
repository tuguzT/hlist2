//! Provides operations defined for heterogenous lists.

pub use self::{
    append::Append,
    extend::Extend,
    flatten::Flatten,
    fold::{Fold, FoldFn, Folder, RFold},
    get::Get,
    get_many::GetMany,
    index::{Dec, Here, Inc, Index, ManyIndex, There},
    map::{Map, MapFn, Mapper},
    pop::Pop,
    pop_front::PopFront,
    prepend::Prepend,
    remove::Remove,
    remove_many::RemoveMany,
    reverse::Reverse,
    shuffle::Shuffle,
    to_ref::ToRef,
    unzip::Unzip,
    zip::Zip,
};

mod append;
mod extend;
mod flatten;
mod fold;
mod get;
mod get_many;
mod index;
mod map;
mod pop;
mod pop_front;
mod prepend;
mod remove;
mod remove_many;
mod reverse;
mod shuffle;
mod to_ref;
mod unzip;
mod zip;
