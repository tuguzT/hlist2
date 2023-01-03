//! Provides operations defined for heterogenous lists.

pub use self::{
    append::Append, extend::Extend, fold::Fold, map::Map, pop::Pop, pop_front::PopFront,
    prepend::Prepend, reverse::Reverse, rfold::RFold, to_mut::ToMut, to_ref::ToRef, zip::Zip,
};

use self::pair::Pair;

mod append;
mod extend;
mod fold;
mod map;
mod pair;
mod pop;
mod pop_front;
mod prepend;
mod reverse;
mod rfold;
mod to_mut;
mod to_ref;
mod zip;
