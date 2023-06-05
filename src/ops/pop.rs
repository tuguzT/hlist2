use crate::{Cons, HList, Nil};

use super::Prepend;

/// Remove the last element from the heterogenous list.
pub trait Pop: HList {
    /// The last element of the heterogenous list.
    type Last;
    /// Remaining part of the heterogenous list without the last element.
    type Remaining: HList;

    /// Removes the last element from the heterogenous list.
    ///
    /// New element will be removed at the end of the heterogenous list,
    /// resulting in pair of new heterogenous list and removed element.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, ops::Pop};
    ///
    /// let list = hlist![1, 2.0, true];
    /// let (elem, list) = list.pop();
    /// assert_eq!(list, hlist![1, 2.0]);
    /// assert_eq!(elem, true);
    /// ```
    fn pop(self) -> (Self::Last, Self::Remaining);
}

impl<Head> Pop for Cons<Head, Nil> {
    type Last = Head;
    type Remaining = Nil;

    fn pop(self) -> (Self::Last, Self::Remaining) {
        let Cons(head, nil) = self;
        (head, nil)
    }
}

impl<Head, Tail> Pop for Cons<Head, Tail>
where
    Tail: Pop,
    Tail::Remaining: Prepend,
{
    type Last = Tail::Last;
    type Remaining = <Tail::Remaining as Prepend>::Output<Head>;

    fn pop(self) -> (Self::Last, Self::Remaining) {
        let Cons(head, tail) = self;
        let (elem, list) = tail.pop();
        let list = list.prepend(head);
        (elem, list)
    }
}
