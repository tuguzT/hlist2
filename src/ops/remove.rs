use crate::{Cons, HList};

use super::{Get, Here, Prepend, There};

/// Move element out of the heterogenous list by type.
pub trait Remove<T, I>: Get<T, I> {
    /// Remaining part of the heterogenous list without a removed element.
    type Remainder: HList;

    /// Moves element out of the heterogenous list by type.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, ops::Remove};
    ///
    /// let list = hlist![0_i32, 1_i64, false];
    /// let (a, remainder): (i64, _) = list.remove();
    /// assert_eq!(a, 1);
    /// assert_eq!(remainder, hlist![0, false]);
    /// ```
    fn remove(self) -> (T, Self::Remainder);
}

impl<Head, Tail> Remove<Head, Here> for Cons<Head, Tail>
where
    Tail: HList,
{
    type Remainder = Tail;

    fn remove(self) -> (Head, Self::Remainder) {
        let Cons(head, tail) = self;
        (head, tail)
    }
}

impl<Head, Tail, FromTail, TailIndex> Remove<FromTail, There<TailIndex>> for Cons<Head, Tail>
where
    Tail: Remove<FromTail, TailIndex>,
    Tail::Remainder: Prepend,
{
    type Remainder = <Tail::Remainder as Prepend>::Output<Head>;

    fn remove(self) -> (FromTail, Self::Remainder) {
        let Cons(head, tail) = self;
        let (removed, tail) = tail.remove();
        let remainder = tail.prepend(head);
        (removed, remainder)
    }
}
