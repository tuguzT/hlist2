use crate::{Cons, HList, Nil};

/// Append new element to the heterogenous list.
pub trait Append: HList {
    /// Type of heterogenous list with new element.
    type Output<T>: HList;

    /// Appends new element to the heterogenous list.
    ///
    /// New element will be placed at the end of the heterogenous list,
    /// resulting in new heterogenous list.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, ops::Append};
    ///
    /// let list = hlist![1, 2.0, true];
    /// assert_eq!(list.append("hello world"), hlist![1, 2.0, true, "hello world"]);
    /// ```
    #[doc(alias("push", "push_back"))]
    fn append<T>(self, value: T) -> Self::Output<T>;
}

impl Append for Nil {
    type Output<T> = Cons<T, Nil>;

    fn append<T>(self, value: T) -> Self::Output<T> {
        Cons(value, self)
    }
}

impl<Head, Tail> Append for Cons<Head, Tail>
where
    Tail: Append,
{
    type Output<T> = Cons<Head, Tail::Output<T>>;

    fn append<T>(self, value: T) -> Self::Output<T> {
        let Cons(head, tail) = self;
        let tail = tail.append(value);
        Cons(head, tail)
    }
}
