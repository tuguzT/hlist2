use crate::{Cons, HList};

use super::Pair;

/// Remove the first element from the heterogenous list.
pub trait PopFront {
    /// Pair of new heterogenous list after removing the first element and removed element.
    type Output: Pair;

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
    /// let list = hlist!(1, 2.0, true);
    /// let (list, elem) = list.pop_front();
    /// assert_eq!(list, hlist!(2.0, true));
    /// assert_eq!(elem, 1);
    /// ```
    fn pop_front(self) -> Self::Output;
}

impl<Head, Tail> PopFront for Cons<Head, Tail>
where
    Tail: HList,
{
    type Output = (Tail, Head);

    fn pop_front(self) -> Self::Output {
        let Cons(head, tail) = self;
        (tail, head)
    }
}
