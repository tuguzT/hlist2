//! Provides operations defined for heterogenous lists.

pub use self::{
    append::Append, extend::Extend, pop::Pop, pop_front::PopFront, prepend::Prepend,
    reverse::Reverse,
};

use self::pair::Pair;

mod append;
mod extend;
mod pair;
mod pop;
mod pop_front;
mod prepend;
mod reverse;
