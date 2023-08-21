use core::{
    any::type_name,
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Add, Sub},
};

/// Used as an index into an [`HList`](trait@crate::HList) which points to the head of the heterogenous list.
///
/// This index can be represented as value of `0`, which points to the head of the heterogenous list.
///
/// This type of index exists due to lack of specialization in Rust.
/// This allows multiple implementations of [`Get`](super::Get) trait based on knowledge
/// where the actual type is located: in the head or somewhere in the tail of the list.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Here;

/// Used as an index into an [`HList`](trait@crate::HList) which points to the tail of the heterogenous list.
///
/// This index can be represented as value of `T + 1`, which points somewhere to the tail of the heterogenous list.
///
/// This type of index exists due to lack of specialization in Rust.
/// This allows multiple implementations of [`Get`](super::Get) trait based on knowledge
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

/// Type of index which is used in all indexing operations of the crate.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// It is implemented only for [`Here`] and [`There`] structs.
pub trait Index: Default + sealed::Sealed {}

impl Index for Here {}

impl<T> Index for There<T> where T: Index {}

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::Here {}

    impl<T> Sealed for super::There<T> where T: Sealed {}
}

/// Type of index which can be incremented,
/// resulting in an index with can be represented as value of `Self + 1`.
pub trait Inc: Index {
    /// Result type of incrementing operation.
    type Output: Dec;

    /// Increments the index,
    /// resulting in an index with can be represented as value of `Self + 1`.
    fn inc(self) -> Self::Output;
}

impl<T> Inc for T
where
    T: Index,
{
    type Output = There<T>;

    fn inc(self) -> Self::Output {
        Default::default()
    }
}

/// Type of index which can be decremented,
/// resulting in an index with can be represented as value of `Self - 1`.
pub trait Dec: Index {
    /// Result type of decrementing operation.
    type Output: Inc;

    /// Decrements the index,
    /// resulting in an index with can be represented as value of `Self - 1`.
    fn dec(self) -> Self::Output;
}

impl<T> Dec for There<T>
where
    T: Index,
{
    type Output = T;

    fn dec(self) -> Self::Output {
        Default::default()
    }
}

impl<T> Add<T> for Here
where
    T: Index,
{
    type Output = T;

    fn add(self, rhs: T) -> Self::Output {
        rhs
    }
}

impl<T, U> Add<T> for There<U>
where
    U: Add<T>,
{
    type Output = There<U::Output>;

    fn add(self, _: T) -> Self::Output {
        Default::default()
    }
}

impl Sub<Here> for Here {
    type Output = Here;

    fn sub(self, rhs: Here) -> Self::Output {
        rhs
    }
}

impl<T> Sub<Here> for There<T>
where
    T: Index,
{
    type Output = There<T>;

    fn sub(self, _: Here) -> Self::Output {
        Default::default()
    }
}

impl<T, U> Sub<There<T>> for There<U>
where
    T: Index,
    U: Index + Sub<T>,
{
    type Output = U::Output;

    fn sub(self, rhs: There<T>) -> Self::Output {
        self.dec() - rhs.dec()
    }
}
