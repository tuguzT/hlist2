//! This crate defines a way to create compile-time heterogenous lists,
//! or lists consisting of multiple types.
//!
//! # Heterogenous lists
//!
//! This crate defines types for an empty list, [`Nil`],
//! and for pair of list head and its remainder, [`Cons`].
//! Heterogenous list consists of many conses contained recursively one in another,
//! and the last cons with the last element contains nil as the remainder.
//!
//! For example, heterogenous list of integer, double and bool can be represented as
//! `Cons(1, Cons(2.0, Cons(true, Nil)))` with type of `Cons<i32, Cons<f64, Cons<bool, Nil>>>`.
//!
//! # Recursive nature and behavior
//!
//! Such recursive nature of heterogenous list allows us to implement various traits recursively
//! and without any restrictions on the size of such list or types contained in it.
//! Unlike [tuples](prim@tuple), traits can be implemented for all heterogenous lists
//! and even for those which count of elements is bigger than 12, lack of which for tuples is a problem sometimes.
//!
//! All heterogenous lists implement [`HList`][hlist] trait, so it can be used in generics.
//! For example, this can be useful to bound generic type to be heterogenous list.
//!
//! To implement your trait for all heterogenous lists of any size,
//! first implement it on [`Nil`] type, which is [`HList`][hlist] too.
//! Then, implement your trait on [`Cons`] struct with head and tail generic types
//! where tail type is heterogenous list too (or which implement [`HList`][hlist] trait).
//!
//! Examples of these technique can be viewed in [`ops`] module, where
//! all the specific operations for all heterogenous list types are implemented.
//! For example, to append any value to the end of the list, use [`Append`][append] trait;
//! to prepend any value to the beginning of the list, use [`Prepend`][prepend] trait, and so on.
//!
//! [hlist]: trait@crate::HList
//! [append]: crate::ops::Append
//! [prepend]: crate::ops::Prepend
//!
//! # Constructing and destructing heterogenous lists
//!
//! But such recursive nature can be a problem when we try to name the type of heterogenous list
//! or use pattern matching with values of heterogenous lists.
//! To simplify creation of lists and naming of list types the crate defines two macros,
//! [`hlist!`] and [`HList!`].
//! The first one should be used for creation of heterogenous lists or for pattern matching,
//! while the second one should be used to name the type of heterogenous list.
//!
//! So instead of writing `Cons(1, Cons(2.0, Cons(true, Nil)))`
//! we can write more readable expression like `hlist![1, 2.0, true]`.
//!
//! To name the type of such list, we can write `HList![i32, f64, bool]`
//! instead of `Cons<i32, Cons<f64, Cons<bool, Nil>>>`.
//!
//! # Tuple compatibility
//!
//! Also this crate has a compatibility with [tuple](prim@tuple) types.
//! It implements conversion between heterogenous lists and their tuple forms
//! when tuple has length of 12 and less, and vise versa.
//!
//! # Features
//!
//! This crate uses **no unsafe code** to provide the same
//! safety guarantees the Rust programming language provides.
//!
//! This crate is `no_std`, so it can be used freely and with no fear in embedded environment.

#![warn(clippy::all)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

pub mod convert;
pub mod iter;
pub mod ops;

/// An empty heterogenous list.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub struct Nil;

/// Heterogenous list with head and tail values, where tail is another heterogenous list.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub struct Cons<Head, Tail>(pub Head, pub Tail)
where
    Tail: ?Sized;

/// Compile-time heterogenous list.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// It is implemented only for [`Cons`] and [`Nil`] structs.
pub trait HList: sealed::Sealed {
    /// Returns the length (count of elements) of the heterogenous list.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, HList};
    ///
    /// assert_eq!(hlist![].len(), 0);
    /// assert_eq!(hlist![1].len(), 1);
    /// assert_eq!(hlist![1, 2.0, true, "hello world"].len(), 4);
    /// ```
    fn len(&self) -> usize;

    /// Checks if the heterogenous list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::ops::Not;
    ///
    /// use hlist2::{hlist, HList};
    ///
    /// assert!(hlist![].is_empty());
    /// assert!(hlist![1, 2.0, true].is_empty().not());
    /// ```
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl HList for Nil {
    fn len(&self) -> usize {
        Self::LEN
    }

    fn is_empty(&self) -> bool {
        true
    }
}

impl<Head, Tail> HList for Cons<Head, Tail>
where
    Tail: HList + ?Sized,
{
    fn len(&self) -> usize {
        let Cons(_, tail) = self;
        1 + tail.len()
    }

    fn is_empty(&self) -> bool {
        false
    }
}

/// Heterogenous list with length (count of elements) known at compile-time.
pub trait Len: HList {
    /// Length (count of elements) of the heterogenous list.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{HList, Len};
    ///
    /// assert_eq!(<HList![]>::LEN, 0);
    /// assert_eq!(<HList![i32]>::LEN, 1);
    /// assert_eq!(<HList![i32, f64, bool, &str]>::LEN, 4);
    /// ```
    const LEN: usize;
}

impl Len for Nil {
    const LEN: usize = 0;
}

impl<Head, Tail> Len for Cons<Head, Tail>
where
    Tail: Len,
{
    const LEN: usize = 1 + Tail::LEN;
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for crate::Nil {}

    impl<Head, Tail> Sealed for crate::Cons<Head, Tail> where Tail: Sealed + ?Sized {}
}

/// Macro creating heterogenous list values from list of expressions.
///
/// This macro supports trailing comma at the end of list of expressions.
///
/// # Examples
///
/// This macro can be used to create heterogenous list
/// as easily as tuple without cons-nil boilerplate:
///
/// ```
/// use hlist2::{hlist, Cons, Nil};
///
/// let list = hlist![1, 2.0, true];
/// assert_eq!(list, Cons(1, Cons(2.0, Cons(true, Nil))));
/// ```
///
/// Also it can be used in pattern matching.
/// For example, we can destruct heterogenous list to its values:
///
/// ```
/// use hlist2::hlist;
///
/// let hlist![a, b, c, d] = hlist![10, -15.0, "hello world", false];
/// assert_eq!((a, b, c, d), (10, -15.0, "hello world", false));
/// ```
#[macro_export]
macro_rules! hlist {
    () => {
        $crate::Nil
    };
    // handling simple identifiers, limited patterns support
    ($head:ident $(,)?) => {
        $crate::Cons($head, $crate::hlist!())
    };
    ($head:ident, $($tail:ident),* $(,)?) => {
        $crate::Cons($head, $crate::hlist!($($tail),*))
    };
    // handling complex expressions
    ($head:expr $(,)?) => {
        $crate::Cons($head, $crate::hlist!())
    };
    ($head:expr, $($tail:expr),* $(,)?) => {
        $crate::Cons($head, $crate::hlist!($($tail),*))
    };
}

/// Macro creating heterogenous list types from list of element types.
///
/// This macro supports trailing comma at the end of list of element types.
///
/// # Examples
///
/// This macro can be used to name heterogenous list type
/// as easily as tuple type without cons-nil boilerplate:
///
/// ```
/// use hlist2::{hlist, HList, Cons, Nil};
///
/// let list: HList![i32, f64, bool] = hlist![1, 2.0, true];
/// let list: Cons<i32, Cons<f64, Cons<bool, Nil>>> = list;
/// ```
#[macro_export]
macro_rules! HList {
    () => {
        $crate::Nil
    };
    ($head:ty $(,)?) => {
        $crate::Cons<$head, $crate::HList!()>
    };
    ($head:ty, $($tail:ty),* $(,)?) => {
        $crate::Cons<$head, $crate::HList!($($tail),*)>
    };
}
