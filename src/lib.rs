//! This crate defines a way to create compile-time heterogenous lists,
//! or lists consisting of multiple types.
//!
//! This is done by defining types for empty list, [`Nil`],
//! and for pair of list head and its remainder, [`Cons`].
//! Heterogenous list consists of many conses contained recursively one in another,
//! and the last cons with the last element contains nil as the remainder.
//! For example, heterogenous list of integer, double and bool can be represented as
//! `Cons(1, Cons(2.0, Cons(true, Nil)))` with type of `Cons<i32, Cons<f64, Cons<bool, Nil>>>`.
//!
//! Such recursive nature of heterogenous list defined by this crate allows us to implement various traits recursively
//! and without any restrictions on the size of such list or types contained in it.
//! Unlike tuples, traits can be implemented for all heterogenous lists
//! and even for those which count of elements is bigger than 12, whish can be a problem sometimes.
//!
//! But such recursive nature can be a problem when we try to name the type of heterogenous list
//! or use pattern matching with values of heterogenous lists.
//! To simplify creation of lists and naming of list types the crate defines two macros,
//! [`hlist`] and [`hlist_type`].
//! The first one should be used for creation of heterogenous lists or for pattern matching,
//! while the second one should be used to name the type of heterogenous list.
//!
//! So instead of writing `Cons(1, Cons(2.0, Cons(true, Nil)))`
//! we can write more readable and tuple-like expression `hlist!(1, 2.0, true)`.
//! To name the type of such list, we can write
//! `hlist_type!(i32, f64, bool)` instead of `Cons<i32, Cons<f64, Cons<bool, Nil>>>`.
//!
//! This crate uses no unsafe code to provide the same safety guarantees provided by Rust programming language.

#![warn(clippy::all)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

/// An empty heterogenous list.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub struct Nil;

/// Heterogenous list with head and tail values, where tail is another heterogenous list.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub struct Cons<Head, Tail>(pub Head, pub Tail)
where
    Tail: HList;

/// Compile-time heterogenous list.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// It is implemented only for [`Cons`] and [`Nil`] structs.
pub trait HList: sealed::Sealed {}

impl HList for Nil {}

impl<Head, Tail> HList for Cons<Head, Tail> where Tail: HList {}

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::Nil {}

    impl<Head, Tail> Sealed for super::Cons<Head, Tail> where Tail: super::HList {}
}

/// Macro creating heterogenous list values from list of expressions.
#[macro_export]
macro_rules! hlist {
    () => {
        $crate::Nil
    };
    // handling simple identifiers, limited patterns support
    ($head:ident $(,)?) => {
        $crate::Cons($head, Nil)
    };
    ($head:ident, $($tail:ident),* $(,)?) => {
        $crate::Cons($head, $crate::hlist!($($tail),*))
    };
    // handling complex expressions
    ($head:expr $(,)?) => {
        $crate::Cons($head, $crate::Nil)
    };
    ($head:expr, $($tail:expr),* $(,)?) => {
        $crate::Cons($head, $crate::hlist!($($tail),*))
    };
}

/// Macro creating heterogenous list types from list of element types.
#[macro_export]
macro_rules! hlist_type {
    () => {
        $crate::Nil
    };
    ($head:ty $(,)?) => {
        $crate::Cons<$head, $crate::Nil>
    };
    ($head:ty, $($tail:ty),* $(,)?) => {
        $crate::Cons<$head, $crate::hlist_type!($($tail),*)>
    };
}
