//! Provides operations defined for heterogenous lists.

pub use self::{
    append::Append,
    extend::Extend,
    fold::Fold,
    get::{Get, Here, There},
    map::Map,
    pop::Pop,
    pop_front::PopFront,
    prepend::Prepend,
    reverse::Reverse,
    rfold::RFold,
    to_ref::ToRef,
    unzip::Unzip,
    zip::Zip,
};

use self::pair::Pair;

mod append;
mod extend;
mod fold;
mod get;
mod map;
mod pair;
mod pop;
mod pop_front;
mod prepend;
mod reverse;
mod rfold;
mod to_ref;
mod unzip;
mod zip;
