use crate::{Cons, HList, Nil};

/// Extend heterogenous list with another heterogenous list.
pub trait Extend: HList {
    /// Type of heterogenous list extended with elements of another heterogenous list.
    type Output<T>: HList
    where
        T: HList;

    /// Extends heterogenous list with another heterogenous list.
    ///
    /// Elements of another heterogenous list will be placed at the end
    /// of the current heterogenous list in the order of which they was in another list.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, ops::Extend};
    ///
    /// let first = hlist!(1, 2.0);
    /// let second = hlist!(true, "hello world");
    /// assert_eq!(first.extend(second), hlist!(1, 2.0, true, "hello world"));
    /// assert_eq!(second.extend(first), hlist!(true, "hello world", 1, 2.0));
    /// ```
    fn extend<T>(self, list: T) -> Self::Output<T>
    where
        T: HList;
}

impl Extend for Nil {
    type Output<T> = T
    where
        T: HList;

    fn extend<T>(self, list: T) -> Self::Output<T>
    where
        T: HList,
    {
        list
    }
}

impl<Head, Tail> Extend for Cons<Head, Tail>
where
    Tail: Extend,
{
    type Output<T> = Cons<Head, Tail::Output<T>>
    where
        T: HList;

    fn extend<T>(self, list: T) -> Self::Output<T>
    where
        T: HList,
    {
        let Cons(head, tail) = self;
        let tail = tail.extend(list);
        Cons(head, tail)
    }
}
