use crate::{Cons, HList, Nil};

/// Transform one heterogenous list into another.
pub trait Map<Mapper>: HList {
    /// Type of new heterogenous list after transformation.
    type Output: HList;

    /// Transforms the heterogenous list into another heterogenous list
    /// by applying an operation to each element by mapper.
    ///
    /// # Examples
    ///
    /// You can map the list if it is homogenous (all elements have the same type):
    ///
    /// ```
    /// use hlist2::{hlist, ops::Map};
    ///
    /// let list = hlist!(1, 2, 3);
    /// assert_eq!(list.map(|x| 2 * x), hlist!(2, 4, 6));
    /// ```
    ///
    /// Mapping of heterogenous list is possible with heterogenous list of closures as folder:
    ///
    /// ```
    /// use hlist2::{hlist, ops::Map};
    ///
    /// let list = hlist!(1, 2.0, true);
    /// let list = list.map(
    ///     hlist!(
    ///         |i| i + 2,
    ///         |f| f - 2.0,
    ///         |b: bool| !b,
    ///     )
    /// );
    /// assert_eq!(list, hlist!(3, 0.0, false));
    /// ```
    fn map(self, m: Mapper) -> Self::Output;
}

impl<M> Map<M> for Nil {
    type Output = Nil;

    fn map(self, _: M) -> Self::Output {
        self
    }
}

impl<M, R, Head, Tail> Map<M> for Cons<Head, Tail>
where
    M: FnMut(Head) -> R,
    Tail: Map<M>,
{
    type Output = Cons<R, Tail::Output>;

    fn map(self, mut m: M) -> Self::Output {
        let Cons(head, tail) = self;
        Cons(m(head), tail.map(m))
    }
}

impl<MHead, MTail, Head, Tail, R> Map<Cons<MHead, MTail>> for Cons<Head, Tail>
where
    MHead: FnOnce(Head) -> R,
    MTail: HList,
    Tail: Map<MTail>,
{
    type Output = Cons<R, Tail::Output>;

    fn map(self, m: Cons<MHead, MTail>) -> Self::Output {
        let Cons(head, tail) = self;
        let Cons(m_head, m_tail) = m;
        Cons(m_head(head), tail.map(m_tail))
    }
}
