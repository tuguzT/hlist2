use crate::{Cons, HList};

use super::{Here, Index, Prepend, There};

/// Provide element of the heterogenous list by type.
pub trait Provide<T, I>: HList
where
    I: Index,
{
    /// Provides a reference to the element of the heterogenous list by type.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, ops::Provide};
    ///
    /// let list = hlist![0_i32, 1_i64];
    /// let a: i64 = *list.provide();
    /// assert_eq!(a, 1);
    /// ```
    fn provide(&self) -> &T;

    /// Provides a mutable reference to the element of the heterogenous list by type.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, ops::Provide};
    ///
    /// let mut list = hlist![0_i32, 1_i64];
    /// *list.provide_mut() = 5_i32;
    /// let a: i32 = *list.provide();
    /// assert_eq!(a, 5);
    /// ```
    fn provide_mut(&mut self) -> &mut T;
}

/// Desired type is located in the head of the heterogenous list.
impl<Head, Tail> Provide<Head, Here> for Cons<Head, Tail>
where
    Tail: HList + ?Sized,
{
    fn provide(&self) -> &Head {
        let Cons(head, _) = self;
        head
    }

    fn provide_mut(&mut self) -> &mut Head {
        let Cons(head, _) = self;
        head
    }
}

/// Desired type is located somewhere in the tail of the heterogenous list.
impl<Head, Tail, FromTail, TailIndex> Provide<FromTail, There<TailIndex>> for Cons<Head, Tail>
where
    Tail: Provide<FromTail, TailIndex> + ?Sized,
    TailIndex: Index,
{
    fn provide(&self) -> &FromTail {
        let Cons(_, tail) = self;
        tail.provide()
    }

    fn provide_mut(&mut self) -> &mut FromTail {
        let Cons(_, tail) = self;
        tail.provide_mut()
    }
}

/// Move element out of the heterogenous list by type.
pub trait ProvideOnce<T, I>: Provide<T, I>
where
    I: Index,
{
    /// Remaining part of the heterogenous list without a removed element.
    type Remainder: HList;

    /// Moves element out of the heterogenous list by type.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, ops::ProvideOnce};
    ///
    /// let list = hlist![0_i32, 1_i64, false];
    /// let (a, remainder): (i64, _) = list.provide_once();
    /// assert_eq!(a, 1);
    /// assert_eq!(remainder, hlist![0, false]);
    /// ```
    fn provide_once(self) -> (T, Self::Remainder);
}

impl<Head, Tail> ProvideOnce<Head, Here> for Cons<Head, Tail>
where
    Tail: HList,
{
    type Remainder = Tail;

    fn provide_once(self) -> (Head, Self::Remainder) {
        let Cons(head, tail) = self;
        (head, tail)
    }
}

impl<Head, Tail, FromTail, TailIndex> ProvideOnce<FromTail, There<TailIndex>> for Cons<Head, Tail>
where
    Tail: ProvideOnce<FromTail, TailIndex>,
    TailIndex: Index,
    Tail::Remainder: Prepend,
{
    type Remainder = <Tail::Remainder as Prepend>::Output<Head>;

    fn provide_once(self) -> (FromTail, Self::Remainder) {
        let Cons(head, tail) = self;
        let (value, tail) = tail.provide_once();
        let remainder = tail.prepend(head);
        (value, remainder)
    }
}
