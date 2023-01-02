//! Provides operations defined for heterogenous lists.

pub use self::{
    append::Append, extend::Extend, fold::Fold, pop::Pop, pop_front::PopFront, prepend::Prepend,
    reverse::Reverse, rfold::RFold,
};

use self::pair::Pair;

mod append;
mod extend;
mod fold;
mod pair;
mod pop;
mod pop_front;
mod prepend;
mod reverse;
mod rfold;
