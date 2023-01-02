use crate::{Cons, HList};

use super::Pair;

/// Remove the first element from the heterogenous list.
///
/// New element will be removed from the beginning of the heterogenous list,
/// resulting in pair of new heterogenous list and removed element.
///
/// # Examples
///
/// ```
/// use hlist2::{hlist, ops::PopFront};
///
/// let hlist = hlist!(1, 2.0, true);
/// let (hlist, elem) = hlist.pop_front();
/// assert_eq!(hlist, hlist!(2.0, true));
/// assert_eq!(elem, 1);
/// ```
pub trait PopFront {
    /// Pair of new heterogenous list after removing the first element and removed element.
    type Output: Pair;

    /// Removes the first element from the heterogenous list.
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
