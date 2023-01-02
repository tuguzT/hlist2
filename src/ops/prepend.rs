use crate::{Cons, HList};

/// Prepend new element to the heterogenous list.
pub trait Prepend: HList {
    /// Type of heterogenous list with new element.
    type Output<T>: HList;

    /// Prepends new element to the heterogenous list.
    ///
    /// New element will be placed at the beginning of the heterogenous list,
    /// resulting in new heterogenous list.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, ops::Prepend};
    ///
    /// let list = hlist!(2.0, true, "hello world");
    /// assert_eq!(list.prepend(1), hlist!(1, 2.0, true, "hello world"));
    /// ```
    fn prepend<T>(self, value: T) -> Self::Output<T>;
}

impl<L> Prepend for L
where
    L: HList,
{
    type Output<T> = Cons<T, L>;

    fn prepend<T>(self, value: T) -> Self::Output<T> {
        Cons(value, self)
    }
}
