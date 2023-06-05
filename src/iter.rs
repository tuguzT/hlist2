//! Defines iteration capabilities over heterogenous lists.
//!
//! Heterogenous list can be iterated if all elements of the list are of the same type.
//!
//! For example, we can iterate by value:
//!
//! ```
//! use hlist2::hlist;
//!
//! let list = hlist![1, 2, 3, 4, 5];
//! for item in list {
//!     println!("item is {item}")
//! }
//! ```
//!
//! Over references:
//!
//! ```
//! use hlist2::hlist;
//!
//! let list = hlist![1, 2, 3, 4, 5];
//! for item in &list {
//!     println!("item is {item}")
//! }
//! assert_eq!(list, hlist![1, 2, 3, 4, 5])
//! ```
//!
//! Or even over mutable references:
//!
//! ```
//! use hlist2::hlist;
//!
//! let mut list = hlist![1, 2, 3, 4, 5];
//! for item in &mut list {
//!     *item += 10;
//!     println!("item is {item}")
//! }
//! assert_eq!(list, hlist![11, 12, 13, 14, 15])
//! ```

use core::iter::FusedIterator;

use crate::{ops::ToRef, Cons, Nil};

use self::impl_details::{PrepareIter, ReadyIter};

/// An iterator that moves out of a heterogenous list.
///
/// # Examples
///
/// ```
/// use hlist2::hlist;
///
/// let mut iter = hlist![1, 2, 3, 4, 5].into_iter();
/// assert_eq!(iter.len(), 5);
///
/// let item = iter.next();
/// assert_eq!(item, Some(1));
/// assert_eq!(iter.len(), 4);
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct IntoIter<T>
where
    T: PrepareIter,
{
    prepared: T::Output,
}

impl<T> Iterator for IntoIter<T>
where
    T: PrepareIter,
{
    type Item = <T::Output as ReadyIter>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { prepared } = self;
        prepared.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for IntoIter<T>
where
    T: PrepareIter,
{
    fn len(&self) -> usize {
        let Self { prepared } = self;
        prepared.len()
    }
}

impl<T> FusedIterator for IntoIter<T> where T: PrepareIter {}

impl<Head, Tail> IntoIterator for Cons<Head, Tail>
where
    Self: PrepareIter,
    <Self as PrepareIter>::Output: ReadyIter<Item = Head>,
{
    type Item = Head;
    type IntoIter = IntoIter<Self>;

    fn into_iter(self) -> Self::IntoIter {
        let prepared = self.prepare_iter();
        IntoIter { prepared }
    }
}

impl<'a, Head, Tail> IntoIterator for &'a Cons<Head, Tail>
where
    Cons<Head, Tail>: ToRef,
    <Cons<Head, Tail> as ToRef>::Ref<'a>: PrepareIter,
    <<Cons<Head, Tail> as ToRef>::Ref<'a> as PrepareIter>::Output: ReadyIter<Item = &'a Head>,
{
    type Item = &'a Head;
    type IntoIter = IntoIter<<Cons<Head, Tail> as ToRef>::Ref<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        let prepared = self.to_ref();
        let prepared = prepared.prepare_iter();
        IntoIter { prepared }
    }
}

impl<'a, Head, Tail> IntoIterator for &'a mut Cons<Head, Tail>
where
    Cons<Head, Tail>: ToRef,
    <Cons<Head, Tail> as ToRef>::RefMut<'a>: PrepareIter,
    <<Cons<Head, Tail> as ToRef>::RefMut<'a> as PrepareIter>::Output:
        ReadyIter<Item = &'a mut Head>,
{
    type Item = &'a mut Head;
    type IntoIter = IntoIter<<Cons<Head, Tail> as ToRef>::RefMut<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        let prepared = self.to_mut();
        let prepared = prepared.prepare_iter();
        IntoIter { prepared }
    }
}

impl<A> FromIterator<A> for Nil {
    /// Creates an empty heterogenous list from an empty iterator.
    ///
    /// # Panics
    ///
    /// This function will panic if input iterator was not empty,
    /// or if there is no space for elements from the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::hlist;
    ///
    /// let hlist!() = core::iter::empty::<i32>().collect();
    /// ```
    ///
    /// Again, this will panic if iterator is not empty:
    ///
    /// ```should_panic
    /// use hlist2::hlist;
    ///
    /// let hlist!() = [0; 5].into_iter().collect();
    /// ```
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>,
    {
        let mut iter = iter.into_iter();
        if iter.next().is_some() {
            panic!("too many elements in the iterator")
        }
        Nil
    }
}

impl<Head, Tail> FromIterator<Head> for Cons<Head, Tail>
where
    Tail: FromIterator<Head>,
{
    /// Creates a new heterogenous list from an input iterator.
    ///
    /// Iterator must contain the same count of elements as the target list type.
    ///
    /// # Panics
    ///
    /// This function will panic if there is not enough elements in the iterator
    /// or if there is no space for elements from the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::hlist;
    ///
    /// let hlist!(a, b, c, d, e) = [42; 5].into_iter().collect();
    /// ```
    ///
    /// If iterator is too long, this call will panic:
    ///
    /// ```should_panic
    /// use hlist2::hlist;
    ///
    /// let hlist!(a, b, c, d, e) = [42; 10].into_iter().collect();
    /// ```
    ///
    /// If iterator is too short, this also will panic:
    ///
    /// ```should_panic
    /// use hlist2::hlist;
    ///
    /// let hlist!(a, b, c, d, e) = [42; 1].into_iter().collect();
    /// ```
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Head>,
    {
        let mut iter = iter.into_iter();
        let head = iter.next().expect("not enough elements in the iterator");
        let from_iter = iter.collect();
        Cons(head, from_iter)
    }
}

mod impl_details {
    use crate::{Cons, HList, Nil};

    pub trait PrepareIter: HList {
        type Output: ReadyIter;

        fn prepare_iter(self) -> Self::Output;
    }

    impl<Head> PrepareIter for Cons<Head, Nil> {
        type Output = Cons<Option<Head>, Nil>;

        fn prepare_iter(self) -> Self::Output {
            let Cons(head, tail) = self;
            let head = Some(head);
            Cons(head, tail)
        }
    }

    impl<Head, Tail> PrepareIter for Cons<Head, Tail>
    where
        Tail: PrepareIter,
        Tail::Output: ReadyIter<Item = Head>,
    {
        type Output = Cons<Option<Head>, Tail::Output>;

        fn prepare_iter(self) -> Self::Output {
            let Cons(head, tail) = self;
            let head = Some(head);
            let tail = tail.prepare_iter();
            Cons(head, tail)
        }
    }

    pub trait ReadyIter: HList {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;

        fn len(&self) -> usize;
    }

    impl<Head> ReadyIter for Cons<Option<Head>, Nil> {
        type Item = Head;

        fn next(&mut self) -> Option<Self::Item> {
            let Cons(head, _) = self;
            head.take()
        }

        fn len(&self) -> usize {
            let Cons(head, _) = self;
            head.is_some() as usize
        }
    }

    impl<Head, Tail> ReadyIter for Cons<Option<Head>, Tail>
    where
        Tail: ReadyIter<Item = Head>,
    {
        type Item = Head;

        fn next(&mut self) -> Option<Self::Item> {
            let Cons(head, tail) = self;
            match head.take() {
                Some(item) => Some(item),
                None => tail.next(),
            }
        }

        fn len(&self) -> usize {
            let Cons(head, tail) = self;
            let head = head.is_some() as usize;
            let tail = ReadyIter::len(tail);
            head + tail
        }
    }
}
