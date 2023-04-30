use crate::{Cons, HList, Nil};

/// Convert the heterogenous list into heterogenous list of mutable references.
pub trait ToMut: HList {
    /// Type of new heterogenous list with mutable references of elements.
    type Output<'a>: HList
    where
        Self: 'a;

    /// Converts the heterogenous list into heterogenous list of mutable references.
    ///
    /// Each element of the list will be converted into its mutable reference
    /// in the same order the list has.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, ops::ToMut};
    ///
    /// let mut list = hlist!(1, 2.0, false);
    /// assert_eq!(list.to_mut(), hlist!(&mut 1, &mut 2.0, &mut false));
    /// ```
    fn to_mut(&mut self) -> Self::Output<'_>;
}

impl ToMut for Nil {
    type Output<'a> = Nil
    where
        Self: 'a;

    fn to_mut(&mut self) -> Self::Output<'_> {
        *self
    }
}

impl<Head, Tail> ToMut for Cons<Head, Tail>
where
    Tail: ToMut,
{
    type Output<'a> = Cons<&'a mut Head, Tail::Output<'a>>
    where
        Self: 'a;

    fn to_mut(&mut self) -> Self::Output<'_> {
        let Cons(head, tail) = self;
        let tail = tail.to_mut();
        Cons(head, tail)
    }
}
