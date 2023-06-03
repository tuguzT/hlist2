use crate::{Cons, HList, Nil};

/// Convert the heterogenous list into heterogenous list of references.
pub trait ToRef: HList {
    /// Type of new heterogenous list with references of elements.
    type Ref<'a>: HList
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
    fn to_ref(&self) -> Self::Ref<'_>;

    /// Type of new heterogenous list with mutable references of elements.
    type RefMut<'a>: HList
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
    /// use hlist2::{hlist, ops::ToRef};
    ///
    /// let mut list = hlist!(1, 2.0, false);
    /// assert_eq!(list.to_mut(), hlist!(&mut 1, &mut 2.0, &mut false));
    /// ```
    fn to_mut(&mut self) -> Self::RefMut<'_>;
}

impl ToRef for Nil {
    type Ref<'a> = Nil
    where
        Self: 'a;

    fn to_ref(&self) -> Self::Ref<'_> {
        *self
    }

    type RefMut<'a> = Nil
    where
        Self: 'a;

    fn to_mut(&mut self) -> Self::RefMut<'_> {
        *self
    }
}

impl<Head, Tail> ToRef for Cons<Head, Tail>
where
    Tail: ToRef,
{
    type Ref<'a> = Cons<&'a Head, Tail::Ref<'a>>
    where
        Self: 'a;

    fn to_ref(&self) -> Self::Ref<'_> {
        let Cons(head, tail) = self;
        let tail = tail.to_ref();
        Cons(head, tail)
    }

    type RefMut<'a> = Cons<&'a mut Head, Tail::RefMut<'a>>
    where
        Self: 'a;

    fn to_mut(&mut self) -> Self::RefMut<'_> {
        let Cons(head, tail) = self;
        let tail = tail.to_mut();
        Cons(head, tail)
    }
}
