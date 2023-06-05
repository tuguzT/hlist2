use crate::{Cons, HList};

/// Remove the first element from the heterogenous list.
pub trait PopFront: HList {
    /// The first element of the heterogenous list.
    type First;
    /// Remaining part of the heterogenous list without the first element.
    type Remaining: HList;

    /// Removes the first element from the heterogenous list.
    ///
    /// New element will be removed from the beginning of the heterogenous list,
    /// resulting in pair of new heterogenous list and removed element.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, ops::PopFront};
    ///
    /// let list = hlist![1, 2.0, true];
    /// let (elem, list) = list.pop_front();
    /// assert_eq!(elem, 1);
    /// assert_eq!(list, hlist![2.0, true]);
    /// ```
    fn pop_front(self) -> (Self::First, Self::Remaining);
}

impl<Head, Tail> PopFront for Cons<Head, Tail>
where
    Tail: HList,
{
    type First = Head;
    type Remaining = Tail;

    fn pop_front(self) -> (Self::First, Self::Remaining) {
        let Cons(head, tail) = self;
        (head, tail)
    }
}
