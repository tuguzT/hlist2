use super::{Here, There};

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

    impl<T> Sealed for super::There<T> where T: super::Index {}
}

/// Type of index which can be incremented,
/// resulting in an index with can be represented as value of `Self + 1`.
pub trait Inc: Index {
    /// Result type of incrementing operation.
    type Output: Dec;

    /// Increments the index,
    /// resulting in an index with can be represented as value of `Self + 1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::ops::{Here, Inc, There};
    ///
    /// let zero = Here;
    /// let one: There<Here> = zero.inc();
    /// let two: There<There<Here>> = one.inc();
    /// ```
    fn inc(self) -> Self::Output;
}

impl<T> Inc for T
where
    T: Index,
{
    type Output = There<T>;

    fn inc(self) -> Self::Output {
        There::new()
    }
}

/// Type of index which can be decremented,
/// resulting in an index with can be represented as value of `Self - 1`.
pub trait Dec: Index {
    /// Result type of decrementing operation.
    type Output: Inc;

    /// Decrements the index,
    /// resulting in an index with can be represented as value of `Self - 1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::ops::{Dec, Here, There};
    ///
    /// let two: There<There<Here>> = There::new();
    /// let one: There<Here> = two.dec();
    /// let zero: Here = one.dec();
    /// ```
    ///
    /// However, decrementing cannot be used to create negative indices
    /// (because there is no such indices):
    ///
    /// ```compile_fail
    /// # use hlist2::ops::{Dec, Here};
    /// # let zero = Here;
    /// let minus_one = zero.dec();
    /// ```
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
