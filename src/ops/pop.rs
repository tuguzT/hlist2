use crate::{Cons, HList, Nil};

use super::{Pair, Prepend};

/// Remove the last element from the heterogenous list.
pub trait Pop: HList {
    /// Pair of new heterogenous list after removing the last element and removed element.
    type Output: Pair;

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
    /// let list = hlist!(1, 2.0, true);
    /// let (list, elem) = list.pop();
    /// assert_eq!(list, hlist!(1, 2.0));
    /// assert_eq!(elem, true);
    /// ```
    fn pop(self) -> Self::Output;
}

impl<Head> Pop for Cons<Head, Nil> {
    type Output = (Nil, Head);

    fn pop(self) -> Self::Output {
        let Cons(head, nil) = self;
        (nil, head)
    }
}

impl<Head, Tail> Pop for Cons<Head, Tail>
where
    Tail: Pop,
    <Tail::Output as Pair>::First: Prepend,
{
    type Output = (
        <<Tail::Output as Pair>::First as Prepend>::Output<Head>,
        <Tail::Output as Pair>::Second,
    );

    fn pop(self) -> Self::Output {
        let Cons(head, tail) = self;
        let (list, elem) = tail.pop().destruct();
        let list = list.prepend(head);
        (list, elem)
    }
}
