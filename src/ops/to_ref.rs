use crate::{Cons, HList, Nil};

/// Convert the heterogenous list into heterogenous list of references.
pub trait ToRef: HList {
    /// Type of new heterogenous list with references of elements.
    type Output<'a>: HList
    where
        Self: 'a;

    /// Converts the heterogenous list into heterogenous list of references.
    ///
    /// Each element of the list will be converted into its reference
    /// in the same order the list has.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, ops::ToRef};
    ///
    /// let list = hlist!(1, 2.0, false);
    /// assert_eq!(list.to_ref(), hlist!(&1, &2.0, &false));
    /// ```
    fn to_ref(&self) -> Self::Output<'_>;
}

impl ToRef for Nil {
    type Output<'a> = Nil
    where
        Self: 'a;

    fn to_ref(&self) -> Self::Output<'_> {
        *self
    }
}

impl<Head, Tail> ToRef for Cons<Head, Tail>
where
    Tail: ToRef,
{
    type Output<'a> = Cons<&'a Head, Tail::Output<'a>>
    where
        Self: 'a;

    fn to_ref(&self) -> Self::Output<'_> {
        let Cons(head, tail) = self;
        Cons(head, tail.to_ref())
    }
}
