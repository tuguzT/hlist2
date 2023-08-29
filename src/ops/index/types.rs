use core::{any::type_name, fmt::Debug, hash::Hash, marker::PhantomData};

/// Used as an index into an [`HList`](trait@crate::HList) which points to the head of the heterogenous list.
///
/// This index can be represented as value of `0`, which points to the head of the heterogenous list.
///
/// This type of index exists due to lack of specialization in Rust.
/// This allows multiple implementations of index traits based on knowledge
/// where the actual type is located: in the head or somewhere in the tail of the list.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Here;

/// Used as an index into an [`HList`](trait@crate::HList) which points to the tail of the heterogenous list.
///
/// This index can be represented as value of `T + 1`, which points somewhere to the tail of the heterogenous list.
///
/// This type of index exists due to lack of specialization in Rust.
/// This allows multiple implementations of index traits based on knowledge
/// where the actual type is located: in the head or somewhere in the tail of the list.
pub struct There<T> {
    phantom: PhantomData<fn() -> T>,
}

impl<T> There<T> {
    /// Creates new index which can be represented as value of `T + 1`.
    pub const fn new() -> Self {
        let phantom = PhantomData;
        Self { phantom }
    }
}

impl<T> Debug for There<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let type_name = type_name::<T>();
        write!(f, "There<{type_name}>")
    }
}

impl<T> Default for There<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for There<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for There<T> {}

impl<T> PartialEq for There<T> {
    fn eq(&self, other: &Self) -> bool {
        let Self { phantom: this } = self;
        let Self { phantom: other } = other;
        this == other
    }
}

impl<T> Eq for There<T> {}

impl<T> PartialOrd for There<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        let Self { phantom: this } = self;
        let Self { phantom: other } = other;
        this.partial_cmp(other)
    }
}

impl<T> Ord for There<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let Self { phantom: this } = self;
        let Self { phantom: other } = other;
        this.cmp(other)
    }
}

impl<T> Hash for There<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let Self { phantom } = self;
        phantom.hash(state)
    }
}
